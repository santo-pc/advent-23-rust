use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const REDS: u32 = 12;
const GREENS: u32 = 13;
const BLUES: u32 = 14;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum_indexes: u32 = 0;
    reader.lines().for_each(|line| {
        if let Ok(line) = line {
            let split: Vec<&str> = line.split(':').collect();
            let game: u32 = split[0]
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let sets: Vec<&str> = split[1].split(';').map(|x| x.trim()).collect();
            println!("line: {:?}", line);
            // println!("game: {:?}", game);
            // println!("sets: {:?}", sets);
            let power = power_of_set(sets);
            println!("power: {:?}", power);
            sum_indexes += power;
        }
    });

    println!("Total: {:?}", sum_indexes);
    Ok(())
}
#[allow(dead_code)]
fn is_possible(set: &str) -> bool {
    let mut blues = 0;
    let mut reds = 0;
    let mut greens = 0;

    set.split(',').map(|x| x.trim()).for_each(|item| {
        // println!("item: {:?}", item);
        let dice: Vec<&str> = item.split_whitespace().collect();
        let num = dice[0].parse::<u32>().unwrap();
        let color = dice[1];

        match color {
            "red" => reds += num,
            "blue" => blues += num,
            "green" => greens += num,
            _ => (),
        }
        println!("num: {:?}, color: {:?}", num, color);
    });

    if reds > REDS || blues > BLUES || greens > GREENS {
        return false;
    }

    true
}

fn power_of_set(sets: Vec<&str>) -> u32 {
    let mut max_blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;

    sets.iter().for_each(|set| {
        set.split(',').map(|x| x.trim()).for_each(|item| {
            // println!("item: {:?}", item);
            let dice: Vec<&str> = item.split_whitespace().collect();
            let num = dice[0].parse::<u32>().unwrap();
            let color = dice[1];

            match color {
                "red" => max_red = std::cmp::max(max_red, num),
                "blue" => max_blue = std::cmp::max(max_blue, num),
                "green" => max_green = std::cmp::max(max_green, num),
                _ => (),
            }
            println!("num: {:?}, color: {:?}", num, color);
        });
    });

    max_green * max_blue * max_red
}
