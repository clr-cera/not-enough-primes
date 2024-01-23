use rug::Integer;

use crate::primality::manipulations::{pow, self};

/// This function implements fermat test of composites, returning true if the number is a composite
/// and false if the number is a probable prime.
pub fn composite_test_fermat(number: &Integer, test: &Integer) -> bool{
    let fermat_result: Integer = test.pow_mod_ref(&Integer::from(number-1), number).unwrap().into();

    if fermat_result != 1 {return true;}
    else {return false;}
}

/// This function implements miller-rabin test of composites, returning true if the number is a composite
/// and false if the number is a strong probable prime.
pub fn composite_test_miller_rabin(number: &Integer, base: &Integer) -> bool{
    let (exp, constant) =  manipulations::miller_decompose(&number);
    let minus_one = Integer::from(number-1);

    let first: Integer = base.pow_mod_ref(&constant, number).unwrap().into();
    if first == 1 {return false}

    let mut r = Integer::from(0);

    loop {
        let result: Integer = base.pow_mod_ref(&(&constant * pow(&Integer::from(2), &r)), number).expect("There is no result").into();
        if result == minus_one {return false}
        
        if r == exp {break}
        r +=1;
    }

    return true;

}

/// This function bruteforces its way on checking if a number is composite, it never mistakes but
/// has root of n complexity.
pub fn composite_test_bruteforce(number: &Integer) -> bool{
    let mut possible_divisor = Integer::from(2);
    
    let number = number.clone();
    let square = number.clone().sqrt();

    loop {
        if number.clone() % &possible_divisor == 0 {return true;}
    
        if possible_divisor == square {break}
        possible_divisor +=1
    }
    return false;
} 

/// This function checks if a number is a composite using lucas probable prime test
pub fn composite_test_lucas(number: &Integer, d: &Integer, p: &Integer, q: &Integer) -> bool {

    return true;
}


