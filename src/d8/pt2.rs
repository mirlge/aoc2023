use std::collections::HashMap;

pub fn calculate_answer(input: String) -> i64 {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    let paths = input_vec[1].split('\n').fold(HashMap::new(), |mut acc, x| {
        let x_vec: Vec<&str> = x.split(" = ").collect();
        let next_paths_string = x_vec[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_owned();
        let next_paths: Vec<String> = next_paths_string
            .split(", ")
            .map(|x| x.to_owned())
            .collect();
        acc.insert(x_vec[0], next_paths);
        acc
    });
    let mut current_locations: Vec<&str> = paths.keys().filter(|&&x| x.ends_with("Z")).collect();
    let mut turns = 0;
    let mut lr_instruction_idx = 0;
    let lr_instructions_vec: Vec<_> = input_vec[0].chars().collect();
    //println!("[DEBUG] lr_instructions_vec: {:?}", lr_instructions_vec);
    let mut lr_instruction = lr_instructions_vec[lr_instruction_idx];
    let mut done_locations = 0;
    while done_locations < current_locations.len() {
        //println!("  [DEBUG] current_location: {}", current_location);
        //println!("  [DEBUG] turns: {}", turns);
        //println!("  [DEBUG] lr_instruction_idx: {}", lr_instruction_idx);
        //println!("  [DEBUG] lr_instruction: {}", lr_instruction);
        match lr_instruction {
            'L' => current_location = &paths[current_location][0],
            'R' => current_location = &paths[current_location][1],
            _ => panic!(
                "The current left/right instruction ({}) isn't 'L' or 'R'!",
                lr_instruction
            ),
        }
        turns += 1;
        lr_instruction_idx = (lr_instruction_idx + 1) % lr_instructions_vec.len();
        lr_instruction = lr_instructions_vec[lr_instruction_idx];
    }
    turns
}
