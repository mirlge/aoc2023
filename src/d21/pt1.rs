use std::collections::HashMap;

pub fn calculate_answer(input: String) -> i64 {
    let (map, start_location) = input.split('\n').enumerate().fold(
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
    let queue: Vec<(i64, i64)> = vec![
        (start_location.0 + 1, start_location.1),
        (start_location.0, start_location.1 + 1),
        (start_location.0 - 1, start_location.1),
        (start_location.0, start_location.1 - 1),
    ];
    let steps_left = 64;
    while queue.len() > 0 {
        todo!()
    }
    println!();
    0
}
