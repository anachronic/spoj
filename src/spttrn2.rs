use std::io;

fn readline() -> String {
    let mut dest = String::new();
    io::stdin().read_line(&mut dest).expect("Need to read string");

    dest.trim().to_string()
}

fn fill_str(n: usize, c: &str) -> String {
    std::iter::repeat(c).take(n).collect()
}

fn print_top(input: usize) {
    let mut prefix = String::from("*.");
    let mut suffix = String::from("*");

    let mut filler = '*';

    print!("{}", prefix);
    print!("{}", fill_str(input - prefix.len() - suffix.len(), "*"));
    println!("{}", suffix);

    for _ in 1..(input - 1) {
        let remaining_chars = input as i32 - prefix.len() as i32 - suffix.len() as i32;
        if remaining_chars <= 0 {
            break;
        }

        if remaining_chars > 1 {
            prefix.push(filler);
        }

        if filler == '*' || remaining_chars == 1 {
            filler = '.'
        } else {
            filler = '*'
        }

        suffix.insert(0, filler);


        print!("{}", prefix);
        print!("{}", fill_str(if remaining_chars >= 2 { remaining_chars as usize - 2 } else { 0 }, &String::from(filler)));
        println!("{}", suffix);
    }
}

fn print_bottom(input: usize, prefix: &String, suffix: &String, filler: char) {
    let remaining_chars = input as i32 - prefix.len() as i32 - suffix.len() as i32;

    if remaining_chars == 0 {
        return
    } else if remaining_chars == 1 {
        let mut the_filler = '.';

        if prefix.chars().last().unwrap() == '.' && suffix.chars().next().unwrap() == '.' {
            the_filler = '*';
        }

        return println!("{}{}{}", prefix, the_filler, suffix);
    }

    let new_prefix = format!("{}{}", prefix, filler);
    let new_suffix = format!("{}{}", filler, suffix);

    let new_filler = if filler == '*' { '.' } else { '*' };

    print_bottom(input, &new_prefix, &new_suffix, new_filler);
    println!("{}{}{}", prefix, fill_str(remaining_chars as usize, &String::from(filler)), suffix);
}

fn print_solution(input: usize) {

    print_top(input);
    print_bottom(input, &String::from("*"), &String::from("*"), '.');
    println!("{}", fill_str(input, "*"));
}


fn main() {
    let numbers: u32;

    let buf = readline();
    numbers = buf.parse::<u32>().expect("expected int");

    let mut inputs: Vec<u32> = vec![];
    for _ in 0..numbers {
        let buf = readline();

        let input = buf.parse::<u32>().expect("needed an int");

        inputs.push(input);
    }

    for input in inputs {
        print_solution(input as usize);
        println!()
    }
}
