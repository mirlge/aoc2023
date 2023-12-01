pub fn calculate_answer(input: String) -> i32 {
    let input_vec: Vec<&str> = input.split('\n').collect();
    return input_vec.iter().fold(0, |acc, x| {
        acc + (x.chars().find(|x| x.is_numeric()).unwrap().to_string()
            + &x.chars()
                .rev()
                .find(|x| x.is_numeric())
                .unwrap()
                .to_string())
            .parse::<i32>()
            .unwrap()
    });
}
