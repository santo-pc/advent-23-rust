use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    hash::Hasher,
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
    if let Ok(lines) = read_lines("input2.txt") {
        let mut galaxy: Vec<Vec<char>> = Vec::new();
        lines.flatten().enumerate().for_each(|(i, line)| {
            println!("Line {:?}: {:?}", i, line);
            galaxy.push(line.chars().collect());
            if !line.contains('#') {
                galaxy.push(line.chars().collect());
            }
        });

        // expand vertically
        let mut col = 0;
        let mut len = galaxy[0].len();
        while col < len {
            if check_col(col, &galaxy) {
                print!("Col {} has no #s", col);
                for r in galaxy.iter_mut() {
                    println!("Inserting");
                    r.insert(col + 1, 'x');
                }
                col += 1;
                len += 1;
            }

            col += 1;
            println!();
        }

        for row in &galaxy {
            println!("{:?}", row);
        }

        let mut hashes: Vec<Coord> = Vec::new();
        galaxy.iter().enumerate().for_each(|(i, v)| {
            v.iter().enumerate().for_each(|(j, c)| {
                if *c == '#' {
                    hashes.push(Coord(i as i32, j as i32));
                }
            })
        });

        for c in &hashes {
            for c2 in &hashes {
                if c != c2 {
                    bfs(&c, &galaxy, &c2);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord(i32, i32);

impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

impl Coord {
    fn new(row: i32, col: i32) -> Self {
        Self(row, col)
    }
}
const DIRECTIONS: &[Coord] = &[UP, RIGHT, DOWN, LEFT];
const UP: Coord = Coord(-1, 0);
const DOWN: Coord = Coord(1, 0);
const LEFT: Coord = Coord(0, -1);
const RIGHT: Coord = Coord(0, 1);

fn bfs(current: &Coord, galaxy: &[Vec<char>], target: &Coord) -> u32 {
    let start = current.clone();
    let mut queue: Vec<Coord> = Vec::new();
    let mut visited: HashSet<Coord> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        visited.insert(current);

        // let (row, col) = (current.0, current.1);

        for coord in neighbors(galaxy, current) {
            if visited.contains(&coord) {
                queue.push(coord);
            }
        }
    }
    1
}

fn neighbors(galaxy: &[Vec<char>], coord: Coord) -> Vec<Coord> {
    let mut n = Vec::new();
    for dir in DIRECTIONS {
        let place: Coord = Coord(coord.0 + dir.0, coord.1 + dir.1);
        if let Some(r) = galaxy.get(place.0 as usize) {
            if r.get(place.1 as usize).is_some() {
                n.push(place);
            }
        }
    }
    n
}
fn check_col(col: usize, matrix: &[Vec<char>]) -> bool {
    matrix.iter().all(|row| row[col] == '.')
}

#[test]
fn test_template() {
    unimplemented!();
}
