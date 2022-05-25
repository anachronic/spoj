use std::io;

fn readline() -> String {
    let mut dest = String::new();
    io::stdin().read_line(&mut dest).expect("Need to read string");

    dest.trim().to_string()
}

fn print_solution(n: u32, x: u32, y: u32) {
    let mut result = String::new();

    for ai in 0..n {
        if ai % x == 0 && ai % y != 0 {
            result = format!("{} {}", result, ai);
        }
    }

    println!("{}", result.trim());
}

fn main() {
    let numbers: u32;

    let buf = readline();
    numbers = buf.parse::<u32>().expect("expected int");

    let mut inputs: Vec<(u32, u32, u32)> = vec![];
    for _ in 0..numbers {
        let buf = readline();

        let input: Vec<u32> = buf
            .split(' ')
            .map(|s| s.parse::<u32>().expect("pass me an int"))
            .collect();

        inputs.push((input[0], input[1], input[2]));
    }

    for (n, x, y) in inputs {
        print_solution(n, x, y);
    }
}
