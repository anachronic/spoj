use std::io;

fn readline() -> String {
    let mut dest = String::new();
    io::stdin().read_line(&mut dest).expect("Need to read string");

    dest.trim().to_string()
}

fn main() {
    let numbers: u32;

    let buf = readline();
    numbers = buf.parse::<u32>().expect("expected int");

    print!("W");
    print!("{}", std::iter::repeat("o").take(numbers as usize).collect::<String>());
    println!("w")
}
