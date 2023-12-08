pub fn calculate_answer(input: String) -> i32 {
    let input_split = input.split('\n').map(|x| x.chars());
    let input_vec: Vec<_> = input_split.clone().collect();
    input_split.clone().enumerate().fold(0, |acc, (i, x)| {
        let mut last_part_nbr_idx = (-3, -3);
        let mut last_part_nbr = "".to_string();
        acc + x.clone().enumerate().fold(0, |xacc, (xi, y)| {
            let positions = (-1..=1).fold(vec![], |pacc, z| {
                (-1..=1).fold(pacc, |parracc, a| {
                    let mut new_parracc = parracc.clone();
                    let input_vec_clone = input_vec.clone();
                    if a < 0 || z < 0 {
                        return parracc;
                    }
                    let x_list = input_vec_clone.get(i + a as usize);
                    if let Some(x_list_unwrapped) = x_list {
                        let xlu_vec = x_list_unwrapped.clone().collect::<Vec<_>>();
                        let xluv_value = xlu_vec.get(xi + z as usize);
                        if let Some(&xluvv_unwrapped) = xluv_value {
                            new_parracc.push(xluvv_unwrapped);
                            println!("[DEBUG] new positions: {:?}", new_parracc);
                            return new_parracc;
                        }
                    }
                    parracc
                })
            });
            if positions
                .clone()
                .into_iter()
                .any(|c| !c.is_numeric() && c != '.')
            {
                if last_part_nbr_idx == (-3, -3) {
                    last_part_nbr_idx = (xi, i);
                } else if last_part_nbr_idx.0 == xi - 1 && last_part_nbr_idx == i {
                    last_part_nbr += y.to_string().as_str();
                }
            }
            println!("[DEBUG] positions: {:?}", positions);
            xacc
        })
    })
}
