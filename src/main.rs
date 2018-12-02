use std::io;
use std::str::FromStr;

fn main() {
    println!("Let's play a guessing game. You pick a number under 1000. I'll guess it.");
    println!("Press enter to begin.");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line :(");
    
    println!("Type h for too high, l for too low, or y if I got it.");
    let res = match binary_search(0, 1000, human_compare) {
        Ok(result) => result,
        Err(_) => return,
    };
    println!("Yay we found it! {}", res);
}

fn human_compare(x: u32) -> i32 {
    println!("Is {} your number?", x);

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line :(");

    let output: i32 = match user_input.trim().to_ascii_lowercase().as_ref() {
        "h" => 1,
        "l" => -1,
        &_ => 0, // treat any input as "yes" if not high/low
    };
    return output;
}

fn binary_search(low: u32, high: u32, compare_fn: fn (u32) -> i32) -> Result<u32, String> {
    if low == high {
        return Ok(low);
    }

    if low > high {
        // how do strings work in rust pls teach me
        let err_string = match String::from_str("Binary search failed") {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        return Err(err_string);
    }

    // get mid and test it
    let mid = (high + low) / 2;
    let res = compare_fn(mid);
    if res > 0 {
        // too high, move high down
        return binary_search(low, mid-1, compare_fn);
    } else if res < 0 {
        // too low, move low up
        return binary_search(mid+1, high, compare_fn);
    }

    return Ok(mid);
}
