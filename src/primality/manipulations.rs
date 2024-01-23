use rug::Integer;

/// This function decomposes a number into number = 2^exp + q. It returns (exp, q).
pub fn miller_decompose(number: &Integer) -> (Integer, Integer){
    if number.clone() % 2 == 0 {return (Integer::ZERO, Integer::ZERO);}

    let mut exp: Integer = Integer::new();
    let mut odd: Integer = number.clone() - 1;

    loop {
        if odd.clone() % 2 != 0 {break;}

        odd = odd / 2;
        exp += 1;
    }
    (exp, odd)
}

/// This function calculates the jacobi symbol of a and n.
pub fn jacobi(a: &Integer, n: &Integer) -> Integer {
    let mut a = a.clone();
    let mut n = n.clone();

    a = a % &n;
    
    let mut t = Integer::from(1);
    let mut r;

    while a != 0 {
        while a.clone() % 2 == 0 {
            a /= 2;
            r = n.clone() % 8;
            
            if r == 3 || r == 5{
                t = -t;
            }
        }

        r = n;
        n = a;
        a = r;
        
        if a.clone() % 4 == 3 && n.clone() % 4 == 3 {
            t = -t;
        }
        a = a % &n;
    }

    if n == 1 {return t;} 
    else {return Integer::ZERO}
}

pub fn pow(number: &Integer, exp: &Integer) -> Integer {
    let mut result: Integer = Integer::from(1);
       
    // Just to build a for loop using Integer
    let mut i = Integer::from(1);

    if exp.clone() != Integer::ZERO {
        loop {
            result = result * number;
            
            if &i == exp {break} //FOR
            i += 1
        }
    }

    return result;
}

