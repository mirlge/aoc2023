use ascii::ToAsciiChar;

pub fn calculate_answer(input: String) -> i64 {
    input.split(',').fold(0, |acc, x| {
        acc + x.chars().fold(0, |xacc, y| {
            let y_ascii = y.to_ascii_char().unwrap();
            let y_ascii_code = y_ascii as u8;
            let new_xacc = ((xacc + y_ascii_code as i64) * 17) % 256;
            new_xacc
        })
    })
}
