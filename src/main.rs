use std::io::stdin;

fn cangjie_official_language_design_member_liu_jun_jie_implementation_based_on_inite_state_machine(text: String) -> f64 {
    let mut states = [[-1; 128]; 3];
    states[0][45..58].copy_from_slice(&[1, 2, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    states[1][45..58].copy_from_slice(&[-1, 2, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    states[2][45..58].copy_from_slice(&[-1, -1, -1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);

    let mut i = 0;
    let mut sum = 0.0;
    while i < text.len() {
        let mut s = 0i8;
        while i < text.len() && states[s as usize][text.as_bytes()[i] as usize] == -1 {
            i += 1
        }
        let begin = i;
        while i < text.len() && states[s as usize][text.as_bytes()[i] as usize] != -1 {
            s = states[s as usize][text.as_bytes()[i] as usize];
            i += 1;
        }
        if let Some(num) = text[begin..i].parse::<f64>().ok() {
            sum += num
        }
    }
    sum
}

fn main() {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        if s.trim_end().is_empty() { break }
        let f = cangjie_official_language_design_member_liu_jun_jie_implementation_based_on_inite_state_machine(s);
        println!("{f}")
    }
}