use std::{env, process::exit};

const DECRYPTION_KEY: i64 = 811589153;

mod examples {
    use std::num::ParseIntError;

    pub fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
        input
            .split("\n")
            .map(|l| l.parse::<i64>())
            .into_iter()
            .collect()
    }

    pub fn example_input(use_example_2: bool) -> &'static str {
        if use_example_2 {
            return include_str!("d20_ex_2.txt");
        } else {
            return include_str!("d20_ex_1.txt");
        }
    }
}

mod crypto {
    /// for each element x: count x steps forward (looping around if needed), remove x, insert x after the new position
    pub fn mix(seq: Vec<i64>, rounds: u32) -> Vec<i64> {
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

                if dest == src {
                    continue;
                }

                if dest > src {
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
                } else if dest < src {
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
            }
        }

        res
    }

    pub fn decrypt(seq: Vec<i64>, decryption_key: i64, rounds: u32) -> i64 {
        let mixed = mix(seq.iter().map(|n| n * decryption_key).collect(), rounds);

        let zero_pos = mixed.iter().position(|&n| n == 0).unwrap();

        let res1 = mixed[(zero_pos + 1000) % mixed.len()];
        let res2 = mixed[(zero_pos + 2000) % mixed.len()];
        let res3 = mixed[(zero_pos + 3000) % mixed.len()];
        let res = res1 + res2 + res3;

        res
    }

    #[cfg(test)]
    mod tests {
        use crate::{
            crypto::*,
            examples::{example_input, parse_input},
        };

        #[test]
        fn decrypting_small_example_works_part_1() {
            let seq = parse_input(example_input(false)).unwrap();

            assert_eq!(decrypt(seq, 1, 1), 3);
        }

        #[test]
        fn decrypting_big_example_works_part_1() {
            let seq = parse_input(example_input(true)).unwrap();

            assert_eq!(decrypt(seq, 1, 1), 11037);
        }

        #[test]
        fn decrypting_small_example_works_part_2() {
            let seq = parse_input(example_input(false)).unwrap();

            assert_eq!(decrypt(seq, crate::DECRYPTION_KEY, 10), 1623178306);
        }

        #[test]
        fn decrypting_big_example_works_part_2() {
            let seq = parse_input(example_input(true)).unwrap();

            assert_eq!(decrypt(seq, crate::DECRYPTION_KEY, 10), 3033720253914);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: d20 <s|b> <1|2>");
        exit(1);
    } else {
        let use_example_2 = args[1] == "b";
        let is_part_2 = args[2] == "2";

        let input = examples::example_input(use_example_2);

        let seq = examples::parse_input(input).unwrap();

        println!("Read {} numbers", seq.len());

        let res = if is_part_2 {
            crypto::decrypt(seq, DECRYPTION_KEY, 10)
        } else {
            crypto::decrypt(seq, 1, 1)
        };

        println!("Result: {}", res);
    }
}
