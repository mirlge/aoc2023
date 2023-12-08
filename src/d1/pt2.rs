pub fn calculate_answer(input: String) -> i32 {
    let input_vec: Vec<&str> = input.split('\n').collect();

    let numbers_strs: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    return input_vec.iter().fold(0, |acc, x| {
        acc + (numbers_strs
            .iter()
            .enumerate()
            .find(|(_, y)| x.find(y).is_some())
            .unwrap()
            .0
            .to_string()
            + &numbers_strs
                .iter()
                .enumerate()
                .rev()
                .find(|(_, y)| x.find(y).is_some())
                .unwrap()
                .0
                .to_string() as &str
            + "2")
            .parse::<i32>()
            .unwrap()
    });
}
