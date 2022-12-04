use core::panic;

use advent22::get_input_contents;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn decode_hand(code: &str) -> Hand {
    match code {
        "A" => return Hand::Rock,
        "B" => return Hand::Paper,
        "C" => return Hand::Scissors,
        "X" => return Hand::Rock,
        "Y" => return Hand::Paper,
        "Z" => return Hand::Scissors,
        _ => panic!("Invalid Hand"),
    }
}

fn calculate_points(my_hand: &Hand, opponent_hand: &Hand) -> i32 {
    let shape_points: i32 = match my_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    let outcome_points: i32 = match my_hand {
        Hand::Rock => match opponent_hand {
            Hand::Scissors => 6,
            Hand::Rock => 3,
            Hand::Paper => 0,
        },
        Hand::Paper => match opponent_hand {
            Hand::Rock => 6,
            Hand::Paper => 3,
            Hand::Scissors => 0,
        },
        _ => match opponent_hand {
            Hand::Paper => 6,
            Hand::Scissors => 3,
            Hand::Rock => 0,
        },
    };

    return outcome_points + shape_points;
}

fn decode_corrected_hand(code: &str, opponent_hand: &Hand) -> Hand {
    match code {
        "X" => match opponent_hand {
            Hand::Rock => return Hand::Scissors,
            Hand::Paper => return Hand::Rock,
            Hand::Scissors => return Hand::Paper,
        },
        "Y" => match opponent_hand {
            Hand::Rock => return Hand::Rock,
            Hand::Paper => return Hand::Paper,
            Hand::Scissors => return Hand::Scissors,
        },
        "Z" => match opponent_hand {
            Hand::Rock => return Hand::Paper,
            Hand::Paper => return Hand::Scissors,
            Hand::Scissors => return Hand::Rock,
        },
        _ => panic!("This is invalid"),
    };
}

fn main() {
    let file_contents = get_input_contents();

    let mut total_score: i32 = 0;
    let mut corrected_total_score: i32 = 0;

    for line in file_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut line_contents = line.split(" ");
        let opponent_code = line_contents.next().expect("failed to parse opponent code");
        let my_code = line_contents.next().expect("failed to parse my code");

        let opponent_hand = decode_hand(opponent_code);
        let my_hand = decode_hand(my_code);

        total_score += calculate_points(&my_hand, &opponent_hand);

        let my_corrected_hand: Hand = decode_corrected_hand(my_code, &opponent_hand);
        corrected_total_score += calculate_points(&my_corrected_hand, &opponent_hand);
    }

    println!("Total Score: {}", total_score);
    println!("Corrected Total Score: {}", corrected_total_score);
}
