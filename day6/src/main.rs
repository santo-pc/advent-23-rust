use std::{
    fs::File,
    io::{self, BufRead, Lines},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy)]
struct Record {
    time: u64,
    distance: u64,
    _moves: u64,
}

impl Record {
    fn new(time: u64, distance: u64, moves: u64) -> Self {
        Self {
            time,
            distance,
            _moves: moves,
        }
    }
}

fn main() {
    let filename = "input.txt";
    //part1(filename);
    part2(filename);
}

fn part2<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let record = build_records_for_part2(lines);
        println!("Total moves: {:?}", calc_moves(&record, 14));
    }
}

#[allow(dead_code)]
fn part1<P>(filename: P)
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let records = build_records(lines);
        let total = records
            .iter()
            .map(|r| calc_moves(r, 0))
            .reduce(|acc, e| acc * e);
        println!("total: {:?}", total);
    }
}

fn calc_moves(record: &Record, start: u64) -> u64 {
    (start..=record.time)
        .map(|pushed_for_ms| (record.time - pushed_for_ms) * pushed_for_ms > record.distance)
        .filter(|x| *x)
        .count() as u64
}

fn build_records_for_part2(lines: Lines<io::BufReader<File>>) -> Record {
    let two_rows = lines
        .flatten()
        .map(|line| {
            line.split(':').collect::<Vec<&str>>()[1]
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();

    Record::new(two_rows[0], two_rows[1], 0)
}

#[allow(dead_code)]
fn build_records(lines: Lines<io::BufReader<File>>) -> Vec<Record> {
    let two_rows = lines
        .flatten()
        .map(|line| {
            println!("Line: {:?}", line);
            line.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .filter(|n| !n.is_empty())
                .flat_map(|n| n.to_owned().parse::<u64>())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    two_rows[0]
        .iter()
        .zip(two_rows[1].iter())
        .map(|(time, distance)| Record::new(*time, *distance, 0))
        .collect::<Vec<Record>>()
}

#[test]
fn test_template() {
    unimplemented!();
}
