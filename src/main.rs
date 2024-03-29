use std::{io::stdin, cell::RefCell};

fn cangjie_official_language_design_member_liu_jun_jie_implementation_based_on_finite_state_machine_translated_by_hzqd(text: String) -> f64 {
    let mut states = [[-1; 128]; 3];
    states[0][45..58].copy_from_slice(&[1, 2, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    states[1][45..58].copy_from_slice(&[-1, 2, -1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    states[2][45..58].copy_from_slice(&[-1, -1, -1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]);

    let i = RefCell::new(0);
    (0..).take_while(|_| *i.borrow() < text.len()).fold(0.0, |acc, _| {
        let s = RefCell::new(0i8);
        (0..).take_while(|_| *i.borrow() < text.len() && states[*s.borrow() as usize][text.as_bytes()[*i.borrow()] as usize] == -1).for_each(|_| *i.borrow_mut() += 1);
        let begin = *i.borrow();
        (0..).take_while(|_| *i.borrow() < text.len() && states[*s.borrow() as usize][text.as_bytes()[*i.borrow()] as usize] != -1)
            .for_each(|_| {
                *s.borrow_mut() = states[s.clone().take() as usize][text.as_bytes()[*i.borrow()] as usize];
                *i.borrow_mut() += 1;
            });
        acc + text[begin..*i.borrow()].parse::<f64>().unwrap_or_default()
    })
}
fn main() {
    (0..).map(|_| {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    }).take_while(|s| !s.trim_end().is_empty())
        .map(|t| cangjie_official_language_design_member_liu_jun_jie_implementation_based_on_finite_state_machine_translated_by_hzqd(t))
        .for_each(|f| println!("{f}"));
}