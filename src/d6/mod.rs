pub mod pt1;

//const PT1_INPUT: &str = "Time: 49 97 94 94
//Distance: 263 1532 1378 1851";

const PT2_INPUT: &str = "Time: 49979494
Distance: 263153213781851";

pub fn main() {
    println!("{}", pt1::calculate_answer(PT2_INPUT.to_string()));
}

#[cfg(test)]
mod tests {
    const PT1_EXAMPLE_INPUT: &str = "Time: 7 15 30
Distance: 9 40 200";

    const PT2_EXAMPLE_INPUT: &str = "Time: 71530
Distance: 940200";

    #[test]
    fn pt1_example_input() {
        assert_eq!(
            crate::d6::pt1::calculate_answer(PT1_EXAMPLE_INPUT.to_string()),
            288
        )
    }

    #[test]
    fn pt2_example_input() {
        assert_eq!(
            crate::d6::pt1::calculate_answer(PT2_EXAMPLE_INPUT.to_string()),
            71503
        )
    }
}
