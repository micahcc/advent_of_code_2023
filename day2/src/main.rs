/*
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with
that configuration. However, game 3 would have been impossible because at one point the Elf showed
you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed
you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you
get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

*/

static INPUT0: &'static str = include_str!("input0.txt");
static INPUT1: &'static str = include_str!("input1.txt");

fn split_2(total: u8) -> Vec<(u8, u8)> {
    let mut out: Vec<(u8, u8)> = vec![];
    let mut buckets = (0, total);
    loop {
        out.push((buckets.0 as u8, buckets.1 as u8));
        if buckets.1 == 0 {
            break;
        }
        buckets.0 += 1;
        buckets.1 -= 1;
    }
    return out;
}

fn split_3(total: u8) -> Vec<(u8, u8, u8)> {
    // split into ((bucket0, bucket1), bucket2)
    let mut out: Vec<(u8, u8, u8)> = vec![];
    for (bucket01, bucket2) in split_2(total) {
        for (bucket0, bucket1) in split_2(bucket01) {
            out.push((bucket0, bucket1, bucket2))
        }
    }
    return out;
}

fn split_4(total: u8) -> Vec<(u8, u8, u8, u8)> {
    // split into ((bucket0, bucket1, bucket2), bucket3)
    let mut out = vec![];
    for (bucket012, bucket3) in split_2(total) {
        for (bucket0, bucket1, bucket2) in split_3(bucket012) {
            out.push((bucket0, bucket1, bucket2, bucket3))
        }
    }
    return out;
}

fn is_valid(red: &(u8, u8, u8, u8), green: &(u8, u8, u8, u8), blue: &(u8, u8, u8, u8)) -> bool {
    // 0 = in the bag
    // 1 = draw 1, 2 = draw 2, 3 = draw 3
    if red.1 + green.1 + blue.1 == 0 {
        // draw 1 had nothing
        return false;
    }
    if red.2 + green.2 + blue.2 == 0 {
        // draw 2 had nothing
        return false;
    }
    if red.3 + green.3 + blue.3 == 0 {
        // draw 3 had nothing
        return false;
    }
    // every draw had at least 1 cube
    return true;
}

#[derive(Debug)]
struct Round {
    n_red: u8,
    n_blue: u8,
    n_green: u8,
}

impl Round {
    fn from_str(data: &str) -> Round {
        let mut out = Round {
            n_red: 0,
            n_blue: 0,
            n_green: 0,
        };

        //println!("In: {}", data);
        for color_count in data.split(',') {
            let spl: Vec<String> = color_count
                .split_whitespace()
                .map(|x| x.trim().to_string())
                .collect();

            //println!("color_count: {}, spl: {:?}", color_count, spl);
            match spl[1].as_str() {
                "blue" => out.n_blue = spl[0].parse().unwrap(),
                "green" => out.n_green = spl[0].parse().unwrap(),
                "red" => out.n_red = spl[0].parse().unwrap(),
                _ => {
                    panic!("Uknown token: {}", spl[1]);
                }
            }
        }

        return out;
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let line = line.trim();
        //println!("{:?}", line);
        // Game X, Rest
        let spl: Vec<String> = line.split(':').map(|x| x.to_string()).collect();
        //println!("{:?}", spl);

        // Game, Num
        let ids: Vec<String> = spl[0].split(' ').map(|x| x.to_string()).collect();
        //println!("{:?}", ids);
        let id: u64 = ids[1].parse().unwrap();

        let rounds: Vec<Round> = spl[1]
            .split(';')
            .map(|x| {
                let x = x.trim();
                //println!("In: {:?}", x);
                let out = Round::from_str(x);
                //println!("out: {:?}", out);
                return out;
            })
            .collect();
        return Game { id, rounds };
    }

    fn compute_power(&self) -> u64 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in self.rounds.iter() {
            min_red = u64::max(min_red, round.n_red as u64);
            min_green = u64::max(min_green, round.n_green as u64);
            min_blue = u64::max(min_blue, round.n_blue as u64);
        }
        return min_red * min_green * min_blue;
    }

    fn is_feasible(&self) -> bool {
        for round in self.rounds.iter() {
            //println!("Round: {:?}", round);
            if round.n_red > 12 || round.n_blue > 14 || round.n_green > 13 {
                return false;
            }
        }
        return true;
    }
}

fn part0() {
    let mut sum = 0;
    for row in INPUT0.split('\n') {
        let row = row.trim();
        if row.len() > 0 {
            //println!("{}", row);
            let game = Game::from_line(row);
            if game.is_feasible() {
                println!("Feasible: {:?}", game);
                sum += game.id;
            } else {
                println!("Not Feasible: {:?}", game);
            }
        }
    }

    println!("Sum: 0 {}", sum);
}

fn part1() {
    let mut sum = 0;
    for row in INPUT1.split('\n') {
        let row = row.trim();
        if row.len() > 0 {
            //println!("{}", row);
            let game = Game::from_line(row);
            //println!("{:?}", game);
            if game.is_feasible() {
                sum += game.id;
            }
        }
    }

    println!("Sum: 0 {}", sum);
}

fn part2() {
    let mut sum = 0;
    for row in INPUT1.split('\n') {
        let row = row.trim();
        if row.len() > 0 {
            let game = Game::from_line(row);
            //println!("{:?}, power: {}", game, game.compute_power());
            sum += game.compute_power();
        }
    }

    println!("Sum: 2 {}", sum);
}

fn main() {
    part0();
    part1();
    part2();

    //let red_combos = split_4(12);
    //let green_combos = split_4(13);
    //let blue_combos = split_4(14);
    ////let red_combos = split_4(2);
    ////let green_combos = split_4(3);
    ////let blue_combos = split_4(4);

    //// now iterate through each of these to get all possible cominations, filtering
    //// invalid ones
    //let mut variations = 0;
    //for red_sel in red_combos.iter() {
    //    for green_sel in green_combos.iter() {
    //        for blue_sel in blue_combos.iter() {
    //            // must draw at least one cube for every group
    //            if is_valid(red_sel, green_sel, blue_sel) {
    //                variations += 1;
    //                sum += variations;
    //                //println!(
    //                //    "Game {}: {} red, {} green, {} blue; {} red, \
    //                //    {} green, {} blue; {} red, {} green, {} blue",
    //                //    variations,
    //                //    red_sel.1,
    //                //    green_sel.1,
    //                //    blue_sel.1,
    //                //    red_sel.2,
    //                //    green_sel.2,
    //                //    blue_sel.2,
    //                //    red_sel.3,
    //                //    green_sel.3,
    //                //    blue_sel.3,
    //                //);
    //            }
    //        }
    //    }
    //}

    //println!("Variations: {}", variations);
    //println!("Sum: {}", sum);
}
