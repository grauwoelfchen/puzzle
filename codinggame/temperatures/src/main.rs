use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _n = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut data: Vec<i32> = vec![];

    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        data.push(t);
    }

    let negative = data.iter().filter(|n| **n < 0).max();
    let positive = data.iter().filter(|n| **n > 0).min();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    println!(
        "{}",
        match (positive, negative) {
            (Some(p), Some(n)) => {
                if n.abs() >= *p {
                    p
                } else {
                    n
                }
            }
            (Some(p), None) => p,
            (None, Some(n)) => n,
            _ => &0,
        }
    );
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(1, 1);
    }
}
