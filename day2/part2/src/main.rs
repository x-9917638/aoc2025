use std::fs;
// use std::{thread::sleep, time::{self, Duration}}

fn load_data() -> Vec<String> {
    let contents = fs::read_to_string("./input.txt").expect("Meow");
    Vec::from_iter(
        contents
            .split(",")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned()),
    )
}

fn main() {
    let contents = load_data();

    let mut total = 0u128;
    for range in contents {
        let range = Vec::from_iter(
            range
                .split("-")
                .map(|x| x.parse::<u128>().expect("Bleh :3")),
        );

        for num in range[0]..range[1] + 1 {
            let candidate = num.to_string();
            let len = candidate.len();

            'substr: for j in 1..=(len / 2) {
                let substring = &candidate[..j];
                let sub_len = substring.len();

                if len % sub_len != 0 {
                    continue;
                }

                let first = &candidate[0..sub_len];
                for a in (sub_len * 2..=len).step_by(sub_len) {
                    if &candidate[a - sub_len..a] != first {
                        continue 'substr;
                    }
                }
                total += num;
                break 'substr;
            }
        }
    }
    println!("The total is: {total}");
}
