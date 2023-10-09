use std::{fs, path::PathBuf};

use common::{
    base_cost, colemak_dh_reference, movement_cost, qwerty_reference, rand_layout, AnnealRamp,
    DispChar, Layout, StarRng,
};

fn main() {
    let population = 1000;
    let sample_len: u64 = 1024;

    let text = fs::read_to_string(PathBuf::from("./primary_layer_text.txt".to_owned())).unwrap();
    let text = text.as_bytes();

    let rng_seed = 0;
    let mut rng = StarRng::new(rng_seed);
    let mut anneal = AnnealRamp::new(rng_seed + 1, population, |_| rand_layout(&mut rng)).unwrap();

    let mut cost_fn = |layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [DispChar; 256] = [DispChar(0); 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = DispChar(i as u8);
        }

        let len = text.len();
        let sample_inx = usize::try_from(
            rng.next_u64() % u64::try_from(len).unwrap().checked_sub(sample_len).unwrap(),
        )
        .unwrap();
        let mut cost = 0;
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
        cost
    };

    for step in 0..100 {
        anneal.step(&mut cost_fn);
        //dbg!(anneal.beam[0].0);
        let mut find_best = vec![];
        for (_, layout) in anneal.beam.iter().take(32) {
            let mut cost = 0;
            for _ in 0..32 {
                cost += cost_fn(layout);
            }
            find_best.push((cost, layout.to_owned()));
        }
        find_best.sort();
        dbg!(find_best[0].0);
    }

    let mut find_best = vec![];
    for (_, layout) in anneal.beam.iter().take(32) {
        let mut cost = 0;
        for _ in 0..32 {
            cost += cost_fn(layout);
        }
        find_best.push((cost, layout.to_owned()));
    }
    find_best.sort();
    dbg!(find_best[0].0);

    println!("annealed:\n{}", find_best[0].1);
    println!("colemak:\n{}", colemak_dh_reference());
}

/*
colemak:
T q w f p b   j l u y ; _
E a r s t g   m n e i o N
: x c d v z   k h , . / S

annealed:
: . h b y ,   w n N o m k
q a T s i u   E S r p t j
/ f l c B d   g e _ ; v x
*/
