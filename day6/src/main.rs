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

#[derive(Debug, Clone, Copy)]
struct Record {
    time: u32,
    distance: u32,
}

fn main() {
    let filename = "input2.txt";
    part1(filename);
}

fn part1<P>(filename: P)
where
    P: AsRef<Path>,
{
    let records: Vec<Record> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mylines: Vec<Vec<String>> = lines
            .flatten()
            .map(|line| {
                println!("Line: {:?}", line);
                &line
                    .split(&[':', ' '])
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[1..]
                    .into_iter()
                    .filter(|i| !i.is_empty())
                    .collect::<Vec<String>>();
            })
            .collect();

        println!("Foo {:?}", mylines);
    }
}

#[test]
fn test_template() {
    unimplemented!();
}
