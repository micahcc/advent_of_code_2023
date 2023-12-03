/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114
(top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part
number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers
in the engine schematic?

--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and
35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490.
(The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all
of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

*/

static INPUT0: &'static str = include_str!("input0.txt");
static INPUT1: &'static str = include_str!("input1.txt");

fn part1(input: &str) {
    let mut array: Vec<Vec<char>> = vec![];
    let mut is_conn: Vec<Vec<bool>> = vec![];
    // first create a map of every point adjacent to a symbol
    for line in input.split_whitespace() {
        let line: Vec<char> = line.trim().chars().collect();
        if !line.is_empty() {
            is_conn.push(vec![false; line.len()]);
            array.push(line);
        }
    }

    let neighbors: [(i8, i8); 9] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for r in 0..array.len() {
        for c in 0..array[r].len() {
            for n in neighbors {
                let r_off: i64 = r as i64 + n.0 as i64;
                let c_off: i64 = c as i64 + n.1 as i64;
                if r_off < 0 || r_off >= array.len() as i64 {
                    continue;
                }
                if c_off < 0 || c_off >= array[r_off as usize].len() as i64 {
                    continue;
                }
                let ch = array[r_off as usize][c_off as usize];
                is_conn[r][c] = is_conn[r][c] || (!ch.is_digit(10) && ch != '.');
            }
        }
    }

    //println!("{:?}", is_conn);
    let mut used_nums = vec![];
    for (row, line) in input.split_whitespace().enumerate() {
        let mut line = line.to_string();
        line.push('.');
        let mut curr_num: Vec<char> = vec![];
        let mut was_conn = false;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                curr_num.push(ch);
                was_conn = was_conn || is_conn[row][col];
            } else if curr_num.len() > 0 {
                if was_conn {
                    // println!("Ending {:?}", curr_num);
                    let n: i64 = curr_num
                        .iter()
                        .collect::<String>()
                        .to_string()
                        .parse()
                        .unwrap();
                    used_nums.push(n);
                }
                curr_num.clear();
                was_conn = false;
            }
        }
    }

    //println!("Nums: {:?}", used_nums);
    println!("Sum: {}", used_nums.iter().sum::<i64>());
}

fn part2(input: &str) {
    let mut array: Vec<Vec<char>> = vec![];
    let mut gear_pos: Vec<Vec<Option<(usize, usize)>>> = vec![];
    // find the gear position for every number
    for line in input.split_whitespace() {
        let line: Vec<char> = line.trim().chars().collect();
        if !line.is_empty() {
            gear_pos.push(vec![None; line.len()]);
            array.push(line);
        }
    }

    let neighbors: [(i8, i8); 9] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for r in 0..array.len() {
        for c in 0..array[r].len() {
            for n in neighbors {
                let r_off: i64 = r as i64 + n.0 as i64;
                let c_off: i64 = c as i64 + n.1 as i64;
                if r_off < 0 || r_off >= array.len() as i64 {
                    continue;
                }
                if c_off < 0 || c_off >= array[r_off as usize].len() as i64 {
                    continue;
                }
                let ch = array[r_off as usize][c_off as usize];
                if array[r][c].is_digit(10) && ch == '*' {
                    gear_pos[r][c] = Some((r_off as usize, c_off as usize));
                }
            }
        }
    }

    let mut gear_pairs: std::collections::HashMap<(usize, usize), Vec<i64>> = Default::default();
    for (row, line) in input.split_whitespace().enumerate() {
        // force the line to end with '.' so we don't have to do special
        // parsing for it
        let mut line = line.to_string();
        line.push('.');

        let mut curr_num: Vec<char> = vec![];
        let mut gear = None;
        for (col, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                curr_num.push(ch);
                // set the gear
                let had_gear = gear.is_some();
                if !had_gear {
                    gear = gear_pos[row][col];
                }
            } else if curr_num.len() > 0 {
                if gear.is_some() {
                    //println!("Ending {:?}", curr_num);
                    let n: i64 = curr_num
                        .iter()
                        .collect::<String>()
                        .to_string()
                        .parse()
                        .unwrap();
                    let gear_pos = gear.unwrap();
                    gear_pairs
                        .entry(gear_pos)
                        .and_modify(|v| v.push(n))
                        .or_insert(vec![n; 1]);
                }

                curr_num.clear();
                gear = None;
            }
        }
    }

    // combine gear pairs

    //println!("Gears: {:?}", gear_pairs);
    let mut sum = 0;
    for nums in gear_pairs.values() {
        if nums.len() < 2 {
            continue;
        } else if nums.len() == 2 {
            let ratio = nums[0] * nums[1];
            sum += ratio;
        } else {
            println!("wtf {:?}", nums);
        }
    }

    println!("{}", sum);
    //println!("Sum: {}", used_nums.iter().sum::<i64>());
}

fn main() {
    part1(INPUT0);
    part1(INPUT1);
    part2(INPUT0);
    part2(INPUT1);
}
