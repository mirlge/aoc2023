use std::collections::HashMap;

pub fn calculate_answer(input: String) -> i64 {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    let workflows = input_vec[0].split('\n').fold(HashMap::new(), |racc, r| {
        let mut new_racc = racc.clone();
        let r_vec: Vec<&str> = r.trim_end_matches('}').split('{').collect();
        new_racc.insert(r_vec[0], r_vec[1]);
        new_racc
    });
    //println!("[DEBUG] workflows: {:?}", workflows);
    input_vec[1].split('\n').fold(0, |acc, x| {
        let part = x
            .trim_end_matches('}')
            .trim_start_matches('{')
            .split(',')
            .fold(HashMap::new(), |pacc, p| {
                let mut new_pacc = pacc.clone();
                let p_vec: Vec<&str> = p.split('=').collect();
                new_pacc.insert(p_vec[0], p_vec[1].parse::<i64>().unwrap());
                new_pacc
            });
        //println!("[DEBUG] part: {:?}", part);
        let mut current_workflow = "in";
        while current_workflow != "A" && current_workflow != "R" {
            let rule = workflows[current_workflow].split(',');
            let mut found_workflow = false;
            rule.for_each(|r| {
                //println!("[DEBUG] r: {}", r);
                if !found_workflow {
                    if r.contains('>') {
                        let r_vec = r.split(':').collect::<Vec<_>>();
                        let r_vec_idx_0 = r_vec[0].split('>').collect::<Vec<_>>();
                        //println!("[DEBUG] r_vec: {:?}", r_vec);
                        //println!("[DEBUG] r_vec_idx_0: {:?}", r_vec_idx_0);
                        if part[r_vec_idx_0[0]] > r_vec_idx_0[1].parse::<i64>().unwrap() {
                            current_workflow = r_vec[1];
                            found_workflow = true;
                        }
                    } else if r.contains('<') {
                        let r_vec = r.split(':').collect::<Vec<_>>();
                        let r_vec_idx_0 = r_vec[0].split('<').collect::<Vec<_>>();
                        //println!("[DEBUG] r_vec: {:?}", r_vec);
                        //println!("[DEBUG] r_vec_idx_0: {:?}", r_vec_idx_0);
                        if part[r_vec_idx_0[0]] < r_vec_idx_0[1].parse::<i64>().unwrap() {
                            current_workflow = r_vec[1];
                            found_workflow = true;
                        }
                    } else {
                        current_workflow = r;
                        found_workflow = true;
                    }
                }
            });
        }
        if current_workflow == "A" {
            return acc + part.into_values().reduce(|xacc, y| xacc + y).unwrap();
        }
        acc
    })
}
