use std::fs;

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

        // We loop through every number in their range
        for num in range[0]..range[1] + 1 {
            let candidate = num.to_string();
            let len = candidate.len();

            // We look for each length of substring. The max length of a substring is half the candidates length.
            'substr: for j in 1..=(len / 2) {
                let substring = &candidate[..j];
                let sub_len = substring.len();

                // Because the whole string has to repeat,
                // if the length of the string is not divisible by the length of the
                // substring, we can skip it. (There's probably a more efficient way to do this!)
                if len % sub_len != 0 {
                    continue;
                }

                let first = &candidate[0..sub_len];
                for a in (sub_len * 2..=len).step_by(sub_len) {
                    // If the next substring is not the same as the first substring, we can skip this length.
                    if &candidate[a - sub_len..a] != first {
                        continue 'substr;
                    }
                }
                total += num;
                // Once we have found 1 valid pattern, we can go to the next candidate.
                break 'substr;
            }
        }
    }
    println!("The total is: {total}");
}
