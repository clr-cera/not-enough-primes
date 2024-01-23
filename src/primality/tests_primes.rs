use rug::Integer;

use crate::primality::tests_composites;
use crate::primality::manipulations;

/// This function checks if a number is prime using miller-rabin test of composites, if it returns
/// true, then the informed number has 1/4^10 of probability of being a prime
pub fn is_prime_miller(number: &Integer, base: &Integer) -> bool {
    let copy = number.clone();

    if copy == 0 || copy == 1 || copy % 2 == 0 { 
        return false;
    }

    if tests_composites::composite_test_miller_rabin(number, base) == true {
        return false;
    }

    return true;
}

pub fn is_prime_fermat(number: &Integer, base: &Integer) -> bool {
    let copy = number.clone();

    if copy == 0 || copy == 1 || copy % 2 == 0 { 
        return false;
    }

    if tests_composites::composite_test_fermat(number, base) == true {
        return false;
    }


    return true;
}

pub fn is_prime_baillie(number: &Integer) -> bool {
    let copy = number.clone();

    if copy == 0 || copy == 1 || copy % 2 == 0 { 
        return false;
    }

    if tests_composites::composite_test_miller_rabin(number, &Integer::from(2)) == true {
        return false;
    }

    let mut d: Integer = Integer::from(5);

    while manipulations::jacobi(&d, number) != -1{
        if d < 0 {
            d = (d-2) * -1;
        }
        else {
            d = (d+2) * -1;
        }
    }
    
    let p = Integer::from(1);
    let q = Integer::from((1 - d.clone())/4);

    if tests_composites::composite_test_lucas(number, &d, &p, &q) == true {
        return false;
    }


    return true;
}

pub fn is_prime(number: &Integer) -> bool { 
    let copy = number.clone();

    if copy == 0 || copy == 1 || copy % 2 == 0 { 
        return false;
    }
   
    let mut base = Integer::from(2);
    
    if tests_composites::composite_test_fermat(number, &base) == true {return false;}

    loop {
        if tests_composites::composite_test_miller_rabin(number, &base) == true {
            return false;
        }
        if base == 4 {break}
        base +=1;
    }

    return true;
}

/// This function checks if a number is a pseudoprime. It returns 1 if the number is a pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_pseudo_prime(number: &Integer, base: &Integer) -> u16 {
    let fermat_result = tests_composites::composite_test_fermat(number, base);
    let brute_result = tests_composites::composite_test_bruteforce(number);

    if fermat_result == false && brute_result == true { return 1;}
    else if brute_result == false { return 2;}
    else {return 0;}

}

/// This function checks if a number is a strong pseudoprime. It returns 1 if the number is a strong pseudoprime,
/// 2 if the number is prime and 0 if the number is composite.
pub fn is_strong_pseudo_prime(number: &Integer, base: &Integer) -> u16 {
    let miller_result = tests_composites::composite_test_miller_rabin(number, base);
    let brute_result = tests_composites::composite_test_bruteforce(number);

    if miller_result == false && brute_result == true { return 1;}
    else if brute_result == false { return 2;}
    else {return 0;}

}

