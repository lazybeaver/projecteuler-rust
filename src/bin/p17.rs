extern crate euler;

#[cfg_attr(rustfmt, rustfmt_skip)]
const SMALL_NAMES: [&'static str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const TENS_NAMES: [&'static str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn word_small(num: usize) -> String {
    match num {
        0...19 => format!("{}", SMALL_NAMES[num]),
        _ => String::from("?"),
    }
}

fn word_tens(num: usize) -> String {
    let (tens, ones) = (num / 10, num % 10);
    let tens_name = TENS_NAMES[tens - 2];
    match ones {
        0 => format!("{}", tens_name),
        1...9 => format!("{}-{}", tens_name, word_small(ones)),
        _ => String::from("?"),
    }
}

fn word_hundreds(num: usize) -> String {
    let (hundreds, ones_tens) = (num / 100, num % 100);
    match ones_tens {
        0 => format!("{} hundred", word_small(hundreds)),
        1...19 => format!(
            "{} hundred and {}",
            word_small(hundreds),
            word_small(ones_tens)
        ),
        20...99 => format!(
            "{} hundred and {}",
            word_small(hundreds),
            word_tens(ones_tens)
        ),
        _ => String::from("?"),
    }
}

fn num_to_word(num: usize) -> String {
    match num {
        0...19 => word_small(num),
        20...99 => word_tens(num),
        100...999 => word_hundreds(num),
        1000 => String::from("one thousand"),
        _ => String::from("?"),
    }
}

fn count_letters(start: usize, end: usize) -> usize {
    (start..end + 1)
        .map(|n| num_to_word(n))
        .map(|w| w.chars().filter(|c| c.is_alphabetic()).count())
        .sum()
}

fn main() {
    let result = count_letters(1, 1000);
    println!("Total letters used in numbers from [1-1000]: {:?}", result);
}

#[test]
fn test_num_to_word() {
    assert_eq!("zero", num_to_word(0));
    assert_eq!("one", num_to_word(1));
    assert_eq!("seven", num_to_word(7));
    assert_eq!("thirteen", num_to_word(13));
    assert_eq!("one hundred", num_to_word(100));
    assert_eq!("one hundred and eleven", num_to_word(111));
    assert_eq!("five hundred and twenty-five", num_to_word(525));
    assert_eq!("nine hundred and ninety-nine", num_to_word(999));
    assert_eq!("one thousand", num_to_word(1000));
}

#[test]
fn test_all_num_to_word() {
    for num in 0..1001 {
        let word = num_to_word(num);
        assert_ne!("", word);
        assert!(!word.contains("?"));
    }
}

#[test]
fn test_count_letters() {
    assert_eq!(19, count_letters(1, 5));
}
