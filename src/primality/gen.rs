use rug::Integer;

use super::tests_primes::is_prime;
use crate::primality::manipulations::pow;

/// This function generates the first prime with informed digits. It uses miller-rabin test of
/// composites.
pub fn generate_first_prime_from(digits: &Integer) -> Integer{
    let mut number: Integer = pow(&Integer::from(2), &Integer::from(digits-1));

    loop {
        if is_prime(&number) {return number;}
        number+=1;
    }
}
