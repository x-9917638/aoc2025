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
    // Note that part2 extends part 1 more concisely (rushing for the star here).
    // I have also included much better explanations in part2.
    // In other words, don't look here, go to part2.

    let contents = load_data();

    let mut total = 0u32;
    for string in contents {
        let mut string = string;
        let mut container = String::new();
        let mut max = 0u8;
        let mut index = 0;
        for i in 0..string.len() - 1 {
            if max < string[i..i + 1].parse::<u8>().expect("") {
                max = string[i..i + 1].parse::<u8>().expect("");
                index = i;
            }
        }
        container.push(string.remove(index));
        println!("{}", max);
        let mut max = 0u8;
        #[allow(clippy::mut_range_bound)]
        for i in index..string.len() {
            if max < string[i..i + 1].parse::<u8>().expect("") {
                max = string[i..i + 1].parse::<u8>().expect("");
                index = i;
            }
        }
        container.push(string.remove(index));
        println!("{}", max);
        println!("Largest output: {}", container);
        total += container.parse::<u32>().expect("");
    }
    println!("Total: {total}");
}
