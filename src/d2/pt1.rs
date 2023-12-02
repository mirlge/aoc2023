pub fn calculate_answer(input: String) -> i32 {
    input
        .split('\n')
        .map(|x| x.split(": ").nth(1).unwrap())
        .enumerate()
        .fold(0, |acc, (i, x)| {
            //println!("[DEBUG] x: {:?}", x);
            if !x.split("; ").any(|y| {
                //println!("  [DEBUG] y: {:?}", &y);
                y.split(", ").map(|z| z.split(' ')).any(|z| {
                    let z_vec: Vec<&str> = z.clone().collect();
                    let cubes_amount = z_vec[0].parse::<i32>().unwrap();
                    //println!(
                    //"    [DEBUG] z: {:?}\n      [DEBUG] z_vec: {:?}\n      [DEBUG] cubes_amount: {:?}",
                    //z.clone().collect::<Vec<_>>(), &z_vec, &cubes_amount
                    //);
                    match z_vec[1] {
                        "red" => cubes_amount > 12,
                        "green" => cubes_amount > 13,
                        "blue" => cubes_amount > 14,
                        _ => false,
                    }
                })
            }) {
                let new_acc = acc + i as i32 + 1;
                //println!("  [DEBUG] Game possible");
                //println!("  [DEBUG] New accumulator: {}", new_acc);
                return new_acc;
            }
            acc
        })
}
