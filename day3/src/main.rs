use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Number {
    row: i32,
    col: i32,
    number: String,
}
impl Number {
    fn new(row: i32, col: i32, number: String) -> Number {
        Number { row, col, number }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut total: u32 = 0;

    reader
        .lines()
        .flatten()
        .enumerate()
        .for_each(|(line_n, line)| {
            println!("Line {}: {:?}", line_n, line);
            matrix.push(line.chars().collect());
        });

    println!("Matrix:");
    matrix.iter().enumerate().for_each(|(row_id, row)| {
        let total_row = extract_number_in_line(row, row_id)
            .iter()
            .map(|n| (n, has_symbol_adjacent(n, &matrix)))
            .filter(|(_, r)| r.0)
            .map(|(n, _)| n.number.parse::<u32>().unwrap())
            .sum::<u32>();
        println!("total_row: {}", total_row);
        total += total_row;
    });

    println!("Total: {}", total);
    Ok(())
}

fn extract_number_in_line(line: &[char], row_id: usize) -> Vec<Number> {
    let mut num = "".to_owned();
    let mut result: Vec<Number> = Vec::new();

    for (c_ix, c) in line.iter().enumerate() {
        if c.is_ascii_digit() {
            num.push(*c)
        }

        // should flush
        if (!c.is_ascii_digit() && !num.is_empty()) || (!num.is_empty() && c_ix == line.len() - 1) {
            result.push(Number::new(
                row_id as i32,
                c_ix as i32 - num.len() as i32,
                num.to_owned(),
            ));
            // dbg!("Got:", num);

            num = "".to_owned();
        }
    }
    result
}

fn has_symbol_adjacent(number: &Number, matrix: &Vec<Vec<char>>) -> (bool, char) {
    let row = number.row as usize;
    let col = number.col as usize;
    let upper_row = std::cmp::max(number.row - 1, 0) as usize;
    let lower_row = std::cmp::min(number.row + 1, matrix.len() as i32 - 1) as usize;

    let left_hand_col = std::cmp::max(col as i32 - 1, 0) as usize;
    let right_hand_col = std::cmp::min(number.number.len() + col + 1, matrix[row].len() - 1);

    let r = &matrix[upper_row..=lower_row]
        .to_vec()
        .iter()
        .flat_map(|r| {
            let row_slice = r[left_hand_col..right_hand_col].to_vec();
            print!(
                "Number: {}, Row: {}, Col: {}",
                number.number, number.row, number.col
            );
            println!("\trow_slice: {:?}", row_slice);
            row_slice
        })
        .map(|c| is_symbol_enhanded(c.to_owned()))
        .filter(|r| r.0)
        .map(|r| r.1)
        .collect::<Vec<char>>();

    if !r.is_empty() {
        return (true, r[0]);
    }
    println!(
        "NOT FOUND. Number: {}, Row: {}, Col: {}",
        number.number, number.row, number.col
    );

    (false, ' ')
}

#[allow(dead_code)]
fn has_symbol_adjacent2(number: &Number, matrix: &Vec<Vec<char>>) -> (bool, char) {
    // calc a bounding box
    let row = number.row as usize;
    let col = number.col as usize;

    let upper_row = std::cmp::max(number.row - 1, 0) as usize;
    let lower_row = std::cmp::min(number.row + 1, matrix.len() as i32 - 1) as usize;

    let left_hand_col = std::cmp::max(number.col - 1, 0) as usize;
    let right_hand_col = std::cmp::min(
        number.col + number.number.len() as i32,
        matrix[0].len() as i32 - 1,
    ) as usize;

    // pretty println
    let mut out = "".to_owned();
    // top
    if let Some(upper) = matrix.get(upper_row) {
        // top left corner
        if let Some(left) = upper.get(left_hand_col) {
            out.push(left.to_owned())
        }
        // upper
        // number
        for i in col..col + number.number.len() {
            out.push(matrix[upper_row][i].to_owned());
        }
        // top right corner
        if let Some(right) = upper.get(right_hand_col) {
            out.push(right.to_owned())
        }
    }

    out.push_str("End of top line\n");

    // middle
    // left
    if let Some(left) = matrix[row].get(left_hand_col) {
        out.push(left.to_owned());
    }
    // number
    for i in col..col + number.number.len() {
        out.push(matrix[row][i].to_owned());
    }
    // right
    if let Some(right) = matrix[row].get(right_hand_col) {
        out.push(right.to_owned())
    }

    out.push_str("End of middle line\n");
    // bottom
    if let Some(bottom) = matrix.get(lower_row) {
        // bottom left corner
        if let Some(left) = bottom.get(left_hand_col) {
            out.push(left.to_owned())
        }
        // bottom
        // number
        for i in col..col + number.number.len() {
            out.push(matrix[lower_row][i].to_owned());
        }
        // bottom right corner
        if let Some(right) = bottom.get(right_hand_col) {
            out.push(right.to_owned())
        }
    }
    out.push_str("End of bottom line\n");
    println!("-------------------------------------------------");
    println!("{}", out);
    println!("-------------------------------------------------");

    for i in col..col + number.number.len() {
        // upper level
        if is_symbol(matrix[upper_row][i]) {
            return (true, matrix[upper_row][i]);
        }
        // lower level
        if is_symbol(matrix[lower_row][i]) {
            return (true, matrix[lower_row][i]);
        }
    }

    // left hand
    if is_symbol(matrix[row][left_hand_col]) {
        return (true, matrix[row][left_hand_col]);
    }

    // right hand
    if is_symbol(matrix[row][right_hand_col]) {
        return (true, matrix[row][right_hand_col]);
    }

    // top left diagonal
    if is_symbol(matrix[upper_row][left_hand_col]) {
        return (true, matrix[upper_row][left_hand_col]);
    }

    // bottom left diagonal
    if is_symbol(matrix[lower_row][left_hand_col]) {
        return (true, matrix[lower_row][left_hand_col]);
    }

    // top right diagonal
    if is_symbol(matrix[upper_row][right_hand_col]) {
        return (true, matrix[upper_row][right_hand_col]);
    }

    // bottom right diagonal
    if is_symbol(matrix[lower_row][right_hand_col]) {
        return (true, matrix[lower_row][right_hand_col]);
    }

    println!(
        "NOT FOUND. Number: {}, Row: {}, Col: {}",
        number.number, number.row, number.col
    );

    (false, ' ')
}

#[allow(dead_code)]
fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}
#[allow(dead_code)]
fn is_symbol_enhanded(c: char) -> (bool, char) {
    (!c.is_ascii_digit() && c != '.', c)
}

