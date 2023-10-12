use std::{fs, path::PathBuf};

use common::{movement_cost, rand_layout, DispChar, Layout, RampOptimize, StarRng};

fn main() {
    let population = 1024;
    let sample_len: u64 = 1024;

    let text = fs::read_to_string(PathBuf::from("./text.txt".to_owned())).unwrap();
    let text = text.as_bytes();

    let rng_seed = 23;
    let mut rng = StarRng::new(rng_seed);
    let mut opt = RampOptimize::new(rng_seed + 1, population, |_| rand_layout(&mut rng)).unwrap();

    /*
    opt.freeze_key('b', 1);
    opt.freeze_key('f', 2);
    opt.freeze_key('l', 3);
    opt.freeze_key('w', 4);
    opt.freeze_key('g', 5);
    opt.freeze_key('n', 13);
    opt.freeze_key('t', 14);
    opt.freeze_key('r', 15);
    opt.freeze_key('s', 16);
    opt.freeze_key('c', 17);
    opt.freeze_key('p', 25);
    opt.freeze_key('v', 26);
    opt.freeze_key('k', 27);
    opt.freeze_key('d', 28);
    opt.freeze_key('m', 29);
    */
    opt.freeze_key('.', 6);
    opt.freeze_key(';', 7);
    opt.freeze_key('/', 8);
    opt.freeze_key('u', 9);
    opt.freeze_key('h', 10);
    opt.freeze_key('_', 18);
    opt.freeze_key('i', 19);
    opt.freeze_key('e', 20);
    opt.freeze_key('a', 21);
    opt.freeze_key('o', 22);
    opt.freeze_key('(', 30);
    opt.freeze_key(',', 31);
    opt.freeze_key('y', 32);
    opt.freeze_key(')', 33);
    opt.freeze_key('x', 34);

    // `samples` makes it so the same samples are applied to all
    let cost_fn = |samples: &[usize], layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [u8; 256] = [u8::MAX; 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = i as u8;
        }

        let mut cost = 0;
        for sample_start in samples {
            let mut v = vec![];
            for i in 0..usize::try_from(sample_len).unwrap() {
                let inx = char_to_layout_inx[usize::from(text[i + sample_start])];
                if inx == u8::MAX {
                    v.clear();
                    continue
                }
                v.push(inx);
                if v.len() > 3 {
                    v.remove(0);
                }
                cost += movement_cost(&v);
            }
        }
        cost
    };

    let num_steps = 240;
    for step in 0..num_steps {
        if (step % 20) == 0 {
            dbg!(step);
        }
        let num_samples = if step == (num_steps - 1) {
            // on the last iteration get the best cases
            32
        } else {
            1 + (step / 20)
        };
        let mut sample_starts = vec![];
        for _ in 0..num_samples {
            let len = text.len();
            let sample_inx = usize::try_from(
                rng.next_u64() % u64::try_from(len).unwrap().checked_sub(sample_len).unwrap(),
            )
            .unwrap();
            sample_starts.push(sample_inx);
        }
        opt.step(|layout| cost_fn(&sample_starts, layout));
        /*let mut find_best = vec![];
        for (_, layout) in opt.beam.iter().take(32) {
            let cost = cost_fn(32, layout);
            find_best.push((cost, layout.to_owned()));
        }
        find_best.sort();*/
        //println!("{} {}", step, find_best[0].0);
        if step == (num_steps - 1) {
            // for checking that the distribution is not too chaotic
            for i in 0..1 {
                dbg!(opt.beam[i].0);
                println!("{}", opt.beam[i].1);
            }
        }
    }

    let num_samples = 128;
    let mut sample_starts = vec![];
    for _ in 0..num_samples {
        let len = text.len();
        let sample_inx = usize::try_from(
            rng.next_u64() % u64::try_from(len).unwrap().checked_sub(sample_len).unwrap(),
        )
        .unwrap();
        sample_starts.push(sample_inx);
    }

    let mut find_best = vec![];
    for (_, layout) in opt.beam.iter().take(32) {
        let cost = cost_fn(&sample_starts, layout);
        find_best.push((cost, layout.to_owned()));
    }
    find_best.sort();
    let mut best = find_best[0].1.clone();

    loop {
        let unswapped_cost = cost_fn(&sample_starts, &best);
        let mut best_swaps = vec![];
        for i in 0..36 {
            for j in 0..i {
                let mut trial_swap = best.clone();
                trial_swap.keys.swap(i, j);
                let cost_diff = unswapped_cost.saturating_sub(cost_fn(&sample_starts, &trial_swap));
                if cost_diff > 0 {
                    best_swaps.push((cost_diff, i, j));
                }
            }
        }
        best_swaps.sort();
        best_swaps.reverse();

        if let Some(swap) = best_swaps.first() {
            println!(
                "{} <-> {}, {}",
                best.keys[swap.1], best.keys[swap.2], swap.0
            );
            best.keys.swap(swap.1, swap.2);
            println!("new best: {} \n{}", unswapped_cost - swap.0, best);
        } else {
            break
        }
    }

    for c in best.keys {
        print!("{c}");
    }
    println!();
}

