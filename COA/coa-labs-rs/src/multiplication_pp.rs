use crate::{addition::binary_add, logic::{fix_bit_length, shift_right}};

// Simple multiplication
fn binary_pp_multiplication(num1:&str, num2:&str, n:usize) -> String {
    #![allow(non_snake_case)]
    let mut M = num1.to_string(); // multiplicand
    let mut Q = num2.to_string(); // multiplier
    let mut A = "0".repeat(n);
    let mut E = '0';
    fix_bit_length(&mut M, n);
    fix_bit_length(&mut Q, n);

    for _i in 0..n {
        let lsb = Q.chars().last().unwrap(); // LSB
        if lsb == '1' {
            // A = A + M
            (E, A) = binary_add(&A, &M, n, '0');
        }
        // shift E, A, Q
        let bit = shift_right(&mut A, E);
        shift_right(&mut Q, bit);
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