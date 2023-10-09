use std::{fs, path::PathBuf};

use common::{
    base_cost, char_to_byte, colemak_dh_reference, movement_cost, rand_layout, DispChar, Layout,
    RampOptimize, StarRng,
};

fn main() {
    let population = 1024;
    let sample_len: u64 = 1024;

    let text = fs::read_to_string(PathBuf::from("./primary_layer_text.txt".to_owned())).unwrap();
    let text = text.as_bytes();

    let rng_seed = 0;
    let mut rng = StarRng::new(rng_seed);
    let mut opt = RampOptimize::new(rng_seed + 1, population, |_| rand_layout(&mut rng)).unwrap();

    // freeze backspace and space at keys 18 and 19
    opt.frozen.keys[18] = true;
    opt.frozen.keys[19] = true;
    for layout in &mut opt.beam {
        let layout = &mut layout.1;
        for i in 0..layout.keys.len() {
            if layout.keys[i].0 == char_to_byte('\u{8}').unwrap() {
                layout.keys.swap(i, 18);
            }
            if layout.keys[i].0 == char_to_byte(' ').unwrap() {
                layout.keys.swap(i, 19);
            }
        }
    }

    let mut cost_fn = |num_samples: usize, layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [DispChar; 256] = [DispChar(0); 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = DispChar(i as u8);
        }

        let mut cost = 0;
        for _ in 0..num_samples {
            let len = text.len();
            let sample_inx = usize::try_from(
                rng.next_u64() % u64::try_from(len).unwrap().checked_sub(sample_len).unwrap(),
            )
            .unwrap();
            for i in 0..usize::try_from(sample_len).unwrap() {
                let j = i + sample_inx;
                let c0 = text[j];
                let inx0 = char_to_layout_inx[usize::from(c0)];
                cost += base_cost(inx0.0);
                if i > 0 {
                    let c1 = text[j - 1];
                    let inx1 = char_to_layout_inx[usize::from(c1)];
                    cost += movement_cost(&[inx0.0, inx1.0]);
                }
            }
        }
        cost
    };

    for step in 0..240 {
        let num_samples = 1 + (step / 80);
        opt.step(|layout| cost_fn(num_samples, layout));
        //dbg!(opt.beam[0].0);
        let mut find_best = vec![];
        for (_, layout) in opt.beam.iter().take(32) {
            let cost = cost_fn(32, layout);
            find_best.push((cost, layout.to_owned()));
        }
        find_best.sort();
        println!("{} {}", step, find_best[0].0);
        /*if step % 20 == 0 {
            for i in 0..32 {
                println!("{}\n{}", find_best[i].0, find_best[i].1);
            }
        }*/
    }

    let mut find_best = vec![];
    for (_, layout) in opt.beam.iter().take(32) {
        let cost = cost_fn(32, layout);
        find_best.push((cost, layout.to_owned()));
    }
    find_best.sort();
    dbg!(find_best[0].0);

    println!("opted:\n{}", find_best[0].1);
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

v4:
z w v m f /   k u d o _ y
x a t r s N   B S T e i h
j ; , c l .   g n p b : q
*/
