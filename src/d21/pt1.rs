use std::{cmp::Reverse, collections::BTreeMap};

use priq::PriorityQueue;

pub fn calculate_answer(input: String, steps_left: i64) -> i64 {
    let input_split = input.split('\n');
    let (map, start_location) = input_split.clone().enumerate().fold(
        (BTreeMap::<(i64, i64), char>::new(), (-1, -1)),
        |(map, start_location), (i, x)| {
            x.char_indices().fold(
                (map, start_location),
                |(xmap, new_start_location), (xi, y)| {
                    let mut new_xmap = xmap.clone();
                    let location = (xi as i64, i as i64);
                    if y == 'S' {
                        new_xmap.insert(location, '.');
                        return (new_xmap, location);
                    }
                    new_xmap.insert(location, y);
                    (new_xmap, new_start_location)
                },
            )
        },
    );
    //println!("[DEBUG] map:");
    //map.clone()
    //    .into_iter()
    //    .for_each(|(k, v)| println!("  [DEBUG] {:?}: {}", k, v));

    let mut queue = PriorityQueue::from([
        (
            Reverse(steps_left - 1),
            (start_location.0, start_location.1 + 1),
        ),
        (
            Reverse(steps_left - 1),
            (start_location.0 + 1, start_location.1),
        ),
        (
            Reverse(steps_left - 1),
            (start_location.0, start_location.1 - 1),
        ),
        (
            Reverse(steps_left - 1),
            (start_location.0 - 1, start_location.1),
        ),
    ]);
    let mut visited_positions: Vec<(i64, i64)> = Vec::new();
    let mut result = 0;
    while queue.len() > 0 {
        match queue.pop() {
            Some((Reverse(new_steps_left), location)) => {
                if map[&location] == '#' {
                    continue;
                }
                if visited_positions.contains(&location) {
                    continue;
                }
                visited_positions.push(location);
                if new_steps_left % 2 == 0 {
                    result += 1;
                    if new_steps_left <= 0 {
                        continue;
                    }
                }
                queue.put(Reverse(new_steps_left - 1), (location.0 + 1, location.1));
                queue.put(Reverse(new_steps_left - 1), (location.0, location.1 + 1));
                queue.put(Reverse(new_steps_left - 1), (location.0 - 1, location.1));
                queue.put(Reverse(new_steps_left - 1), (location.0, location.1 - 1));
                //println!(
                //    "[DEBUG] Location {:?} checked ({} step(s) left)!",
                //    location, new_steps_left
                //);
                //println!("[DEBUG] Current result: {}", result);
            }
            None => (),
        }
    }
    result
}
