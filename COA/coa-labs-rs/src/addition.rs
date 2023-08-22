use crate::logic::{self, char_to_logic, logic_to_char, fix_bit_length};

// half adder to add two bits
fn half_adder(a:char, b:char) -> (char, char) {
    let a = logic::char_to_logic(a);
    let b = logic::char_to_logic(b);
    let cy = a && b;
    let sm = a ^ b;
    let carry = logic::logic_to_char(cy);
    let sum = logic::logic_to_char(sm);
    return (carry, sum);
}

// to add three bits
fn full_adder(a:char, b:char, c:char) -> (char, char) {
    let (hc1, half_sum) = half_adder(a, b);
    let (hc2, sum) = half_adder(half_sum, c);
    let carry = if [hc1, hc2].contains(&'1') {'1'} else {'0'};
    return (carry, sum);
}

// another implementation, does the same job
fn full_adder_2(a:char, b:char, c:char) -> (char, char) {
    let (a, b, c) = (
        char_to_logic(a),
        char_to_logic(b),
        char_to_logic(c),
    );
    let carry = (a && b) || (b && c) || (c && a);
    let sum = a ^ b ^ c;
    return (logic_to_char(carry), logic_to_char(sum));
}

// binary addition: two bits of n length
pub fn binary_add(num1:&str, num2:&str, n:usize, mut c:char) -> (char, String) {
    let mut num1 = num1.to_string();
    let mut num2 = num2.to_string();
    let num1 = &mut num1;
    let num2 = &mut num2;
    fix_bit_length(num1, n);
    fix_bit_length(num2, n);

    let mut sum = "".to_string();
    for (a, b) in std::iter::zip(num1.chars().rev(), num2.chars().rev()) {
        let sm;
        (c, sm) = full_adder(a, b, c);
        sum.insert(0, sm);
    }
    return (c, sum);
}

// To display add or subtraction in pretty format
pub fn display_add_sub(num1: String, num2: String, c:char, sum:String, n:usize, sign:char) {
    println!("  {num1}");
    println!("{sign} {num2}");
    println!("{}", "-".repeat(n+4));
    println!("{c} {sum}");
}

#[test]
fn test_binary_adder() {
    let num1 = "101101".to_string();
    let num2 = "101100".to_string();
    let n = 6;
    let (c, sum) = binary_add(&num1, &num2, n, '0');
    display_add_sub(num1, num2, c, sum.clone(), n, '+');
    assert_eq!(('1', "011001"), (c, sum.as_str()));
    return;
}

#[test]
fn test_adders() {
    let bits = ['0', '1'];
    let two_bits : Vec<(char, char)> =
        bits.iter().flat_map(|b0| {
            bits.iter().map(|b1| {
                (*b0, *b1)
            })
        }).collect();

    let three_bits : Vec<(char, char, char)> =
        bits.iter().flat_map(|b0| {
            bits.iter().flat_map(|b1| {
                bits.iter().map(|b2| (*b0, *b1, *b2))
            })
        }).collect();
    
    println!("Testing half adder");
    for (a, b) in two_bits {
        let (carry, sum) = half_adder(a, b);
        println!("{a} + {b} = {carry}{sum}");
    }
    
    println!("Testing full adder that uses half adder");
    for (a, b, c) in three_bits.iter().copied() {
        let (carry, sum) = full_adder(a, b, c);
        println!("{a} + {b} + {c} = {carry}{sum}");
    }
    
    println!("Testing another full adder");
    for (a, b, c) in three_bits {
        let (carry, sum) = full_adder_2(a, b, c);
        println!("{a} + {b} + {c} = {carry}{sum}");
    }
}