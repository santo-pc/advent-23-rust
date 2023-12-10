use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    thread::current,
};
const CARDS: Vec<char> = vec![
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

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
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

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
        )
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
    if let Ok(lines) = read_lines("input.txt") {
        let mut list: Vec<Hand> = Vec::new();
        lines
            .flatten()
            .enumerate()
            .filter(|(_, l)| !l.is_empty())
            .for_each(|(_i, line)| {
                let parts: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                println!("line {:?}", line);

                list.push(build_hand(parts));
            });

        list.sort();
        let total: u32 = list
            .iter()
            .enumerate()
            .map(|(i, hand)| {
                println!("Hand {}: {:?}", i, hand);
                (i as u32 + 1) * hand.value
            })
            .sum();
        println!("Total: {}", total);
    }
}
fn build_hand(parts: Vec<&str>) -> Hand {
    let value = parts[1].parse::<u32>().unwrap();
    let letters = parts[0].to_owned();
    let type_hand = calc_type(&letters);
    let hand = Hand::new(type_hand, value, letters);

    let mut generated_hands = HashMap::new();
    generated_hands.insert(hand.letters.clone(), hand.hand.to_owned());

    if hand.letters.contains('J') {
        let jays = hand
            .letters
            .chars()
            .filter(|c| *c == 'J')
            .collect::<Vec<char>>()
            .len();
        let matrix: Vec<String> = Vec::new();
        for (i, l) in letters.chars().enumerate() {
            if l == 'J' {
                for card in CARDS {
                    shuffle()
                }
            }
        }
    }

    hand
}

fn shuffle(st: &str) -> Vec<String> {
    let result: Vec<String> = Vec::new();
    if st.is_empty() {
        return Vec::new();
    }

    for (i, s) in st.chars().enumerate() {
        if s == 'J' {
            for card in CARDS {
                let current = card.to_string().push_str(&st[i..]);

                for combi in shuffle(current[i + 1].to_owned()) {
                    result.push(combi);
                }
            }
        } else {
            result.push(st.to_owned());
        }
    }
    result
}

fn calc_type(letters: &str) -> HandType {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    letters.chars().for_each(|c| {
        *letter_counts.entry(c).or_insert(0) += 1;
    });
    match (letter_counts.values().max().unwrap(), letter_counts.len()) {
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

fn calc_value(frequencies: &HashMap<char, i32>) -> u32 {
    frequencies.keys().map(|k| get_char_value(*k)).sum()
}

fn get_char_value(c: char) -> u32 {
    let cards = CARDS.clone();
    cards.reverse();
    let value = cards.iter().position(|i| *i == c).unwrap() as u32;
    println!("Value of char: {:?}: {:?}", c, value);
    value
}

#[test]
fn test_template() {
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
fn test_enum() {
    assert!(HandType::Five > HandType::Four);
    assert!(HandType::Four > HandType::Full);
    assert!(HandType::Full > HandType::Three);
    assert!(HandType::Three > HandType::TwoPairs);
    assert!(HandType::TwoPairs > HandType::OnePair);
    assert!(HandType::OnePair > HandType::HighCard);
}
