use std::io;

fn main() {
    // immutability
    let val: u32 = 1024;
    println!("val is {}", val);

    // scoped shadowing
    {
        let val = "a string!";
        println!("val is {}", val);
    }

    println!("val is {}", val);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("bad input.");

    let input: String = input.trim().to_string();

    println!("Input is '{}'", input);

    // tuples
    let mut first_tuple: (i32, char, u8) = (256, '😊', 8);
    println!(
        "Tuple contents: {}, {}, {}",
        first_tuple.0, first_tuple.1, first_tuple.2
    );

    let (x, y, z) = first_tuple; // copy
    println!("Tuple contents: {}, {}, {}", x, y, z);

    first_tuple.0 = 1024;

    println!(
        "Tuple contents: {}, {}, {}",
        first_tuple.0, first_tuple.1, first_tuple.2
    );

    //arrays
    let first_array = [1, 2, 3, 4, 5];

    for item in first_array {
        println!("{}", item);
    }

    let first_array: [u8; 6] = [6, 7, 8, 9, 10, 11];

    for i in 0..(first_array.len()) {
        println!("{}", first_array[i]);
    }

    for item in first_array {
        match item % 2 {
            0 => println!("{} is even!", item),
            _ => println!("{} is odd!", item),
        }
    }

    let mut input_index = String::new();
    io::stdin().read_line(& mut input_index).expect("Invalid input.");
    let input_index : usize = input_index.trim().parse().expect("Couldn't parse input.");

    println!("Item {} is {}", input_index, first_array[input_index]);

    // arithmetics w/ varying edge-case behaviour
    let a : u8 = 250;
    assert_eq!(a.saturating_add(20), 255);
    assert_eq!(a.saturating_sub(255), 0);
    assert_eq!(a.wrapping_add(10), 4);
    assert_eq!(a.overflowing_add(10), (4, true));

}