#[test]
fn should_load_numbers() {
    {
        let case = "69..172..............................454..46.......507..........809......923.778..................793..............137.............238........11".to_owned();
        println!("\nTesting case: {:?}", case);
        let result = extract_number_in_line(&case.chars().collect::<Vec<char>>(), 0);
        assert_eq!(12, result.len());
        assert_eq!("69", result[0].number);
        assert_eq!("172", result[1].number);
        assert_eq!("454", result[2].number);
        assert_eq!("46", result[3].number);
        assert_eq!("507", result[4].number);
        assert_eq!("809", result[5].number);
        assert_eq!("923", result[6].number);
        assert_eq!("778", result[7].number);
        assert_eq!("793", result[8].number);
        assert_eq!("137", result[9].number);
        assert_eq!("238", result[10].number);
        assert_eq!("11", result[11].number);
    }
    {
        let case = "123".to_owned();
        println!("\nTesting case: {:?}", case);

        let result = extract_number_in_line(&case.chars().collect::<Vec<char>>(), 0);
        assert_eq!(1, result.len());
        assert_eq!("123", result[0].number);
    }
}

#[test]
fn playground() {
    let vector = vec![0, 1, 2, 3, 4, 5];
    println!("Result get(): {:?}", vector.get(0..8));
    assert_eq!(Some(&[0, 1, 2][..]), vector.get(0..8));
}

#[test]
fn should_check_adjacents() {
    let matrix: Vec<Vec<char>> = vec![
        //    0    1    2    3    4    5    6    7    8    9
        vec!['1', '2', '3', '.', '.', '.', '.', '.', '8', '8'], // 0
        vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'], // 1
        vec!['.', '.', '.', '.', '.', '.', '5', '.', '.', '.'], // 2
        vec!['.', '$', '4', '5', '7', '.', '%', '.', '.', '4'], // 3
        vec!['.', '.', '.', '.', '.', '.', '%', '.', '.', '.'], // 4
        vec!['&', '2', '.', '.', '5', '5', '%', '.', '.', '.'], // 5
        vec!['.', '3', '.', '.', '.', '.', '%', '.', '6', '7'], // 6
        vec!['.', '.', '4', '.', '.', '.', '%', '.', '.', '$'], // 7
    ];

    assert_eq!(
        (true, '#'),
        has_symbol_adjacent(&Number::new(0, 0, "123".to_string()), &matrix)
    );
    assert_eq!(
        (false, ' '),
        has_symbol_adjacent(&Number::new(0, 8, "88".to_string()), &matrix)
    );

    assert_eq!(
        (true, '%'),
        has_symbol_adjacent(&Number::new(2, 7, "5".to_string()), &matrix)
    );
    assert_eq!(
        (true, '$'),
        has_symbol_adjacent(&Number::new(3, 2, "457".to_string()), &matrix)
    );

    assert_eq!(
        (true, '&'),
        has_symbol_adjacent(&Number::new(5, 1, "2".to_string()), &matrix)
    );

    assert_eq!(
        (true, '&'),
        has_symbol_adjacent(&Number::new(6, 1, "3".to_string()), &matrix)
    );

    assert_eq!(
        (false, ' '),
        has_symbol_adjacent(&Number::new(7, 2, "4".to_string()), &matrix)
    );

    assert_eq!(
        (true, '%'),
        has_symbol_adjacent(&Number::new(5, 4, "55".to_string()), &matrix)
    );
    assert_eq!(
        (false, ' '),
        has_symbol_adjacent(&Number::new(3, 9, "4".to_string()), &matrix)
    );
    assert_eq!(
        (true, '$'),
        has_symbol_adjacent(&Number::new(6, 8, "6".to_string()), &matrix)
    );
    assert_eq!(
        (true, '$'),
        has_symbol_adjacent(&Number::new(6, 9, "7".to_string()), &matrix)
    );
}
