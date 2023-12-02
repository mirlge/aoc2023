struct Game {
    red: i32,
    green: i32,
    blue: i32,
}
impl Game {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

pub fn calculate_answer(input: String) -> i32 {
    input
        .split('\n')
        .map(|x| x.split(": ").nth(1).unwrap())
        .fold(0, |acc, x| {
            //println!("[DEBUG] x: {:?}", x);

            let mut game = Game::new();

            x.split("; ").for_each(|y| {
                //println!("  [DEBUG] y: {:?}", &y);
                y.split(", ").map(|z| z.split(' ')).for_each(|z| {
                    let z_vec: Vec<&str> = z.clone().collect();
                    let cubes_amount = z_vec[0].parse::<i32>().unwrap();
                    //println!(
                    //"    [DEBUG] z: {:?}\n      [DEBUG] z_vec: {:?}\n      [DEBUG] cubes_amount: {:?}",
                    //z.clone().collect::<Vec<_>>(), &z_vec, &cubes_amount
                    //);
                    match z_vec[1] {
                        "red" => {
                            if cubes_amount > game.red {
                                game.red = cubes_amount;
                            }
                        }
                        "green" => {
                            if cubes_amount > game.green {
                                game.green = cubes_amount
                            }
                        }
                        "blue" => {
                            if cubes_amount > game.blue {
                                game.blue = cubes_amount
                            }
                        }
                        _ => {
                            println!("[WARN] Something went wrong...");
                        }
                    };
                });
            });
            acc + game.red * game.green * game.blue
        })
}
