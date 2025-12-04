use std::fs;

fn load_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("./input.txt").expect("Meow");
    let tmp = Vec::from_iter(
        input
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_owned()),
    );
    Vec::from_iter(tmp.iter().map(|x| Vec::from_iter(x.chars())))
}

fn main() {
    let input = load_input();

    let mut total = 0u32;
    // The approach we are taking is to queue each toilet roll we see to check if it's valid.
    // queue stores (row, col).
    let mut queue: Vec<(usize, usize)> = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter == '@' {
                queue.push((i, j));
            }
        }
    }
    // Now that we have recorded all our toilet rolls, we can go and find valid placements.
    'top: for (row, col) in queue.iter() {
        // Start at -1 because we will double count itself
        let mut neighbours = -1i8;
        for roll in queue.iter() {
            for a in row.saturating_sub(1)..=row + 1 {
                for b in col.saturating_sub(1)..=col + 1 {
                    if *roll == (a, b) {
                        neighbours += 1;
                        if neighbours >= 4 {
                            continue 'top;
                        }
                    }
                }
            }
        }
        total += 1;
    }
    println!("Total: {total}");
}
