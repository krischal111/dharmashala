// To fix bit length of bit list to n bits
pub fn fix_bit_length(num: &mut String, n: usize) {
    let length = num.len();
    if num.len() < n {
        num.insert_str(0, & "0".repeat(n-length));
    }
    if length > n {
        num.replace_range(..(length - n), "");
    }
}

// Bit manipulations
// To invert a single bit
pub fn invert(bit:char) -> char {
    if bit == '0' {
        '1'
    } else {
        '0'
    }
}
// To complement all bits
pub fn complement_bits(x: &mut String) {
    let replace_string : String = x.chars().map(invert).collect();
    x.replace_range(.., &replace_string);
}
// To shift left
pub fn shift_left(x:&mut String, bit:char) -> char {
    // Let us assume, that it will contain characters and unwrap it.
    let outbit = x.chars().next().unwrap();
    x.replace_range(..1, "");
    x.push(bit);
    return outbit;
}

// To shift right
pub fn shift_right(x:&mut String, bit:char) -> char {
    // Let us assume, that it will contain characters and unwrap it.
    let outbit = x.pop().unwrap();
    x.insert(0, bit);
    return  outbit;
}

// To shift right arithmetically
pub fn shift_right_arithmetic(x:&mut String) -> char {
    // assuming that x will contain something, we will unwrap it.
    shift_right(x, x.chars().next().unwrap())
}

// To extract msb and lsb
pub fn extract_msb(x:&str) -> char {
    x.chars().next().unwrap()
}
pub fn extract_lsb(x:&str) -> char {
    x.chars().last().unwrap()
}

// To convert bit to logic and vice versa
pub fn char_to_logic(x:char) -> bool {
    x != '0'
}
pub fn logic_to_char(x:bool) -> char {
    if x {'1'} else {'0'}
}

// To set msb and lsb to specified bit value
pub fn set_msb_to(x:&mut String, bit: char) {
    shift_left(x, '0');
    shift_right(x, bit);
}
pub fn set_lsb_to(x:&mut String, bit: char) {
    shift_right(x, '0');
    shift_left(x, bit);
}

use std::io::{self, Write};
pub fn input_anything() -> io::Result<String> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    user_input.pop();
    Ok(user_input)
}

pub fn input_with_prompt(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    input_anything()
}

pub fn input_two_binary_number_and_length() -> (String, String, usize) {
    let n = loop {
        let n = input_with_prompt("Please input number of bits = ")
            .unwrap_or("4".to_string());
        match n.parse() {
            Ok(n ) if n > 0 => break n,
            _ => {
                println!("You have to enter +ve number of bits");
                continue;
            },
        }
    };

    let mut a = input_with_prompt("Please enter first  binary number = ").unwrap();
    let mut b = input_with_prompt("Please enter second binary number = ").unwrap();
    fix_bit_length(&mut a, n);
    fix_bit_length(&mut b, n);
    return (a,b,n);
}

pub fn input_two_binary_number_of_n_length(n: usize) -> (String, String) {
    let mut a = input_with_prompt("Please enter first  binary number = ").unwrap();
    let mut b = input_with_prompt("Please enter second binary number = ").unwrap();
    fix_bit_length(&mut a, n);
    fix_bit_length(&mut b, n);
    return (a,b);
}

pub fn input_two_binary_number_of_length_4() -> (String, String) {
    input_two_binary_number_of_n_length(4)
}

pub fn check_input() {
    #![allow(unused)]
    let (a,b,n) = input_two_binary_number_and_length();
    eprintln!("Bit count = {n}");
    eprintln!("First  Number = {a}");
    eprintln!("Second Number = {b}");

    println!("\nYou are entering binary number of length 5");
    let (a,b) = input_two_binary_number_of_n_length(5);
    eprintln!("Bit count = 5");
    eprintln!("First  Number = {a}");
    eprintln!("Second Number = {b}");

    eprintln!("\nYou are entering binary number of length 4");
    let (a,b) = input_two_binary_number_of_length_4();
    eprintln!("Bit count = 4");
    eprintln!("First  Number = {a}");
    eprintln!("Second Number = {b}");
}

#[test]
pub fn check_logic() {
    println!("Hello, world!");
    let mut num = "00100011".to_string();
    println!("{num} : Original");
    
    fix_bit_length(&mut num, 10);
    println!("{num} : Truncated to length 10 (greater)");
    assert_eq!(num, "0000100011");
    
    fix_bit_length(&mut num, 6);
    println!("{num} : Truncated to length 6 (less)");
    assert_eq!(num, "100011");
    
    complement_bits(&mut num);
    println!("{num} : All bits complemented");
    assert_eq!(num, "011100");
    
    shift_left(&mut num, '0');
    println!("{num} : Shifted left, inserting 0 at the end");
    assert_eq!(num, "111000");
    
    shift_left(&mut num, '1');
    println!("{num} : Shifted left, inserting 1 at the end");
    assert_eq!(num, "110001");
    
    shift_right(&mut num, '1');
    println!("{num} : Shifted right, inserting 1 at the beginning");
    assert_eq!(num, "111000");
    
    shift_right_arithmetic(&mut num);
    println!("{num} : Shifted right arithmetically");
    assert_eq!(num, "111100");
    
    shift_right(&mut num, '0');
    println!("{num} : Shifted right, inserting 0 at the beginning");
    assert_eq!(num, "011110");
    
    shift_right_arithmetic(&mut num);
    println!("{num} : Shifted right, again arithmetically");
    assert_eq!(num, "001111");
    
    let msb = extract_msb(&num);
    println!("MSB is {msb}");
    assert_eq!(msb, '0');
    
    let lsb = extract_lsb(&num);
    println!("LSB is {lsb}");
    assert_eq!(lsb, '1');
    
    let logica = char_to_logic(msb);
    let logicb = char_to_logic(lsb);
    let bita = logic_to_char(logica);
    let bitb = logic_to_char(logicb);
    println!("{logica} is {bita}");
    println!("{logicb} is {bitb}");
    println!("{msb} is {logica}");
    println!("{lsb} is {logicb}");
    assert_eq!(logica, false);
    assert_eq!(logicb, true);
    assert_eq!(bita, '0');
    assert_eq!(bitb, '1');
    
    println!("{num} : Current value");
    set_msb_to(&mut num, '1');
    println!("{num} : Setting MSB to 1");
    assert_eq!(num, "101111");
    
    set_lsb_to(&mut num, '0');
    println!("{num} : Setting LSB to 0");
    assert_eq!(num, "101110");

    println!("{num}");
}

// String manipulations:
pub fn char_repeat(ch: char, count: i32) -> String {
    let count = if count > 0 { count } else { 0 };
    ch.to_string().repeat(count as usize)
}