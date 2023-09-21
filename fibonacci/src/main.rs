use std::u128;
use std::str;

fn main() {
    // these numbers are the ones used to generate the fibonacci sequence.
    // this is done by adding the previous two values together, starting 
    // with one (f1) and two (f2)
    let mut f1 = 0u128;
    let mut f2 = 1u128;

    // this is here to show the two initial values
    println!("{}\n{}", f1, f2);

    // this is just the itterator to ensure that the while loop doesn't 
    // go too far and overflow the u128 
    let mut i = 0u128;

    // this is the main loop. It can only go for 185 itterations before 
    // overflowing a u128
    while i < 185 { 
        i += 1;

        // calculates the next number
        let three = f2 + f1;
       
        // sends the next value to be formatted and printed 
        print_num(three);

        // if you think of the equation to find the fibonacci sequence, 
        // Fn(three) = Fn-1(f2) + Fn-2(f1), this part shifts everything to the 
        // right by one, working from right to left:  
        // Fn-1 -> Fn-2, then Fn -> Fn-1
        f1 = f2;
        f2 = three;
    }
    
    // print the number of itterations it took and exit
    println!("\nThe final number was the {}th number \
             in the fibonacci sequence.", i);
}

// from https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust
// i added did this so the program would be package free. I should figure out 
// how it works someday. 
fn print_num(num: u128) {
    let new_num = num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");  // separator

    println!("{new_num}");
}
