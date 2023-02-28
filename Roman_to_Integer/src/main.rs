use core::panic;

fn main() {
    let s = "MCMXCIV".to_string();
    let a = roman_to_int(s);
    println!("{}", a);
}
pub fn roman_to_int(s: String) -> i32 {
    let mut chars = s.chars();
    let mut sum = 0;
    let mut prev = 1000;

    while let Some(c) = chars.next() {
        let mut roman_c = from_char_to_roman(c);
        let first_int = roman_c.retrieve();
        if prev < first_int {
            sum -= 2 * prev;
        }
        prev = first_int;
        sum += first_int;
    }

    sum
}
pub enum Roman {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}
impl Roman {
    pub fn retrieve(&mut self) -> i32 {
        match self {
            Roman::I => 1,
            Roman::V => 5,
            Roman::X => 10,
            Roman::L => 50,
            Roman::C => 100,
            Roman::D => 500,
            Roman::M => 1000,
        }
    }
}
pub fn from_char_to_roman(ch: char) -> Roman {
    match ch.to_string().as_str() {
        "I" => Roman::I,
        "V" => Roman::V,
        "X" => Roman::X,
        "L" => Roman::L,
        "C" => Roman::C,
        "D" => Roman::D,
        "M" => Roman::M,
        _ => panic!(),
    }
}
