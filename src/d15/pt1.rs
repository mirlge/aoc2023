use ascii::ToAsciiChar;

pub fn calculate_answer(input: String) -> i64 {
    input.split('\n').fold(0, |acc, x| {
        acc + x.split(',').fold(0, |xacc, y| {
            xacc + y.chars().fold(xacc, |yacc, z| {
                let z_ascii = z.to_ascii_char().unwrap();
                let z_ascii_code = z_ascii as u8;
                let new_yacc = ((yacc + z_ascii_code as i64) * 17) % 256;
                println!("[DEBUG] z: {}", z);
                println!("  [DEBUG] z_ascii: {}", z_ascii);
                println!("  [DEBUG] z_ascii_code: {}", z_ascii_code);
                println!("  [DEBUG] new_yacc: {}", new_yacc);
                new_yacc
            })
        })
    })
}
