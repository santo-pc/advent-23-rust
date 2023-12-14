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
#[derive(Debug)]
struct Karte {
    pub matrix: Vec<Vec<char>>,
}

const DIRECTIONS: &[Coord] = &[UP, DOWN, LEFT, RIGHT];
const UP: Coord = (-1, 0);
const DOWN: Coord = (1, 0);
const LEFT: Coord = (0, -1);
const RIGHT: Coord = (0, 1);

impl Karte {
    fn new() -> Self {
        Self { matrix: Vec::new() }
    }

    fn get_at_add(&self, coord1: &Coord, coord2: &Coord) -> Option<&char> {
        self.get_at((coord1.0 + coord2.0, coord1.1 + coord2.1))
    }
    fn get_at(&self, coord: Coord) -> Option<&char> {
        let r = self
            .matrix
            .get(coord.0 as usize)
            .and_then(|row| row.get(coord.1 as usize));
        //println!("Get At {:?}, found: {:?}", coord, r);
        r
    }
}

type Coord = (i32, i32);
fn main() {
    if let Ok(lines) = read_lines("input2.txt") {
        let mut origin: Coord = (0, 0);
        let mut scores: Vec<Vec<u32>> = Vec::new();
        let mut karte = Karte::new();
        lines.flatten().enumerate().for_each(|(i, line)| {
            println!("Line {:?}: {:?}", i, line);
            karte.matrix.push(line.chars().collect::<Vec<char>>());
            scores.push(vec![0; line.len()]);
            if let Some(s_idx) = line.find('S') {
                origin = (i as i32, s_idx as i32);
            }
        });

        println!("Origin: {:?}", origin);
        let (mut dir1, mut dir2) = find_initiators(&karte, &origin);

        let mut current1: Coord = origin;
        let mut current2: Coord = origin;
        let mut steps = 1;
        println!("Starting walk:\n\n");
        loop {
            current1 = add_coords(&current1, &dir1);
            current2 = add_coords(&current2, &dir2);

            dir1 = get_step(dir1, *karte.get_at(current1).unwrap());
            dir2 = get_step(dir2, *karte.get_at(current2).unwrap());

            scores[current1.0 as usize][current1.1 as usize] = 1; //steps;
            scores[current2.0 as usize][current2.1 as usize] = 1; // steps;
            steps += 1;
            if current1 == current2
                || *karte.get_at(current1).unwrap() == 'S'
                || *karte.get_at(current2).unwrap() == 'S'
            {
                break;
            }
        }

        scores.iter().enumerate().for_each(|(_row, v)| {
            v.iter().enumerate().for_each(|(_col, c)| {
                print!("{}\t", c);
            });
            println!();
        });

        let max = scores.iter().flatten().max();
        println!("Max: {}", max.unwrap());
    }
}
fn add_coords(c1: &Coord, c2: &Coord) -> Coord {
    (c1.0 + c2.0, c1.1 + c2.1)
}

fn get_step(dir: Coord, c: char) -> Coord {
    match (c, dir) {
        ('|', UP) => UP,
        ('|', DOWN) => DOWN,
        ('-', LEFT) => LEFT,
        ('-', RIGHT) => RIGHT,
        ('L', DOWN) => RIGHT,
        ('L', LEFT) => UP,
        ('J', DOWN) => LEFT,
        ('J', RIGHT) => UP,
        ('7', RIGHT) => DOWN,
        ('7', UP) => LEFT,
        ('F', UP) => RIGHT,
        ('F', LEFT) => DOWN,
        _ => {
            println!("Crashing with dir {:?} and c {:?}", dir, c);
            (0, 0)
        }
    }
}

fn find_initiators(karte: &Karte, origin: &Coord) -> (Coord, Coord) {
    let mut starters: Vec<Coord> = Vec::new();
    for dir in DIRECTIONS {
        if let Some(c) = karte.get_at_add(origin, dir) {
            if *c == '|' || *c == '-' {
                println!(
                    "Found initiator c: {:?} at: {:?}",
                    c,
                    (origin.0 + dir.0, origin.1 + dir.1)
                );
                starters.push(*dir);
            }
        }
    }
    (starters[0], starters[1])
}

#[test]
fn test_template() {
    unimplemented!();
}
