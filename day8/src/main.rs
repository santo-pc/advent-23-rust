use num::integer::lcm;
use std::{
    collections::HashMap,
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

#[derive(Debug, Clone)]
struct Entry {
    key: String,
    left: String,
    right: String,
}

impl Entry {
    fn new(key: String, left: String, right: String) -> Self {
        Self { key, left, right }
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut map: HashMap<String, Entry> = HashMap::new();
        let mut instructions: Vec<char> = Vec::new();
        lines
            .flatten()
            .filter(|line| !line.is_empty())
            .enumerate()
            .for_each(|(i, line)| {
                if i == 0 {
                    instructions = line.chars().collect();
                } else {
                    let node: String = line[0..3].to_string();
                    let left: String = line[7..line.len() - 1]
                        .split(',')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>()[0]
                        .to_string();
                    let right: String = line[7..line.len() - 1]
                        .split(',')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>()[1]
                        .to_string();

                    let entry = Entry::new(node.clone(), left.clone(), right.clone());
                    map.insert(node.clone(), entry);
                }
            });

        //let steps = part1("AAA", &map, &instructions);
        let steps = part2(&map, &instructions);
        println!("Found at steps: {:?}", steps);
    }
}

// brute for this will take a long type.
// so instead we find all cycles individually and then calculate lcm
// of all of them to know when they'll meet
fn part2(map: &HashMap<String, Entry>, instructions: &Vec<char>) -> u128 {
    let mut steps: u64 = 0;
    let starters: Vec<Entry> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| v.clone())
        .collect();

    let mut loops = starters
        .clone()
        .iter()
        .map(|x| (x.key.clone(), (0, x.clone())))
        .collect::<HashMap<String, (u64, Entry)>>();
    let mut i = 0;
    while !loops.iter().all(|e| e.1 .0 != 0) {
        for current in loops.iter_mut() {
            let entry = map.get(&current.1 .1.key).unwrap();
            let next = if instructions[i] == 'L' {
                &entry.left
            } else if instructions[i] == 'R' {
                &entry.right
            } else {
                panic!("Boom");
            };

            current.1 .1 = map.get(next).unwrap().clone();

            if current.1 .1.key.ends_with('Z') {
                if current.1 .0 == 0 {
                    current.1 .0 = steps + 1;
                }
                println!("Reached loop for starter: {:?}", current);
            }
        }
        steps += 1;
        i += 1;
        if i == instructions.len() {
            i = 0;
        }
    }

    // find the lcm of all cycles
    loops
        .iter()
        .map(|x| x.1 .0 as u128)
        .reduce(num::integer::lcm)
        .unwrap()
}

#[allow(dead_code)]
fn part1(first: &str, map: &HashMap<String, Entry>, instructions: &Vec<char>) -> u64 {
    let mut i = 0;
    let mut steps = 0;
    let mut current: &Entry = map.get(first).unwrap();

    while current.key != "ZZZ" {
        let entry = map.get(&current.key).unwrap();
        let next = if instructions[i] == 'L' {
            &entry.left
        } else if instructions[i] == 'R' {
            &entry.right
        } else {
            panic!("Boom");
        };

        current = map.get(next).unwrap();
        steps += 1;
        i += 1;
        if i == instructions.len() {
            i = 0;
        }
    }
    steps
}
#[test]
fn test_get_steps() {
    unimplemented!()
}
