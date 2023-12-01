/*
--- Day 1: Trebuchet?! ---
Something is wrong with global snow production, and you've been selected to take a look. The Elves
have even given you a map; on it, they've used stars to mark the top fifty locations that are
likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all
fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're
even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of
questions") and hang on did you just say the sky ("of course, where do you think snow comes from")
when you realize that the Elves are already loading you into a trebuchet ("please hold still, we
need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle
input) has been amended by a very young Elf who was apparently just excited to show off her art
skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a
specific calibration value that the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in that order) to form a single
two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these
together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

*/
static INPUT1: &'static str = include_str!("input1.txt");
static INPUT2: &'static str = include_str!("input2.txt");
static INPUT3: &'static str = include_str!("input3.txt");

static DIGITS: [(&str, u8); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part1() {
    let mut total = 0;
    for row in INPUT2.split_whitespace() {
        let digits: Vec<u32> = row
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() - '0'.to_digit(10).unwrap())
            .collect();

        if digits.len() == 0 {
            continue;
        }

        total += 10 * digits[0] + digits[digits.len() - 1];
    }
    println!("Total: {}", total);
}

fn part2() {
    // part 2
    let mut total = 0;
    for row in INPUT2.split_whitespace() {
        let mut digits: Vec<u32> = vec![];
        for i in 0..row.len() {
            for (dstr, dval) in DIGITS {
                if row[i..].starts_with(dstr) {
                    digits.push(dval as u32);
                }
            }
        }
        //println!("digits: {:?}", digits);
        if digits.len() == 0 {
            continue;
        }

        total += 10 * digits[0] + digits[digits.len() - 1];
    }
    println!("Total: {}", total);
}

fn main() {
    part1();
    part2();
}
