use std::io;

fn readline() -> String {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Need to read string");

    buffer.trim().to_string()
}

fn print_chessboard(rows: u32, columns: u32) {
    let mut current;

    for i in 0..rows {
        if i % 2 == 0 {
            current = '*';
        } else {
            current = '.';
        }

        for _ in 0..columns {
            print!("{}", current);

            current = if current == '*' { '.' } else { '*' };
        }

        println!();
    }
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
        print_chessboard(row, column);
        println!();
    }
}
