use std::{
    collections::{HashMap, HashSet},
    fs::File,
    i32,
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
fn printmap(mymap: &HashMap<usize, u32>) {
    mymap
        .into_iter()
        .for_each(|e| println!("Line {}: {}", e.0, e.1))
}
// #[allow(unused_variables, unused_mut)]
fn main() {
    let mut map: HashMap<usize, u32> = HashMap::new();
    let mut n_lines: usize = 0;
    if let Ok(lines) = read_lines("input.txt") {
        lines.flatten().enumerate().for_each(|(i, line)| {
            n_lines += 1;
            let line_score = line_score(line);
            println!("Line {} has {} matches", i, line_score);
            if let Some(count) = map.get_mut(&i) {
                // update in case of copies
                *count += 1;
            } else {
                // If not in map, insert for the first time
                map.insert(i, 1);
            }

            // propagate copies
            if line_score > 0 {
                let copies = map.get(&i).unwrap_or(&0);
                for _ in 0..*copies {
                    for n in i + 1..i + line_score as usize + 1 {
                        if let Some(count) = map.get_mut(&n) {
                            *count += 1;
                        } else {
                            map.insert(n, 1);
                        }
                    }
                }
            }

            printmap(&map);
        });
    }

    let sum: u32 = map
        .iter()
        .filter(|entry| entry.0 <= &n_lines)
        .map(|entry| entry.1)
        .sum();
    println!("Total: {:?}", sum)
}

fn line_score(line: String) -> u32 {
    let vec_split = line
        .split(|c| c == ':' || c == '|')
        .map(|split| split.trim())
        .collect::<Vec<&str>>();

    let winning: HashSet<i32> = HashSet::from_iter(
        vec_split[1]
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<i32>().unwrap()),
    );
    let set: HashSet<i32> = HashSet::from_iter(
        vec_split[2]
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<i32>().unwrap()),
    );
    let intersection: HashSet<_> = winning.intersection(&set).collect();
    println!("intersection: {:?}", intersection);
    intersection.len() as u32
}
