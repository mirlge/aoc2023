pub mod pt1;

const INPUT: &str = "Time: 49 97 94 94
Distance: 263 1532 1378 1851";

pub fn main() {
    println!("{}", pt1::calculate_answer(INPUT.to_string()));
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "Time: 7 15 30
Distance: 9 40 200";

    #[test]
    fn pt1_example_input() {
        assert_eq!(
            crate::d6::pt1::calculate_answer(EXAMPLE_INPUT.to_string()),
            288
        )
    }
}
