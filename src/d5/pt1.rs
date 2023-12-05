pub fn calculate_answer(input: String) -> i64 {
    let mut seeds: Vec<i64> = vec![];
    input.split("\n\n").clone().for_each(|x| {
        let x_vec = x.split('\n').collect::<Vec<_>>();
        //println!("[DEBUG] x: {:?}", x_vec);
        let x_iter = x_vec[1..].into_iter();
        if seeds.len() == 0 {
            seeds = x
                .split(": ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|y| y.parse::<i64>().unwrap())
                .collect();
            return;
        }
        seeds = seeds
            .clone()
            .into_iter()
            .map(|s| {
                //println!("  [DEBUG] s: {}", s);
                x_iter.clone().fold(s, |acc, y| {
                    let y_vec: Vec<_> = y
                        .split(' ')
                        .map(|z| z.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    //println!("    [DEBUG] y: {:?}", y_vec);
                    if (y_vec[1]..(y_vec[1] + y_vec[2])).contains(&s) {
                        let new_seed = s - y_vec[1] + y_vec[0];
                        //println!("      [DEBUG] New seed: {}", new_seed);
                        return new_seed;
                    }
                    acc
                })
            })
            .collect();
        //println!("[DEBUG] seeds: {:?}", seeds);
    });
    seeds.clone().into_iter().min().unwrap()
}
