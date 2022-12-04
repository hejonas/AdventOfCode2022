use std::io::{self, Read};

fn main() {
    let mut input_part1 = String::new();
    io::stdin().read_to_string(&mut input_part1).ok();
    
    let input_part2 = input_part1.clone();

    // Part 1
    let mut tot_score = 0;

    for line in input_part1.lines() {
        let mut game_input = line.split_ascii_whitespace();
        let oponent_input = game_input.next();
        let my_input = game_input.next(); 

        match my_input
        {
            // My input is Rock
            Some("X") => {
                tot_score = tot_score + 1;
                match oponent_input
                {
                    Some("A") => {tot_score = tot_score + 3;} // Draw
                    Some("B") => {} // Loss
                    Some("C") => {tot_score = tot_score + 6;} // Win
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            // My input is Paper
            Some("Y") => {
                tot_score = tot_score + 2;
                match oponent_input
                {
                    Some("A") => {tot_score = tot_score + 6;} // Win
                    Some("B") => {tot_score = tot_score + 3;} // Draw
                    Some("C") => {} // Loss
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            // My input is Scissors
            Some("Z") => {
                tot_score = tot_score + 3;
                match oponent_input
                {
                    Some("A") => {} // Loss
                    Some("B") => {tot_score = tot_score + 6;} // Win
                    Some("C") => {tot_score = tot_score + 3;} // Draw
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            None => {println!("Something went terribly wrong!");}
            _ => {println!("Something went terribly wrong!");}
        }
    }

    println!("The total score for part 1 was {}", tot_score);

    // Part 2
    tot_score = 0;

    for line in input_part2.lines() {
        let mut game_input = line.split_ascii_whitespace();
        let oponent_input = game_input.next();
        let outcome = game_input.next(); 

        match outcome
        {
            // Outcome should be loss
            Some("X") => {
                tot_score = tot_score + 0;
                match oponent_input
                {
                    Some("A") => {tot_score = tot_score + 3;} // Openent chooses rock, I choose scissors
                    Some("B") => {tot_score = tot_score + 1;} // Openent chooses paper, I choose rock
                    Some("C") => {tot_score = tot_score + 2;} // Openent chooses scissors, I choose paper
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            // Outcome should be draw
            Some("Y") => {
                tot_score = tot_score + 3;
                match oponent_input
                {
                    Some("A") => {tot_score = tot_score + 1;} // Openent chooses rock, I choose rock
                    Some("B") => {tot_score = tot_score + 2;} // Openent chooses paper, I choose paper
                    Some("C") => {tot_score = tot_score + 3;} // Openent chooses scissor, I choose scissors
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            // Outcome should be win
            Some("Z") => {
                tot_score = tot_score + 6;
                match oponent_input
                {
                    Some("A") => {tot_score = tot_score + 2;} // Openent chooses rock, I choose paper
                    Some("B") => {tot_score = tot_score + 3;} // Openent chooses paper, I choose scissors
                    Some("C") => {tot_score = tot_score + 1;} // Openent chooses scissor, I choose rock
                    None => {println!("Something went terribly wrong!");}
                    _ => {println!("Something went terribly wrong!");}
                }
            }
            None => {println!("Something went terribly wrong!");}
            _ => {println!("Something went terribly wrong!");}
        }
    }

    println!("The total score for part 2 was {}", tot_score);
}
