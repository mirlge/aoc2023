use std::collections::{HashMap, VecDeque};

pub fn calculate_answer(input: String) -> i64 {
    let input_vec: Vec<&str> = input.split("\n\n").collect();
    let map = input_vec[1]
        .split('\n')
        .fold(HashMap::new(), |acc, x| -> HashMap<&str, Vec<&str>> {
            let mut new_acc = acc.clone();
            let x_split: Vec<&str> = x.split(": ").collect();
            let value: Vec<_> = x_split[1].split(' ').collect();
            if new_acc.contains_key(x_split[0]) {
                value.clone().into_iter().for_each(|y| {
                    if !new_acc[x_split[0]].contains(&y) {
                        new_acc.get_mut(x_split[0]).unwrap().push(y);
                    }
                });
            } else {
                new_acc.insert(x_split[0], value.clone());
            }
            value.into_iter().for_each(|y| {
                if new_acc.contains_key(y) {
                    if !new_acc[y].contains(&x_split[0]) {
                        new_acc.get_mut(y).unwrap().push(x_split[0]);
                    }
                } else {
                    new_acc.insert(y, vec![x_split[0]]);
                }
            });
            new_acc
        });
    //println!("[DEBUG] map:");
    //map.clone()
    //    .into_iter()
    //    .for_each(|(k, v)| println!("  [DEBUG] {:?}: {}", k, v));

    let mut queue: VecDeque<(i64, &str)> =
        input_vec[0]
            .split("  ")
            .enumerate()
            .fold(VecDeque::new(), |acc, (i, x)| {
                let mut new_acc = acc.clone();
                x.split(' ').for_each(|y| new_acc.push_back((i as i64, y)));
                new_acc
            });
    let mut visited_nodes: Vec<&str> = Vec::new();
    let mut groups = vec![0, 0];
    while queue.len() > 0 {
        match queue.pop_front() {
            Some((group, node)) => {
                if visited_nodes.contains(&node) {
                    continue;
                }
                visited_nodes.push(node);

                if map.contains_key(&node) {
                    map[node]
                        .clone()
                        .into_iter()
                        .for_each(|x| queue.push_back((group, x)));
                }
                groups[group as usize] += 1;
            }
            None => (),
        }
    }
    groups.into_iter().fold(1, |acc, x| acc * x)
}
