use std::cmp::Ordering;

pub fn decrypt(seq: Vec<i64>, decryption_key: i64, rounds: u32) -> i64 {
    let mixed = mix(seq.iter().map(|n| n * decryption_key).collect(), rounds);

    let zero_pos = mixed.iter().position(|&n| n == 0).unwrap();

    let res1 = mixed[(zero_pos + 1000) % mixed.len()];
    let res2 = mixed[(zero_pos + 2000) % mixed.len()];
    let res3 = mixed[(zero_pos + 3000) % mixed.len()];

    res1 + res2 + res3
}

/// for each element x: count x steps forward (looping around if needed), remove x, insert x after the new position
fn mix(seq: Vec<i64>, rounds: u32) -> Vec<i64> {
    let seq_len = seq.len() as i64;

    let mut res: Vec<i64> = seq.clone();

    // idx[i] is the index of seq[i] in res
    let mut res_idx: Vec<usize> = (0..seq.len()).collect();

    // inv_idx[i] is the index of res[i] in seq
    let mut seq_idx: Vec<usize> = (0..seq.len()).collect();

    for _ in 0..rounds {
        for n in 0..seq.len() {
            let src = res_idx[n];
            let here = res[src];

            let dest = src as i64 + here % (seq_len - 1);
            let dest = (dest + (seq_len - 1)) % (seq_len - 1);
            let dest = dest as usize;

            match dest.cmp(&src) {
                Ordering::Equal => continue,
                Ordering::Greater => {
                    for i in (src + 1)..=dest {
                        res_idx[seq_idx[i]] -= 1;
                    }
                    res_idx[seq_idx[src]] = dest;

                    for i in (src + 1)..=dest {
                        res[i - 1] = res[i];
                        seq_idx[i - 1] = seq_idx[i];
                    }
                    res[dest] = here;
                    seq_idx[dest] = n;
                }
                Ordering::Less => {
                    for i in dest..src {
                        res_idx[seq_idx[i]] += 1;
                    }
                    res_idx[seq_idx[src]] = dest;

                    for i in (dest..src).rev() {
                        res[i + 1] = res[i];
                        seq_idx[i + 1] = seq_idx[i];
                    }
                    res[dest] = here;
                    seq_idx[dest] = n;
                }
            };
        }
    }

    res
}
