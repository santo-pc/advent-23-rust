use std::fs::File;
use std::io::{self, BufRead};

const VALID_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    // let result: i32 = part1(reader);
    let result: i32 = part2(reader);

    println!("Result: {}", result);
    Ok(())
}

fn part2(reader: io::BufReader<File>) -> i32 {
    let mut vector: Vec<(Option<i32>, Option<i32>, String)> = Vec::new();
    for (line_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let len = line.len();

        if !line.is_empty() {
            vector.push((None, None, line.to_owned()));
        }

        println!("Line {}: {}", line_index + 1, line);
        for i in 1..=len {
            if vector[line_index].0.is_none() {
                if let Some(first) = find_in_slice(&line[..i]) {
                    println!("found first: {}", first);
                    vector[line_index].0 = Some(first);
                }
            }

            if vector[line_index].1.is_none() {
                if let Some(second) = find_in_slice(&line[len - i..]) {
                    println!("found second: {}", second);
                    vector[line_index].1 = Some(second);
                }
            }

            if vector[line_index].0.is_some() & vector[line_index].1.is_some() {
                continue;
            }
        }
    }

    let mut r = 0;
    let mut count = 0;

    for t in vector {
        if t.0.is_some() && t.1.is_some() {
            println!("Tuple {:?}", t);
            let num = t.0.unwrap() * 10 + t.1.unwrap();
            r += num;
            println!("num: {:?}", num);
            count += 1;
        }
    }
    println!("Numbers found: {}", count);
    r

    // vector
    //     .iter()
    //     .filter(|tuple| tuple.0.is_some() && tuple.0.is_some())
    //     .map(|tuple| {
    //         println!("Tuple {:?}", tuple);
    //         tuple.1.unwrap() * 10 + tuple.0.unwrap()
    //     })
    //     .for_each(|n| println!("Number={}", n));
    //
    // vector
    //     .iter()
    //     .filter(|tuple| tuple.0.is_some() && tuple.0.is_some())
    //     .map(|tuple| tuple.0.unwrap() * 10 + tuple.1.unwrap())
    //     .sum()
}

fn find_in_slice(str: &str) -> Option<i32> {
    println!("Slice: {}", str);
    for i in 0..str.len() {
        let c = str.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            return Some(c.to_digit(10).unwrap() as i32);
        }
        for (index, num_word) in VALID_NUMBERS.iter().enumerate() {
            if str.contains(num_word) {
                return Some(index as i32 + 1);
            }
        }
    }
    None
}

#[allow(dead_code)]
fn part1(reader: io::BufReader<File>) -> i32 {
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .map(|chars: Vec<char>| chars.into_iter().filter(|c| c.is_numeric()).collect())
        .filter(|chars: &Vec<char>| !chars.is_empty())
        .map(|numbers: Vec<char>| {
            let mut my_string = numbers.first().unwrap().to_string();
            my_string.push_str(&numbers.last().unwrap().to_string());
            let num = my_string.parse::<i32>().unwrap();
            println!("num={}", num);
            num
        })
        .sum()
}