/*
colemak:
T q w f p b   j l u y ; _
E a r s t g   m n e i o N
: x c d v z   k h , . / S

v2:
: . h b y ,   w n N o m k
q a T s i u   E S r p t j
/ f l c B d   g e _ ; v x

v3:
k u . f p /   , N r c d m
q l a e s i   : B S t T y
j b _ g n w   z o ; h v x

// freeze back space and tab

v4:
z w v m f /   k u d o _ y
x a t r s N   B S T e i h
j ; , c l .   g n p b : q

// r s is common, freeze r, s, and newline. Not sure what we
// want to do with tab, freeze it in corner

v5: found this accidentally on a shorter run
w p m l . /   , u o T : k
j t n r s N   B S i e a y
q h f b ; g   v _ x c d z

// have found more than one with the
u o
  i e a
// on the right side, v5 has y on the end which coincidentally
// makes it easier to remember

v5 also features `;` and `_` in potentially ideal places

// 'x' below 'i', 'y' and 'z' close together, 'n' and 'm' close

// j, z, w, q might go in special enabled zone

keep tab in upper corner

map newline or `_` or tab as shift + space?
map delete as shift + backspace

commiting to making `:` from shift + `;`

T / g m p b   . o N y u Z
j n t r s h   B S i a e x
q , v c l k   ; _ f w d z

I have fixed aeiouy in reverse order for easy memorization, it seems to
actually improve perf
tlrs comes up in a row often it seems

on a staggered keyboard the
B S
; _
square works well

v6
T , f N / w   y u o . h j
Z t l r s p   B S i e a k
q x d c n v   ; _ m b g z

even with brute force swap optimization it really likes `t l r s`` so I have frozen it

also q and z like their respective corners so I have frozen them as well

I also move new line to the right column middle row, I don't really have a problem
with that short pinky movement

...

v7
E w b / m v   y u o h . j
T t l r s p   B S i e a N
q , g d n k   ; _ c x f z

where `E` is escape. shift + `/` = `?` which happens to
simultaneously line up with the dual layout and the
correspondance on standard qwerty

After testing out v7 vs other metrics online, it turns out there are significant flaws
with my cost function in regards to bigrams and redirections.

After thinking about it some more, I need to reorient around the time it takes to move
fingers between alternations. e.x. on qwerty, if I have been using my right hand and
then follow up with 'e' and 'f', I will have already adjusted my middle finger to
type the 'e' immediately followed by the 'f', so there is little delay from
changing rows.

From practice, I also found that putting space in the primary set was going to put too
much load on my right hand, defeating the purposes of this program. I have decided upon
putting space and newline on the thumbs of an ortholinear, only delaying switching slightly.

l m c b / d   u z o i ) w
h r s n t v   k e a Z _ g
; Z q , p f   y . Z j ( x

v8: FTL
y Z o , q h   k w c r j d
. i e a n p   f t l _ x s
u ( ) ; / b   v m z Z Z g

v9: room for customization around j,q,(,) for boards without the pinky column, also
 there is a free space for something
j u o / ; w   m k l _ v q
( i e a t d   p n r s c )
  y , . f g   b h x   z

after fixing bug
l u o . q w   v k l m _
( i e a t g   p n r s c )
y z / d f   b h x , j

j v c ) k m   . / u o f Z
z n r t l d   _ i e a s Z
q p b w g h   ( , y ; x Z

j . c o y (   g v m l b z
Z i p e a _   h s r t n q
Z , ; ) / u   f d x k w Z

z b f l w g   ( , u ) x Z
v n t r s c   _ i e a o Z
q p ; k m d   . / j y h Z

V10
j b f l w g   . ; / u h ~
z n t r s c   _ i e a o ~
q p v k d m   ( , y ) x ~

V11
j b l d v w   . ; / u h ~
q n r t s p   _ i e a o ~
z m k g f c   ( , y ) x ~


*/
