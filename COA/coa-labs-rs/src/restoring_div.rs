// Restoring division algorithm

use crate::{logic::{fix_bit_length, shift_left, set_lsb_to, extract_msb, invert}, subtraction::binary_subtract, addition::binary_add};

fn restoring_division(num1: &str, num2: &str, n:usize) -> (String, String) {
    #![allow(non_snake_case)]
    // Init values
    let mut B = num2.to_string(); // divisor
    let mut A = "0".repeat(n); // zeros initially
    let mut Q = num1.to_string(); // dividend
    fix_bit_length(&mut B, n);
    fix_bit_length(&mut Q, n);

    for _i in 0..n {
        // shift left AQ
        let bit = shift_left(&mut Q, '0');
        shift_left(&mut A, bit);

        // subtract
        (_, A) = binary_subtract(&A, &B, n, '0');
        let msb = extract_msb(&A);

        // put inverted msb to lsb
        set_lsb_to(&mut Q, invert(msb));

        // if neg (msb == 1), restore (i.e. add back)
        if msb == '1' {
            (_, A) = binary_add(&A, &B, n, '0')
        }
    }
    return (A, Q);
}

#[test]
fn test_restoring_division() {
    #![allow(non_snake_case)]
    let A = "1011";
    let B = "0101";
    let n = 4;
    let (remainder, quotient) = restoring_division(A, B, n);
    println!("Dividing {A} by {B} we get Quotient = {quotient} with Remainder = {remainder}");
    assert_eq!((remainder.as_str(), quotient.as_str()), ("0001", "0010"));
}