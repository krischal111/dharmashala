use crate::{addition::binary_add, logic::{fix_bit_length, shift_right, char_repeat}};

// Simple multiplication
fn binary_pp_multiplication(num1:&str, num2:&str, n:usize) -> String {
    #![allow(non_snake_case)]
    let mut M = num1.to_string(); // multiplicand
    let mut Q = num2.to_string(); // multiplier
    let mut A = "0".repeat(n);
    let mut E = '0';
    fix_bit_length(&mut M, n);
    fix_bit_length(&mut Q, n);


    // Display part
    println!("Multiplying {M} with {Q} in {n} bits.");
    println!();
    println!("Initial values: ");
    println!();

    // Markdown compatible tabular printing
    println!("| Registers | Values | Remarks         |");
    println!("|-----------|-------{}-|-----------------|", char_repeat('-', n as i32-6));
    println!("|    M      | {M:>6} | Multiplicand    |");
    println!("|    E      | {E:>6} | Initially bit 0 |");
    println!("|    A      | {A:>6} | Initially zeros |");
    println!("|    Q      | {Q:>6} | Multiplier      |");

    println!();
    println!("Calculation Table");
    println!();
    println!("| Iter | E | A{space}| Q{space}| Remarks      |", space=char_repeat(' ', n as i32));
    println!("|-----:|---|--{minus}|--{minus}|--------------|", minus=char_repeat('-', n as i32));
    println!("|    0 | {E} | {A} | {Q} | Initially    |");

    for _i in 0..n {
        let mut iter = format!("{:>4}", _i+1);
        // Check LSB of !
        let lsb = Q.chars().last().unwrap(); // LSB
        if lsb == '1' {
            // A = A + M
            (E, A) = binary_add(&A, &M, n, '0'); 
            println!("| {iter} | {E} | {A} | {Q} | EA -> EA + M |");
            iter = char_repeat(' ', 4);
        }
        // shift E, A, Q
        let bit = shift_right(&mut A, E);
        shift_right(&mut Q, bit);
        println!("| {iter} | {E} | {A} | {Q} | Shift right  |");
    }

    A.push_str(&Q);
    return A;
}

#[test]
fn test_multiplier_pp() {
    #![allow(non_snake_case)]
    let n = 5;
    let A = "0111";
    let B = "1001";
    let mul = binary_pp_multiplication(A, B, n);
    assert_eq!(mul, "0000111111");
    println!("{A} * {B} = {mul}");
}