use crate::util::inputs::day_input;

pub fn run() {
    let inputs = day_input(25)
        .iter()
        .map(|v| String::from(v).parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let card_pub_key = *inputs.get(0).unwrap();
    let door_pub_key = *inputs.get(1).unwrap();

    let card_loop_size = find_loop_size(card_pub_key);
    let door_loop_size = find_loop_size(door_pub_key);

    let card_enc_key = transform(card_pub_key, door_loop_size, None);
    let door_enc_key = transform(door_pub_key, card_loop_size, None);

    println!("Part 1: {} {}", card_enc_key, door_enc_key);
}

fn find_loop_size(pub_key: i64) -> i64 {
    let mut loop_size = 0;

    let mut transformed_number = 1;
    loop {
        if transformed_number == pub_key {
            break;
        }

        transformed_number = transform(7, 1, Some(transformed_number));

        loop_size += 1;
    }

    loop_size
}

fn transform(subject_number: i64, times: i64, starting_number: Option<i64>) -> i64 {
    let mut transformed_number = if let Some(starting_number) = starting_number {
        starting_number
    } else {
        1
    };

    for i in 0..(times as usize) {
        transformed_number *= subject_number;
        transformed_number %= 20201227;
    }

    transformed_number
}

#[cfg(test)]
mod tests {
    use crate::day25::{find_loop_size, transform};

    #[test]
    fn example_1() {
        let card_pub_key = 5764801;
        let door_pub_key = 17807724;

        assert_eq!(8, find_loop_size(card_pub_key));
        assert_eq!(11, find_loop_size(door_pub_key));

        assert_eq!(14897079, transform(door_pub_key, 8, None));
        assert_eq!(14897079, transform(card_pub_key, 11, None));
    }
}
