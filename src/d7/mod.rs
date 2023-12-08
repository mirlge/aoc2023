pub mod pt1;

const INPUT: &str = "";

pub fn main() {
    println!("{}", pt1::calculate_answer(INPUT.to_string()));
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn pt1_example_input() {
        assert_eq!(
            crate::d7::pt1::calculate_answer(EXAMPLE_INPUT.to_string()),
            6440
        )
    }
}
