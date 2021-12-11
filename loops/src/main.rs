use std::io;

const COUNT: usize = 5;

fn calculate_sum(array: [i32; COUNT]) -> (i32, bool) {
    let mut sum: i32 = 0;
    let mut result_overflown = false;

    for number in array {
        let (result, overflown) = sum.overflowing_add(number);
        sum = result;
        result_overflown |= overflown;
    }

    return (sum, result_overflown);
}

fn get_number() -> i32 {
    loop {
        let mut input_number = String::new();
        match io::stdin().read_line(&mut input_number) {
            Ok(_) => (),
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };

        match input_number.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
        };
    }
}

fn main() {
    let mut array: [i32; COUNT] = [0; COUNT];

    for idx in 0..array.len() {
        println!("Enter number #{}.", idx);
        array[idx] = get_number();
    }

    match calculate_sum(array) {
        (sum, false) => println!("The sum of these is {}.", sum),
        (_, true) => println!("Could not calculate sum, overflow detected. Sorry."),
    }
}
