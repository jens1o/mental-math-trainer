extern crate rand;

use rand::Rng;
use std::fmt;
use std::io::{self, Write};
use std::ops;

#[derive(Copy, Clone, PartialEq)]
enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl fmt::Display for MathOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MathOperation::*;

        write!(
            f,
            "{}",
            match self {
                Addition => '+',
                Subtraction => '-',
                Multiplication => '*',
                Division => '/',
            }
        )
    }
}

struct MathOperationDistribution {}

impl rand::distributions::Distribution<MathOperation> for MathOperationDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MathOperation {
        let rand_int: u8 = rng.gen_range(0..4);

        match rand_int {
            0 => MathOperation::Addition,
            1 => MathOperation::Subtraction,
            2 => MathOperation::Multiplication,
            3 => MathOperation::Division,
            _ => unreachable!(rand_int),
        }
    }
}

#[derive(Copy, Clone)]
struct MathNumber {
    pub value: i8,
}

impl fmt::Display for MathNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.value.is_negative() {
                format!("({})", self.value)
            } else {
                format!("{}", self.value)
            }
        )
    }
}

#[derive(Clone, Debug)]
struct MathNumberDistribution {
    range: ops::Range<i8>,
}

impl Default for MathNumberDistribution {
    fn default() -> Self {
        MathNumberDistribution::easy()
    }
}

impl MathNumberDistribution {
    #[allow(unused)]
    fn easy() -> MathNumberDistribution {
        MathNumberDistribution { range: (-10..20) }
    }

    #[allow(unused)]
    fn normal() -> MathNumberDistribution {
        MathNumberDistribution { range: (-30..30) }
    }

    #[allow(unused)]
    fn hard() -> MathNumberDistribution {
        MathNumberDistribution {
            range: (i8::MIN..i8::MAX),
        }
    }
}

impl rand::distributions::Distribution<MathNumber> for MathNumberDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MathNumber {
        let value: i8 = rng.gen_range(self.range.clone());

        MathNumber { value }
    }
}

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input_buffer = String::with_capacity(4);

    println!("Welcome to your mental math trainer. Solve the exercises to improve your ability to do calculations mentally.");

    'outer: loop {
        let operation: MathOperation = rng.sample(MathOperationDistribution {});

        let first_number: MathNumber = rng.sample(MathNumberDistribution::default());

        let mut second_number: MathNumber;

        loop {
            second_number = rng.sample(MathNumberDistribution::default());

            // avoid division by zero
            if operation == MathOperation::Division && second_number.value == 0 {
                continue;
            } else {
                break;
            }
        }

        let result = solve_math_operation(first_number, operation, second_number);

        loop {
            input_buffer.clear();
            println!("\n{} {} {}", first_number, operation, second_number);

            print!("> ");
            stdout.flush()?;

            stdin.read_line(&mut input_buffer)?;
            println!();

            let input = input_buffer.trim();

            if input == "exit" {
                break 'outer;
            }

            let user_result = match input_buffer.trim().parse::<i16>() {
                Err(_) => {
                    println!("Invalid input, please enter a number");
                    continue;
                }
                Ok(user_result) => user_result,
            };

            if user_result == result {
                println!("✅ correct answer!");
                continue 'outer;
            } else {
                println!("❌ that's wrong :( try again?");
            }
        }
    }

    println!("Goodbye. Hope you had fun!");

    Ok(())
}

fn solve_math_operation(
    first_number: MathNumber,
    operation: MathOperation,
    second_number: MathNumber,
) -> i16 {
    let first = first_number.value as i16;
    let second = second_number.value as i16;

    match operation {
        MathOperation::Addition => first + second,
        MathOperation::Subtraction => first - second,
        MathOperation::Multiplication => first * second,
        MathOperation::Division => first / second,
    }
}
