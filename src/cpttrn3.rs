use std::io;

fn readline() -> String {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Need to read string");

    buffer.trim().to_string()
}

fn print_pattern(rows: u32, columns: u32) {
    for i in 0..(3*rows) {
        for j in 0..(3*columns) {
            if i % 3 == 0 || j % 3 == 0 {
                print!("*");
            } else {
                print!(".")
            }
        }

        println!("*");
    }

    let final_line = std::iter::repeat("*").take(((columns as usize)*3)+1).collect::<String>();
    println!("{}", final_line)
}

fn main() {
    let numbers = readline().parse::<u32>().expect("expected int");

    let mut chessboards: Vec<(u32, u32)> = vec![];

    for _ in 0..numbers {
        let size_arr: Vec<_> = readline()
            .split(' ')
            .map(|s| s.parse::<u32>().expect("Needed an int"))
            .collect();

        chessboards.push((size_arr[0], size_arr[1]));
    }

    for (row, column) in chessboards {
        print_pattern(row, column);
        println!();
    }
}
