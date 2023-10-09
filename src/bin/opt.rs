use std::{fs, path::PathBuf};

use common::{colemak_dh_reference, rand_layout, DispChar, Layout, RampOptimize, StarRng};

fn main() {
    let population = 1024;
    let sample_len: u64 = 1024;

    let text = fs::read_to_string(PathBuf::from("./primary_layer_text.txt".to_owned())).unwrap();
    let text = text.as_bytes();

    let rng_seed = 10;
    let mut rng = StarRng::new(rng_seed);
    let mut opt = RampOptimize::new(rng_seed + 1, population, |_| rand_layout(&mut rng)).unwrap();

    /*opt.freeze_key('\t', 0);
    opt.freeze_key('\u{8}', 18);
    opt.freeze_key(' ', 19);
    opt.freeze_key('_', 31);
    opt.freeze_key('r', 15);
    opt.freeze_key('s', 16);
    opt.freeze_key(';', 30);*/

    //opt.freeze_key('\u{8}', 18);
    //opt.freeze_key(' ', 19);
    //opt.freeze_key('\n', 17);

    // `samples` makes it so the same samples are applied to all
    let cost_fn = |samples: &[usize], layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [DispChar; 256] = [DispChar(0); 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = DispChar(i as u8);
        }

        let mut cost = 0;
        for sample_start in samples {
            for i in 0..usize::try_from(sample_len).unwrap() {
                let j = i + sample_start;
                let c0 = char_to_layout_inx[usize::from(text[j])];
                let c1 = char_to_layout_inx[usize::from(text[j - 1])];
                if i > 0 {
                    cost += layout.bigram_cost(c1, c0);
                } else {
                    cost += layout.unigram_cost(c0);
                }
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
        find_best.sort();
        println!("{} {}", step, find_best[0].0);*/
        if step == (num_steps - 1) {
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
    let best = find_best[0].1.clone();

    /*for i in 0..36 {
        dbg!(i);
        for j in 0..36 {
            let mut trial_swap = best.clone();
            trial_swap.keys.swap(i, j);
            if (cost_fn(32, &trial_swap) + 100000) < cost_fn(32, &best) {
                best = trial_swap;
            }
        }
    }*/

    println!("opted:\n{}", best);
    println!("colemak:\n{}", colemak_dh_reference());
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

*/
