use std::{fs, path::PathBuf};

use common::{map, movement_cost, DispChar, Layout};

fn main() {
    let text = fs::read_to_string(PathBuf::from("./text.txt".to_owned())).unwrap();
    let text = text.as_bytes();

    // `samples` makes it so the same samples are applied to all
    let cost_fn = |sample: &[u8], layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [u8; 256] = [u8::MAX; 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = i as u8;
        }

        let mut cost = 0;
        let mut v = vec![];
        for i in 0..sample.len() {
            let inx = char_to_layout_inx[usize::from(sample[i])];
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
        cost
    };

    let mut best = map!("jbldvw.)uoy~qnrtsf(ieah~zmkgcx_p,;/~");
    println!("{best}");

    let mut frozen = Layout::<bool>::new(|_| false);
    frozen.keys[19] = true;

    loop {
        let unswapped_cost = cost_fn(&text, &best);
        let unswapped_cost_trial = cost_fn(&text[..1000000], &best);
        let mut best_swaps = vec![];
        for i in 0..36 {
            dbg!(i);
            for j in 0..i {
                if frozen.keys[i] || frozen.keys[j] {
                    continue
                }
                let mut trial_swap = best.clone();
                trial_swap.keys.swap(i, j);
                // trial swap on a million to reject if the difference was only slightly
                // negative
                let cost = cost_fn(&text[..1000000], &trial_swap);
                let mut reject = false;
                if cost > unswapped_cost_trial {
                    if (unswapped_cost_trial / (cost - unswapped_cost_trial)) <= 500 {
                        reject = true;
                    }
                }
                if !reject {
                    let cost = cost_fn(&text, &trial_swap);
                    let cost_diff = unswapped_cost.saturating_sub(cost);
                    if cost_diff > 0 {
                        best_swaps.push((cost_diff, i, j));
                    }
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
            println!("new best:\n{}", best);
        } else {
            break
        }
        for c in best.keys {
            print!("{c}");
        }
        println!();
    }
}
