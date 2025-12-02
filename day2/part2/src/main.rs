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

        for i in range[0]..range[1] + 1 {
            let candidate = i.to_string();
            let len = candidate.len();

            for j in 1..=(len / 2) {
                let substring = &candidate[..j];
                let sub_len = substring.len();

                if len % sub_len != 0 {
                    continue;
                }

                let mut substrings = Vec::new();
                for a in (sub_len..=len).step_by(sub_len) {
                    substrings.push(&candidate[a.saturating_sub(sub_len)..a]);
                }

                // eprintln!("DEBUG - substrings={substrings:?}");
                // sleep(Duration::from_millis(200));

                if substrings.iter().all(|&x| x == substrings[0]) {
                    total += i;
                    break;
                }
            }
        }
    }
    println!("The total is: {total}");
}
