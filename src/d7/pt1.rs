enum HandType {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
    Undefined,
}

pub fn calculate_answer(input: String) -> i32 {
    let input_split = input.split('\n').map(|x| x.split(' '));
    input_split.clone().fold(0, |acc, x| {
        let x_vec = x.collect::<Vec<_>>();
        let hand_type = x_vec[0].chars().fold(HandType::Undefined, |hacc, c| hacc);
        let rank = x_vec[0].chars().fold(1, |racc, y| racc);
        input_split.clone().fold(rank, |acc2, x2| acc2);
        acc + x_vec[1].parse::<i32>().unwrap() * rank
    })
}
