use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(PartialEq, PartialOrd, Debug, Ord, Eq, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    hand: HandType,
    value: u32,
    letters: String,
}

impl Hand {
    fn new(hand: HandType, value: u32, letters: String) -> Self {
        Self {
            hand,
            value,
            letters,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(clippy::all)]
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand == other.hand {
            for (selfc, otherc) in self.letters.chars().zip(other.letters.chars()) {
                if selfc != otherc {
                    return Some(get_char_value(selfc).cmp(&get_char_value(otherc)));
                }
            }

            return Some(Ordering::Equal);
        }
        Some(self.hand.cmp(&other.hand))
    }
}

fn get_char_value(c: char) -> u32 {
    let mut cards = CARDS;
    cards.reverse();
    let value = cards.iter().position(|i| *i == c).unwrap() as u32;
    println!("Value of char: {:?}: {:?}", c, value);
    value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut list: Vec<Hand> = Vec::new();
        lines
            .flatten()
            .enumerate()
            .filter(|(_, l)| !l.is_empty())
            .for_each(|(_i, line)| {
                let parts: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                let value = parts[1].parse::<u32>().unwrap();
                let letters = parts[0].to_owned();
                list.push(build_hand(&letters, value));
            });

        list.sort();
        let total: u32 = list
            .iter()
            .enumerate()
            .map(|(i, hand)| {
                println!("Hand Rank {}: {:?}", i + 1, hand);
                (i as u32 + 1) * hand.value
            })
            .sum();
        println!("Total: {}", total);
    }
}

fn build_hand(letters: &str, value: u32) -> Hand {
    let type_hand = calc_type(letters);

    // simple hand
    if !letters.contains('J') {
        return Hand::new(type_hand, value, letters.to_string());
    }

    // hand with one or more Js
    let mut combis: Vec<String> = Vec::new();
    shuffle(0, letters, "", &mut combis);
    let max = combis
        .iter()
        .map(|combi| (combi, calc_type(combi)))
        .max_by(|x, y| x.1.cmp(&y.1));
    Hand::new(max.unwrap().1, value, letters.to_string())
}

fn shuffle(i: usize, og: &str, so_far: &str, list: &mut Vec<String>) {
    if let Some(c) = og.chars().nth(i) {
        if c == 'J' {
            for card in CARDS {
                let mut current = String::from(so_far);
                current.push(card);
                shuffle(i + 1, og, &current, list);
            }
        } else {
            let mut new_so_far = so_far.to_string();
            new_so_far.push(c);
            shuffle(i + 1, og, &new_so_far, list);
        }
    } else {
        // base case, reached end of string and store whats build so far
        list.push(so_far.to_string());
    }
}

fn calc_type(letters: &str) -> HandType {
    let mut frequencies: HashMap<char, i32> = HashMap::new();
    letters.chars().for_each(|c| {
        *frequencies.entry(c).or_insert(0) += 1;
    });
    match (frequencies.values().max().unwrap(), frequencies.len()) {
        (1, _) => HandType::HighCard,
        (2, 4) => HandType::OnePair,
        (2, 3) => HandType::TwoPairs,
        (3, len) => match len {
            3 => HandType::Three,
            2 => HandType::Full,
            _ => panic!(),
        },
        (4, _) => HandType::Four,
        (5, _) => HandType::Five,
        _ => panic!("Oops"),
    }
}

#[test]
fn test_ordering_hands() {
    let weaker = Hand::new(HandType::Full, 20, "T55J5".to_owned());
    let stronger = Hand::new(HandType::Full, 20, "QQQJA".to_owned());
    let lowest = Hand::new(HandType::OnePair, 20, "QQQJA".to_owned());

    let mut list = vec![weaker.clone(), stronger.clone(), lowest.clone()];
    list.sort();
    assert_eq!(Some(Ordering::Less), weaker.partial_cmp(&stronger));
    assert_eq!(
        list,
        vec![stronger, weaker, lowest]
            .into_iter()
            .rev()
            .collect::<Vec<Hand>>()
    );
}

#[test]
fn test_build_with_combinations() {
    {
        let mut combinations: Vec<String> = Vec::new();
        let lettes = "J1234";
        shuffle(0, lettes, "", &mut combinations);
        assert_eq!(13, combinations.len());
    }

    {
        let mut combinations: Vec<String> = Vec::new();
        let lettes = "J123J";
        shuffle(0, lettes, "", &mut combinations);
        assert_eq!(u64::pow(13, 2) as usize, combinations.len());
    }
    {
        let mut combinations: Vec<String> = Vec::new();
        let lettes = "JJJJJ";
        shuffle(0, lettes, "", &mut combinations);
        assert_eq!(u64::pow(13, 5) as usize, combinations.len());
    }
}

#[test]
fn test_enum() {
    assert!(HandType::Five > HandType::Four);
    assert!(HandType::Four > HandType::Full);
    assert!(HandType::Full > HandType::Three);
    assert!(HandType::Three > HandType::TwoPairs);
    assert!(HandType::TwoPairs > HandType::OnePair);
    assert!(HandType::OnePair > HandType::HighCard);
}
