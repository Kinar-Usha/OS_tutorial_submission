use std::io;

pub fn input_int() -> u32 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");
    let number: u32 = line.trim().parse().expect("!ENter a number");
    return number;
}