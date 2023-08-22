
use crate::{addition::binary_add, logic::{fix_bit_length, shift_right, shift_right_arithmetic}, subtraction::{binary_subtract, twos_complement}};

// Simple multiplication
fn binary_booth_multiplication(num1:&str, num2:&str, n:usize) -> String {
    #![allow(non_snake_case)]
    let mut M = num1.to_string(); // multiplicand
    let mut _negm = twos_complement(M.clone());
    let mut Q = num2.to_string(); // multiplier
    let mut A = "0".repeat(n);
    let mut Qb = '0';
    fix_bit_length(&mut M, n);
    fix_bit_length(&mut Q, n);

    for i in 0..n {
        let lsb = Q.chars().last().unwrap();
        if lsb == '0' && Qb == '1' {
            // A = A + M
            (_, A) = binary_add(&A, &M, n, '0');
        } else if lsb == '1' && Qb == '0' {
            // A = A - m 
            (_, A) = binary_subtract(&A, &M, n, '0');
        }
        // shift E, A, Q
        let bit = shift_right_arithmetic(&mut A);
        Qb = shift_right(&mut Q, bit);
    }

    A.push_str(&Q);
    return A;
}

#[test]
fn test_multiplier_pp() {
    #![allow(non_snake_case)]
    let n = 5;
    let A = "00111";
    let B = "01001";
    let mul = binary_booth_multiplication(A, B, n);
    assert_eq!(mul, "0000111111");
    println!("{A} * {B} = {mul}");
}