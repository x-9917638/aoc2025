use std::{collections::VecDeque, fs};

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
    // Hiiii
    // My day4 solutions are pretty trash...
    // This is defenitely NOT the best way to approach this.

    let input = load_input();

    let mut total = 0u32;
    // The approach we are taking is to queue each toilet roll we see to check if it's valid.
    // rolls stores (row, col).
    let mut rolls: VecDeque<(usize, usize)> = VecDeque::new();
    for (i, row) in input.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter == '@' {
                rolls.push_back((i, j));
            }
        }
    }

    // Now that we have recorded all our toilet rolls, we can go and find valid placements.
    let mut possible = true; // Possibliility of new removals
    while possible {
        // We need to queue things to remove, so that they preserve the correct index.
        let mut removal_queue: Vec<usize> = Vec::new();
        possible = false;
        'top: for (pos, &(row, col)) in rolls.iter().enumerate() {
            let mut neighbours = 0i8;
            for roll in rolls.iter() {
                for r in row.saturating_sub(1)..=row + 1 {
                    for c in col.saturating_sub(1)..=col + 1 {
                        // Skip self
                        if r == row && c == col {
                            continue;
                        }
                        if *roll == (r, c) {
                            neighbours += 1;

                            if neighbours >= 4 {
                                continue 'top;
                            }
                        }
                    }
                }
            }
            removal_queue.push(pos);
            possible = true;
        }
        // By removing from the end, we target the correct indexes.
        removal_queue.reverse();
        for pos in removal_queue {
            rolls.remove(pos);
            total += 1;
        }
    }

    println!("Total: {total}");
}
