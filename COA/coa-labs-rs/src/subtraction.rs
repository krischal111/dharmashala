use crate::{
    logic::{
        complement_bits, 
        fix_bit_length, invert
    }, 
    addition::{
        binary_add, 
        display_add_sub
    }
};

// To generate 2's complement (the negative)
fn twos_complement(mut bits: String) -> String {
    complement_bits(&mut bits);
    let (_, bits) = binary_add(&bits, "1", bits.len());
    return bits;
}

// To perform binary subtraction 
fn binary_subtract(num1: &str, num2: &str, n:usize) -> (char, String) {
    let mut num1 = num1.to_string();
    let mut num2 = num2.to_string();
    fix_bit_length(&mut num1, n);
    fix_bit_length(&mut num2, n);
    let num2 = twos_complement(num2);
    let (cy, diff) = binary_add(&num1, &num2, n);
    return (invert(cy), diff);
}





#[test]
fn test_binary_subtractor() {
    let num1 = "001101".to_string();
    let num2 = "001100".to_string();
    let n = 6;
    let (c, diff) = binary_subtract(&num1, &num2, n);
    display_add_sub(num1, num2, c, diff.clone(), n, '-');
    assert_eq!(('0', "000001"), (c, diff.as_str()));
    return;
}

#[test]
fn test_twos_complementer() {
    let bits = "000011".to_string();
    let neg_bits = twos_complement(bits.clone());
    println!("{bits}\n{neg_bits}");
    assert_eq!(neg_bits, "111101");
}