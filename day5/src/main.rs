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
    if let Ok(lines) = read_lines("input.txt") {
        lines.flatten().enumerate().for_each(|(i, line)| {
            println!("Line {:?}: {:?}", i, line);
        });
    }
}

#[test]
fn test_template() {
    unimplemented!();
}
