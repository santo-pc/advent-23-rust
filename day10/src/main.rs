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
type Coord = (i32, i32);
fn main() {
    if let Ok(lines) = read_lines("input2.txt") {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut origin: Coord = (0, 0);
        let mut scores: Vec<Vec<u32>> = Vec::new();
        lines.flatten().enumerate().for_each(|(i, line)| {
            println!("Line {:?}: {:?}", i, line);
            matrix.push(line.chars().collect::<Vec<char>>());
            scores.push(vec![0; line.len()]);
            if let Some(s_idx) = line.find('S') {
                origin = (i as i32, s_idx as i32);
            }
        });

        println!("Origin: {:?}", origin);
        let (one, two) = find_initiator(&matrix, origin);
        matrix.iter().for_each(|v| {
            for c in v {
                print!("{}", c);
            }
            println!();
        });
    }
}
const DIRECTIONS: &[Coord] = &[up, down, left, right];
const up: Coord = (-1, 0);
const down: Coord = (1, 0);
const left: Coord = (0, -1);
const right: Coord = (0, 1);
fn find_initiator(matrix: &[Vec<char>], origin: Coord) -> (Coord, Coord) {
    for dir in DIRECTIONS {
        origin
    }
    ((0, 0), (0, 0))
}

#[test]
fn test_template() {
    unimplemented!();
}
