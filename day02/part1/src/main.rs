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
        if range[0].to_string().len() % 2 != 0 && (range[1] + 1).to_string().len() % 2 != 0 {
            continue;
        }
        for i in range[0]..range[1] + 1 {
            let candidate = i.to_string();
            let len = candidate.len();

            if candidate[..(len / 2)] == candidate[(len / 2)..] {
                total += i;
            }
        }
    }
    println!("The total is: {total}");
}
