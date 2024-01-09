use std::collections::{BTreeMap, VecDeque};

pub fn calculate_answer(input: String) -> i64 {
    let input_split = input.split('\n');
    let (map, max_loc) = input_split.clone().enumerate().fold(
        (BTreeMap::<(i64, i64), char>::new(), (0, 0)),
        |(map, max_loc), (i, x)| {
            x.char_indices()
                .fold((map, max_loc), |(xmap, xmax_loc), (xi, y)| {
                    let mut new_xmap = xmap.clone();
                    let mut new_xmax_loc = xmax_loc;
                    let location = (xi as i64, i as i64);
                    if location.0 > new_xmax_loc.0 {
                        new_xmax_loc = (location.0, new_xmax_loc.1);
                    }
                    if location.1 > new_xmax_loc.1 {
                        new_xmax_loc = (new_xmax_loc.0, location.1)
                    }
                    new_xmap.insert(location, y);
                    (new_xmap, new_xmax_loc)
                })
        },
    );
    let start_location = (1, 0);
    //println!("[DEBUG] map:");
    //map.clone()
    //    .into_iter()
    //    .for_each(|(k, v)| println!("  [DEBUG] {:?}: {}", k, v));

    let mut queue = VecDeque::from([(0, start_location)]);
    let mut visited_positions: Vec<(i64, i64)> = Vec::new();
    let mut result = 0;
    while queue.len() > 0 {
        match queue.pop_back() {
            Some((new_steps, location)) => {
                //println!("[DEBUG] length of queue: {}", queue.len());
                //println!("[DEBUG] queue: {:?}", queue);
                //println!(
                //    "[DEBUG] Char at location: {}",
                //    map.get(&location).unwrap_or(&' ')
                //);
                if location.0 < 0
                    || location.1 < 0
                    || location.0 > max_loc.0
                    || location.1 > max_loc.1
                    || map[&location] == '#'
                    || visited_positions.contains(&location)
                {
                    continue;
                }
                visited_positions.push(location);
                match map[&location] {
                    '^' => {
                        queue.push_front((new_steps + 1, (location.0, location.1 - 1)));
                    }
                    '>' => {
                        queue.push_front((new_steps + 1, (location.0 + 1, location.1)));
                    }
                    'v' => {
                        queue.push_front((new_steps + 1, (location.0, location.1 + 1)));
                    }
                    '<' => {
                        queue.push_front((new_steps + 1, (location.0 - 1, location.1)));
                    }
                    '.' => {
                        queue.push_front((new_steps + 1, (location.0 + 1, location.1)));
                        queue.push_front((new_steps + 1, (location.0, location.1 + 1)));
                        queue.push_front((new_steps + 1, (location.0 - 1, location.1)));
                        queue.push_front((new_steps + 1, (location.0, location.1 - 1)));
                    }
                    _ => panic!("ERR `map[location]` isn't set to something expected"),
                }
                if new_steps > result {
                    result = new_steps;
                }
                //println!(
                //    "[DEBUG] Location {:?} checked ({} step(s) left)!",
                //    location, new_steps_left
                //);
                //println!("[DEBUG] Current result: {}", result);
            }
            None => {
                ();
            }
        }
    }
    result
}
