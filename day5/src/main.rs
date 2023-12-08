use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{self, BufRead},
    ops::Range,
    path::Path,
};

#[derive(Debug, Default, Clone)]
struct Mapper {
    name: String,
    table: HashMap<Range<u64>, Range<u64>>, // origin to destination
    inverse_table: HashMap<Range<u64>, Range<u64>>, // destination to origin
}

impl Mapper {
    fn new(name: String) -> Self {
        Self {
            name,
            table: HashMap::new(),
            inverse_table: HashMap::new(),
        }
    }

    fn insert(&mut self, source: &Range<u64>, dest: &Range<u64>) {
        self.table.insert(source.clone(), dest.clone());
        self.inverse_table.insert(dest.clone(), source.clone());
    }

    fn find_or_default_reverse(&self, num: u64) -> u64 {
        let mut result: u64 = num;
        if let Some(ranges) = self
            .inverse_table
            .iter()
            .find(|re| re.0.start <= num && re.0.end >= num)
        {
            result = ranges.1.start + (num - ranges.0.start);
            // println!("Found in mapper {:?}: {:?}-> {:?}", self.name, num, result);
        } else {
            // println!("Not Found in mapper {:?} using num {:?}", self.name, num);
        }

        result
    }

    fn find_or_default(&self, num: u64) -> u64 {
        let mut result: u64 = num;
        if let Some(ranges) = self
            .table
            .iter()
            .find(|re| re.0.start <= num && re.0.end >= num)
        {
            result = ranges.1.start + (num - ranges.0.start);
        }

        result
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "input.txt";
    // brute_force(filename);
    reverse_search(filename);
}

fn reverse_search<P>(filename: P)
where
    P: AsRef<Path>,
{
    let (mut mappers, mut seeds) = build_mappers(filename);
    mappers.reverse();
    seeds.sort();

    // println!("Mappers: {:?}", mappers);

    let locations: Vec<u64> = mappers
        .iter()
        .find(|m| m.name == "humidity-to-location map:")
        .expect("Expected humydity map")
        .inverse_table
        .values()
        .flat_map(|location_r| location_r.clone().collect::<Vec<u64>>())
        .collect();

    // println!("Locations: {:?}", locations);
    let mut location_min = u64::MAX;
    //for location in locations.iter() {
    for location in 0u64.. {
        if location % 100000 == 0 {
            println!("Checking {}", location);
        }
        let found_seed = map_in_mappers_reverse(&mappers, location);

        if seeds.iter().any(|seed| *seed == found_seed) && location_min > location {
            location_min = location;
            println!("Min Location: {:?}", location_min);
            return;
        }
    }

    println!("Min Location: {:?}", location_min);
}

#[allow(dead_code)]
fn brute_force<P>(filename: P)
where
    P: AsRef<Path>,
{
    let mut location_min: u64 = u64::MAX;
    let (mappers, seeds) = build_mappers(filename);

    println!("Seeds: {:?}", seeds);
    seeds
        .chunks(2)
        .map(|chunk| chunk[0]..=chunk[0] + chunk[1] - 1)
        .for_each(|range| {
            println!("Range: {:?}", range);
            for seed in range {
                location_min = std::cmp::min(map_in_mappers(&mappers, seed), location_min);
            }
        });

    println!("Min Location: {:?}", location_min);
}

fn build_mappers<P>(filename: P) -> (Vec<Mapper>, Vec<u64>)
where
    P: AsRef<Path>,
{
    let mut mappers: Vec<Mapper> = Vec::new();

    let mut numbers: String = "".to_string();
    if let Ok(lines) = read_lines(filename) {
        let mut current: Mapper = Mapper::new("".to_owned());

        lines
            .flatten()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .for_each(|(i, line)| {
                // println!("Line {:?}: {:?}", i, line);

                if line.starts_with("seeds:") {
                    numbers = line.clone();
                } else if line.contains("map:") {
                    mappers.push(current.clone());
                    current = Mapper::new(line.to_owned());
                } else {
                    let line_parsed: Vec<u64> =
                        line.split(' ').flat_map(|n| n.parse::<u64>()).collect();

                    let (dest_start, source_start, len) =
                        (line_parsed[0], line_parsed[1], line_parsed[2]);
                    let dest: Range<u64> = dest_start..dest_start + len + 1;
                    let source: Range<u64> = source_start..source_start + len + 1;
                    current.insert(&source, &dest);
                }
            });

        mappers.push(current);
    }

    let numbers: Vec<u64> = numbers.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .flat_map(|n| n.parse::<u64>())
        .collect();
    (mappers, numbers)
}

fn map_in_mappers_reverse(mappers: &[Mapper], input: u64) -> u64 {
    mappers
        .iter()
        .filter(|mapper| !mapper.name.is_empty())
        .fold(input, |current, mapper| {
            mapper.find_or_default_reverse(current)
        })
}
fn map_in_mappers(mappers: &[Mapper], input: u64) -> u64 {
    mappers
        .iter()
        .filter(|mapper| !mapper.name.is_empty())
        .fold(input, |current, mapper| mapper.find_or_default(current))
}

#[test]
fn test_template() {
    unimplemented!();
}
