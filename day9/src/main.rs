use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // part1();
    part2();
}

fn find_prediction_part2(nums: &[i32]) -> i32 {
    let mut differences: Vec<Vec<i32>> = vec![nums.to_vec()];
    let mut current = 0;
    while differences[current].iter().any(|n| *n != 0) {
        let mut new: Vec<i32> = Vec::new();
        for i in 1..differences[current].len() {
            let prev = i - 1;
            new.push(differences[current][prev] - differences[current][i]);
        }
        differences.push(new);
        current += 1;
    }

    differences.iter().map(|v| v.first().unwrap_or(&0)).sum()
}

fn part2() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: i32 = 0;
        lines
            .flatten()
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(i, line)| {
                let nums = line
                    .split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let prediction = find_prediction_part2(&nums);

                println!("Line {} Prediction {:?}", i, prediction);
                sum += prediction;
            });
        println!("Total is: {:?}", sum);
    }
}

#[allow(dead_code)]
fn part1() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: i32 = 0;
        lines
            .flatten()
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(i, line)| {
                let nums = line
                    .split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let prediction = find_prediction(&nums);

                println!("Line {} Prediction {:?}", i, prediction);
                sum += prediction;
            });
        println!("Total is: {:?}", sum);
    }
}

fn find_prediction(nums: &[i32]) -> i32 {
    let mut differences: Vec<Vec<i32>> = vec![nums.to_vec()];
    let mut current = 0;
    while differences[current].iter().any(|n| *n != 0) {
        let mut new: Vec<i32> = Vec::new();
        for i in 1..differences[current].len() {
            let prev = i - 1;
            new.push(differences[current][i] - differences[current][prev]);
        }
        differences.push(new);
        current += 1;
    }

    differences.iter().map(|v| v.last().unwrap_or(&0)).sum()
}

#[test]
fn test_template() {
    unimplemented!();
}
