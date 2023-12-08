pub fn calculate_answer(input: String) -> i64 {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    input_vec[1]
        .split('\n')
        .map(|x| x.split(" = "))
        .fold(0, |acc, x| acc)
}
