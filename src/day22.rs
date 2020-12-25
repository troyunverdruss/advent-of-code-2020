use crate::day22::Winner::{Player1, Player2};
use crate::util::inputs::read_lines_split_by_double_newline;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn run() {
    let sections = read_lines_split_by_double_newline(22);
    let (player1_cards, player2_cards) = get_deals(&sections);

    let game_result = play_game(&player1_cards, &player2_cards, false);

    let winner_value = compute_deck_value(&game_result);
    println!("Part 1: winner: {:?}, hand value: {}", game_result.winner, winner_value);

    let game_result = play_game(&player1_cards, &player2_cards, true);

    let winner_value = compute_deck_value(&game_result);
    println!("Part 1: winner: {:?}, hand value: {}", game_result.winner, winner_value);
}

fn compute_hash_from_hands(player1_cards: &[i64], player2_cards: &[i64]) -> String {
    let player1 = player1_cards.iter().map(|v| format!("{}", v)).join(",");
    let player2 = player2_cards.iter().map(|v| format!("{}", v)).join(",");

    format!("{}-{}", player1, player2)
}

fn play_game(player1_cards: &[i64], player2_cards: &[i64], recurse: bool) -> GameResult {
    // To handle infinite recursion checks
    let mut seen_states = HashSet::new();

    let mut player1_cards = Vec::from(player1_cards);
    let mut player2_cards = Vec::from(player2_cards);
    seen_states.insert(compute_hash_from_hands(&player1_cards, &player2_cards));

    let mut first = true;
    loop {
        // Handle the infinite recursion case up front
        // Protecting against a "match" on the first loop
        if !first && seen_states.contains(&compute_hash_from_hands(&player1_cards, &player2_cards))
        {
            let i = 0;
        };

        if !first && seen_states.contains(&compute_hash_from_hands(&player1_cards, &player2_cards))
        {
            return GameResult {
                winner: Player1,
                player_1_ending_deck: player1_cards.clone(),
                player_2_ending_deck: player2_cards.clone(),
            };
        }
        first = false;
        seen_states.insert(compute_hash_from_hands(&player1_cards, &player2_cards));

        // Now let's get our values (this block bails if the game is over)
        let (card1, card2) = {
            let card1 = player1_cards.first();
            let card2 = player2_cards.first();

            // If a deck is depleted, then we're done
            if card1.is_none() {
                return GameResult {
                    winner: Player2,
                    player_1_ending_deck: player1_cards.clone(),
                    player_2_ending_deck: player2_cards.clone(),
                };
            }
            if card2.is_none() {
                return GameResult {
                    winner: Player1,
                    player_1_ending_deck: player1_cards.clone(),
                    player_2_ending_deck: player2_cards.clone(),
                };
            }
            (*card1.unwrap(), *card2.unwrap())
        };

        // Now compare and assign cards to decks
        player1_cards.remove(0);
        player2_cards.remove(0);

        // println!("  hands: {:?}, {:?}", player1_cards, player2_cards);
        // println!("  cards: {:?}, {:?}", card1, card2);

        if recurse && player1_cards.len() as i64 >= card1 && player2_cards.len() as i64 >= card2 {
            let game_result = play_game(
                &player1_cards[0..card1 as usize],
                &player2_cards[0..card2 as usize],
                true,
            );
            if game_result.winner == Player1 {
                player1_cards.push(card1);
                player1_cards.push(card2);
            } else {
                player2_cards.push(card2);
                player2_cards.push(card1);
            }
        } else if card1 > card2 {
            player1_cards.push(card1);
            player1_cards.push(card2);
        } else {
            player2_cards.push(card2);
            player2_cards.push(card1);
        }
    }
}

struct GameResult {
    winner: Winner,
    player_1_ending_deck: Vec<i64>,
    player_2_ending_deck: Vec<i64>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Winner {
    Player1,
    Player2,
}

fn get_deals(sections: &[String]) -> (Vec<i64>, Vec<i64>) {
    let player1 = sections
        .get(0)
        .unwrap()
        .split('\n')
        .filter(|e| !e.starts_with("Player"))
        .filter(|e| !e.is_empty())
        .map(|v| String::from(v).parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let player2 = sections
        .get(1)
        .unwrap()
        .split('\n')
        .filter(|e| !e.starts_with("Player"))
        .filter(|e| !e.is_empty())
        .map(|v| String::from(v).parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (player1, player2)
}

fn compute_deck_value(game_result: &GameResult) -> i64 {
    let mut value = 0;
    let mut multiplier = 1;

    let deck = if game_result.winner == Player1 {
        &game_result.player_1_ending_deck
    } else {
        &game_result.player_2_ending_deck
    };

    for card in deck.iter().rev() {
        value += card * multiplier;
        multiplier += 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use crate::day22::play_game;
    use crate::day22::Winner::Player1;
    use crate::day22::{compute_deck_value, get_deals};

    #[test]
    fn example1() {
        let sections = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
        let (mut player1_cards, mut player2_cards) = get_deals(&sections);
        let game_result = play_game(&mut player1_cards, &mut player2_cards, false);
        assert_eq!(306, compute_deck_value(&game_result));
    }

    #[test]
    fn example2_infinite() {
        let sections = "\
Player 1:
43
19

Player 2:
2
29
14"
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
        let (mut player1_cards, mut player2_cards) = get_deals(&sections);
        let game_result = play_game(&mut player1_cards, &mut player2_cards, false);
        assert_eq!(Player1, game_result.winner);
    }

    #[test]
    fn example2_full_game() {
        let sections = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
        let (mut player1_cards, mut player2_cards) = get_deals(&sections);
        let game_result = play_game(&mut player1_cards, &mut player2_cards, true);
        assert_eq!(291, compute_deck_value(&game_result));
    }
}
