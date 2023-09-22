use num_bigint::BigUint;        // for getting larger integers than u128
use num_traits::ToPrimitive;    // for turning BigUints back to regular ints
use std::io;                    // basic input from user

fn main() {
    // initialize the input
    let mut input = String::new();
    
    // get the input
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    // parse the input into a u32 for processing
    let number: u32 = match input.trim().parse() {
        // if it parses ok, return the numer
        Ok(num) => num,
        // if it doesn't parse correctly, return an error
        Err(_) => {
            eprintln!("Invalid input. Please enter a positive integer.");
            return;
        }
    };

    // turn the parsed user input into a BigUint 
    let big_input = BigUint::from(number);
    
    // prints the fibonacci number in the position that is input into it
    println!("{}", fibonacci(big_input));
}


fn fibonacci(n: BigUint) -> BigUint {
    // initializes the vector with the numbers 0 and 1 in their respective 
    // indexes
    let mut f = vec![BigUint::from(0u32), BigUint::from(1u32)];

    // for actuallu calculating every fibonacci number 
    for i in 2..(n.to_usize().unwrap() + 1) {
        // the next value is the value in the position behind it plus the value
        // that is behind that number 
        let next_value = &f[i - 1] + &f[i - 2];
        // push the new value to the main vector
        f.push(next_value.clone());
    }
    // return the last number
    f[n.to_usize().unwrap()].clone() 
}

