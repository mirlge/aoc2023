use std::collections::{HashMap, VecDeque};

pub fn calculate_answer(input: String, steps_left: i64) -> i64 {
    let input_split = input.split('\n');
    let (map, start_location) = input_split.clone().enumerate().fold(
        (HashMap::<(i64, i64), char>::new(), (-1, -1)),
        |(map, start_location), (i, x)| {
            x.char_indices().fold(
                (map, start_location),
                |(xmap, new_start_location), (xi, y)| {
                    let mut new_xmap = xmap.clone();
                    let location = (i as i64, xi as i64);
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
    let mut queue: VecDeque<(i64, (i64, i64))> = VecDeque::from([
        (steps_left, (start_location.0, start_location.1 + 1)),
        (steps_left, (start_location.0 + 1, start_location.1)),
        (steps_left, (start_location.0, start_location.1 - 1)),
        (steps_left, (start_location.0 - 1, start_location.1)),
    ]);
    let mut result = 0;
    while queue.len() > 0 {
        match queue.pop_front() {
            Some((new_steps_left, location)) => {
                if queue
                    .iter()
                    .filter(|&x| *x == (new_steps_left, location))
                    .count()
                    > 0
                {
                    continue;
                }
                if new_steps_left <= 0
                    || location.0 >= input_split.clone().count() as i64
                    || location.0 < 0
                    || location.1 < 0
                    || location.1 >= input_split.clone().nth(0).unwrap().len() as i64
                {
                    result += 1;
                    continue;
                } else if map[&location] == '#' {
                    continue;
                }
                queue.push_back((new_steps_left - 1, (location.0 + 1, location.1)));
                queue.push_back((new_steps_left - 1, (location.0, location.1 + 1)));
                queue.push_back((new_steps_left - 1, (location.0 - 1, location.1)));
                queue.push_back((new_steps_left - 1, (location.0, location.1 - 1)));
                println!(
                    "[DEBUG] Location {:?} checked ({} steps left)!",
                    location, new_steps_left
                );
            }
            None => (),
        }
    }
    result
}
