use std::fs;

// I have no idea why I decided to make the dial a struct.
struct Dial {
    value: i32,
}

impl Dial {
    fn process_turns(&mut self, instructions: Vec<&str>) -> i32 {
        let mut zeros = 0;
        for instruction in instructions {
            let direction = &instruction[0..1];
            let right = match direction {
                "R" => true,
                "L" => false,
                _ => unreachable!(),
            };
            let rotations: i32 = instruction[1..].parse().expect("Meow mrrp :3");
            
            // Yes, we are manually rotating by 1.
            for _ in 0..rotations {
                if right {
                    if self.value == 99 {
                        self.value = 0;
                    } else {
                        self.value += 1;
                    }
                } else if self.value == 0 {
                    self.value = 99;
                } else {
                    self.value -= 1;
                }
                if self.value == 0 {
                    zeros += 1
                }
            }
        }
        zeros
    }
}

fn main() {
    let mut dial = Dial { value: 50 };
    let binding = fs::read_to_string("./input.txt").expect("Meow");

    let input = binding;
    let instructions: Vec<&str> = Vec::from_iter(
        input
            .split("\n")
            .filter(|s| !s.is_empty())
    );

    let password = dial.process_turns(instructions);

    println!("The password is {}", password);
}
