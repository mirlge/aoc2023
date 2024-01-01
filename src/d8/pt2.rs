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
    let mut current_locations: Vec<&str> = paths
        .clone()
        .into_keys()
        .filter(|x| x.ends_with("A"))
        .collect();
    let mut turns = 0;
    let mut lr_instruction_idx = 0;
    let lr_instructions_vec: Vec<_> = input_vec[0].chars().collect();
    //println!("[DEBUG] lr_instructions_vec: {:?}", lr_instructions_vec);
    let mut lr_instruction = lr_instructions_vec[lr_instruction_idx];
    loop {
        println!(
            "  [DEBUG] current_locations: {:?}",
            current_locations.clone()
        );
        //println!("  [DEBUG] turns: {}", turns);
        //println!("  [DEBUG] lr_instruction_idx: {}", lr_instruction_idx);
        //println!("  [DEBUG] lr_instruction: {}", lr_instruction);
        current_locations = current_locations
            .into_iter()
            .map(|x| {
                let new_x = match lr_instruction {
                    'L' => paths[x][0].as_str(),
                    'R' => paths[x][1].as_str(),
                    _ => panic!(
                        "The current left/right instruction ({}) isn't 'L' or 'R'!",
                        lr_instruction
                    ),
                };
                new_x
            })
            .collect();
        turns += 1;
        lr_instruction_idx = (lr_instruction_idx + 1) % lr_instructions_vec.len();
        lr_instruction = lr_instructions_vec[lr_instruction_idx];
        if current_locations
            .clone()
            .into_iter()
            .all(|x| x.ends_with("Z"))
        {
            break;
        }
    }
    turns
}
