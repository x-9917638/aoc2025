use std::fs;
use std::ops::{Add, AddAssign, Sub, SubAssign};
struct Dial {
    value: i32,
}

impl AddAssign<i32> for Dial {
    fn add_assign(&mut self, rhs: i32) {
        self.value = (self.value + rhs) % 100;
    }
}

impl SubAssign<i32> for Dial {
    fn sub_assign(&mut self, rhs: i32) {
        let to_sub = rhs % 100;
        if self.value - to_sub >= 0 {
            self.value -= to_sub;
        } else {
            self.value = 100 + (self.value - to_sub)
        }
    }
}

impl Dial {
    fn right(&mut self, rotations: i32) {
        *self += rotations;
    }
    fn left(&mut self, rotations: i32) {
        *self -= rotations;
    }
}

fn main() {
    let mut dial = Dial { value: 50 };
    let binding = fs::read_to_string("./input.txt").expect("Meow");

    let mut password = 0;

    let input = binding;
    for instruction in input.split("\n").filter(|s| !s.is_empty()) {
        let direction = &instruction[0..1];
        match direction {
            "R" => {
                dial.right(instruction[1..].parse::<i32>().expect("Meow mrrp :3"));
            }
            "L" => {
                dial.left(instruction[1..].parse::<i32>().expect("Meow mrrp :3"));
            }
            _ => {}
        }
        println!(
            "The dial is rotated {instruction} to point at {}.",
            dial.value
        );
        if dial.value == 0 {
            password += 1;
        }
    }
    println!("The password is {}", password);
}
