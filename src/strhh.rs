use std::io;

fn readline() -> String {
    let mut dest = String::new();
    io::stdin().read_line(&mut dest).expect("Need to read string");

    dest.trim().to_string()
}

fn print_solution(ztr: &String) {
    let bytestr = ztr.as_bytes();

    for i in 0..(bytestr.len() / 2) {
        let byte = char::from_u32(bytestr[i].into()).expect("fak me");

        if i % 2 == 0 {
            print!("{}", byte);
        }
    }
}

fn main() {
    let numbers: u32;

    let buf = readline();
    numbers = buf.parse::<u32>().expect("expected int");

    let mut strs: Vec<String> = vec![];
    for _ in 0..numbers {
        let buf = readline();

        strs.push(String::from(&buf));
    }

    for ztr in strs {
        print_solution(&ztr);
        println!();
    }
}
