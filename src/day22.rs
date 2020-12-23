use crate::util::inputs::read_lines_split_by_double_newline;
use std::collections::VecDeque;

pub fn run() {
    let sections = read_lines_split_by_double_newline(22);
    let (mut player1_cards, mut player2_cards) = get_deals(&sections);

    play_rounds(&mut player1_cards, &mut player2_cards);

    let winner_value = compute_deck_value(&player1_cards) + compute_deck_value(&player2_cards);
    println!("Part 1: {}", winner_value);
}

fn play_rounds(player1_cards: &mut VecDeque<i64>, player2_cards: &mut VecDeque<i64>) {
    loop {
        let card1 = player1_cards.pop_front();
        let card2 = player2_cards.pop_front();

        // If a deck is depleted, then we're done
        if card1.is_none() {
            player2_cards.push_front(card2.unwrap());
            break;
        }
        if card2.is_none() {
            player1_cards.push_front(card1.unwrap());
            break;
        }

        let card1 = card1.unwrap();
        let card2 = card2.unwrap();

        if card1 > card2 {
            player1_cards.push_back(card1);
            player1_cards.push_back(card2);
        } else {
            player2_cards.push_back(card2);
            player2_cards.push_back(card1);
        }
    }
}

fn get_deals(sections: &[String]) -> (VecDeque<i64>, VecDeque<i64>) {
    let player1 = sections
        .get(0)
        .unwrap()
        .split('\n')
        .filter(|e| !e.starts_with("Player"))
        .filter(|e| !e.is_empty())
        .map(|v| String::from(v).parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();
    let player2 = sections
        .get(1)
        .unwrap()
        .split('\n')
        .filter(|e| !e.starts_with("Player"))
        .filter(|e| !e.is_empty())
        .map(|v| String::from(v).parse::<i64>().unwrap())
        .collect::<VecDeque<i64>>();

    (player1, player2)
}

fn compute_deck_value(deck: &VecDeque<i64>) -> i64 {
    let mut value = 0;
    let mut multiplier = 1;
    for card in deck.iter().rev() {
        value += card * multiplier;
        multiplier += 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use crate::day22::{get_deals, compute_deck_value};
    use crate::day22::play_rounds;

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
10".split("\n\n").map(String::from).collect::<Vec<String>>();
        let (mut player1_cards, mut player2_cards) = get_deals(&sections);
        play_rounds(&mut player1_cards, &mut player2_cards);
        assert_eq!(306, compute_deck_value(&player2_cards));
    }
}
