pub fn calculate_answer(input: String) -> i32 {
    let input_vec: Vec<_> = input
        .split('\n')
        .map(|x| {
            x.split(": ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|y| y.parse::<i32>().unwrap())
        })
        .collect();
    input_vec[0].clone().enumerate().fold(1, |acc, (i, x)| {
        //println!("[DEBUG] x: {}", x);
        let record = input_vec[1].clone().collect::<Vec<_>>()[i];
        //println!("  [DEBUG] record: {}", record);
        let result = (0..=x).fold(0, |xacc, y| {
            //println!("  [DEBUG] y: {}", y);
            if (x - y) * y > record {
                //println!("    [DEBUG] you go longer than the record");
                return xacc + 1;
            }
            xacc
        });
        //println!("  [DEBUG] result: {}", result);
        if result == 0 {
            return acc;
        }
        acc * result
    })
}
