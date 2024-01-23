use not_enough_primes::primality::{tests_primes, gen};

use crate::test_lib::interface;

use std::time::Instant;


pub fn check_prime_job() {
    let number = interface::receive_prime_check();
    let primality = tests_primes::is_prime(&number);

    if primality == true {println!("Your number {number} is prime!");}
    else {println!("Your number {number} is not prime!\n:/");}
}

pub fn check_pseudoprime_job() {
    let (number, base) = interface::receive_pseudoprime_check();

    let pseudoprimality: u16 = tests_primes::is_pseudo_prime(&number, &base);

    if pseudoprimality == 1 {
        println!("{number} is a pseudoprime in base {base}!");
    }

    else if pseudoprimality == 2 {
        println!("{number} is a prime!");
    }

    else {println!("{number} is just composite and {base} is one of its witnesses!");}
}

pub fn check_strong_pseudoprime_job() {
    let (number, base) = interface::receive_strong_pseudoprime_check();

    let strong_pseudoprimality: u16 = tests_primes::is_strong_pseudo_prime(&number, &base);

    if strong_pseudoprimality == 1 {
        println!("{number} is a strong pseudoprime in base {base}!");
    }

    else if strong_pseudoprimality == 2 {
        println!("{number} is a prime!");
    }

    else {println!("{number} is just composite and {base} is one of its witnesses!");}
}
pub fn generate_prime_job() {
    let digits = interface::receive_prime_generation();
    
    let start = Instant::now();
    let prime = gen::generate_first_prime_from(&digits);
    let duration = start.elapsed(); 
    
    println!("The first prime of {digits} digits is {prime}!");
    println!("It was discovered after {:?}.", duration);
}

