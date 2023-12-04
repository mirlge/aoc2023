pub fn calculate_answer(input: String) -> i32 {
    input.split('\n').fold(0, |acc, x| {
        let x_vec: Vec<_> = x
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .map(|y| {
                let y_split = y
                    .split(' ')
                    .filter(|z| z != &" " && z != &"")
                    .map(|z| z.parse::<i32>().unwrap());
                y_split
            })
            .collect();
        println!(
            "[DEBUG] x: {:?}",
            x_vec
                .iter()
                .map(|y| y.clone().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
        acc + x_vec.clone()[1]
            .clone()
            .fold((1, 0), |(multiplier, xacc), y| {
                println!("  [DEBUG] y: {}", y);
                println!("    [DEBUG] multiplier: {}", multiplier);
                println!("    [DEBUG] xacc: {}", xacc);
                if x_vec.clone()[0].any(|z| y == z) {
                    println!("    [DEBUG] Win");
                    let new_xacc = if xacc == 0 { 1 } else { xacc * 2 };
                    return (multiplier, new_xacc);
                }
                (multiplier, xacc)
            })
            .1
    })
}
