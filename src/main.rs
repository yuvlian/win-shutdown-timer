use std::io;
use std::process::Command;

fn main() {
    // time unit selection, obvious idk why im even commenting this
    println!("hi, welcome to shutdowner 3000!\nselect the time unit u want for the shutdown timer:");
    println!("[1] Seconds");
    println!("[2] Minutes");
    println!("[3] Hours");
    println!("[4] Weeks");
    println!("[5] Months");
    println!("or, CTRL+C to exit!");

    // read time unit selection :3
    let mut time_unit_input = String::with_capacity(1); 
    io::stdin().read_line(&mut time_unit_input).unwrap();

    // parse the selection!!!
    let time_unit: u8 = match time_unit_input.trim().parse() {
        Ok(num) if num >= 1 && num <= 5 => num,
        _ => {
            println!("Bro, please select a number between 1 and 5.");
            return;
        }
    };

    println!("enter the amount of time:");

    // read time value input
    let mut time_value_input = String::new();
    io::stdin().read_line(&mut time_value_input).unwrap();

    // turn it into u64
    let time_value: u64 = match time_value_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input"); // skill issue!
            return;
        }
    };

    // convert all times to seconds
    let seconds = match time_unit {
        1 => time_value, // seconds, duh
        2 => time_value * 60, // minutes
        3 => time_value * 60 * 60, // hours
        4 => time_value * 60 * 60 * 24 * 7, // weeks
        5 => time_value * 60 * 60 * 24 * 30, // months with approx of 30 days
        _ => unreachable!(), // literally unreachable
    };

    println!("shutting down in {} seconds", seconds);

    // run window shutdown command!
    let shutdown_command = Command::new("shutdown")
        .args(&["/s", "/t", &seconds.to_string()])
        .output();

    match shutdown_command {
        Ok(output) => {
            if output.status.success() {
                println!("command has been executed.");
            } else {
                println!("command execution failed...");
            }
        }
        Err(e) => {
            println!("hey error! look! {}", e);
        }
    }
}
