use std::fs;

fn load_data() -> Vec<String> {
    let contents = fs::read_to_string("./input.txt").expect("Meow");
    Vec::from_iter(
        contents
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned()),
    )
}

fn main() {
    let contents = load_data();
    let mut total = 0u128;

    for mut bank in contents {
        let mut container = String::new();

        // Indicate the start index at which to start looking for the highest digit.
        let mut index = 0;
        for last in (0..12).rev() {
            let mut max = 0u8;

            // What we want to do here is look through the valid range
            // for the highest digit. The start is the index at which
            // the last biggest digit was found at. The end is the last
            // available digit that can be chosen such that we have enough
            // space to fufill all 12 digits of the final joltage.

            // We do not need to mutate index while in the loop.
            // We mutate index so that the next loop starts from
            // the correct index.
            #[allow(clippy::mut_range_bound)]
            for i in index..bank.len() - last {
                let candidate = bank[i..i + 1].parse::<u8>().expect("");
                if max < candidate {
                    max = candidate;
                    index = i;
                }
            }
            // Here, we remove the chosen digit from the string and add it to our growing joltage
            container.push(bank.remove(index));
        }
        total += container.parse::<u128>().expect("");
    }
    println!("Total joltage: {total}");
}
