fn modify_reference(str: &mut String) {
    str.push_str("CCCCC");
}

fn modify_borrowed(mut str: String) -> String {
    str.push_str("BBBBB");
    return str;
}

fn main() {
    let mut mut_str: String = String::new();
    mut_str.push_str("AAAAA");
    mut_str = modify_borrowed(mut_str);
    modify_reference(&mut mut_str);

    println!("{}", &mut_str);

    let mut mut_str2 = mut_str;
    modify_reference(&mut mut_str2);

    let mut mut_str3 = & mut mut_str2;

    modify_reference(& mut mut_str3);
    // modify_reference(& mut mut_str2); <-- illegal since one mutable reference already exists
    // println!("{}", &mut_str2);        <-- also illegal because we're already borrowing as immutable

    modify_reference(& mut mut_str3);

    println!("{}", &mut_str2);

    let imm_str = mut_str2;

    println!("{}", &imm_str);

    let array = [0, 1, 2, 3, 4, 5];
    for item in array {        
        print!("{} ", item);
    }

    print!("\n");
    println!("Slice:");

    for item in &array[0..4] {
        print!("{} ", item);
    }

    let imm_str2 = String::from("test!");
    println!("{} {}", imm_str2, &imm_str2[0..4]);

    let imm_str_slice : &str = "&str!";
    println!("{}", imm_str_slice);    

}
