use std::io;
use std::{thread::sleep, time::Duration};
use rug::Integer;

pub fn receive_work() -> i16 {
    sleep(Duration::from_millis(500));
    println!("\nSelect the type of job you want to do:\n1 to check a prime\n2 to check a pseudoprime\n3 to check a strong pseudoprime\n4 to generate a prime\n5 to quit");
    let choice = receive_number() as i16;

    clear_screen();
    sleep(Duration::from_millis(500));
    choice
}


pub fn receive_prime_check() -> Integer{
    println!("Enter the number to check if it is a prime:");
    let num = receive_big_number();

    clear_screen();
    sleep(Duration::from_millis(500));
    num
}

pub fn receive_pseudoprime_check() -> (Integer, Integer){
    println!("Enter the number to check if it is a pseudoprime and its base:");
    let result = (receive_big_number(), receive_big_number());
    
    clear_screen();
    sleep(Duration::from_millis(500));
    result
}

pub fn receive_strong_pseudoprime_check() -> (Integer, Integer){
    println!("Enter the number to check if it is a strong pseudoprime and its base:");
    let result = (receive_big_number(), receive_big_number());
    
    clear_screen();
    sleep(Duration::from_millis(500));
    result
}

pub fn receive_prime_generation() -> Integer{
    println!("Enter the number of binary digits of your desired prime");
    let result = receive_big_number();
    
    clear_screen();
    sleep(Duration::from_millis(500));
    result
}

fn receive_number() -> u64{
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    let number: u64 = string.trim().parse().expect("Please enter a number!"); 

    number
}

fn receive_big_number() -> Integer{
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    let number: Integer = string.trim().parse().expect("Please enter a number!"); 

    number
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
