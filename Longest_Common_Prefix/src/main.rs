use std::{str::Chars, vec};

fn main() {
    let str = longest_common_prefix(vec![
        "strs".to_string(),
        "strggg".to_string(),
        "strrr".to_string(),
    ]);
    println!("{}", str);
}

pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    strs.sort_by_key(|s| s.len());

    let mut chs: Vec<Chars> = (&strs).iter().map(|f| f.chars()).collect();
    let mut res = true;

    let mut result = "".to_string();

    while let Some(c) = chs[0].next() {
        let mut i = 1;
        check_letter(&c, &mut chs, &mut res, &mut i);
        if !res {
            return result;
        } else {
            result.push(c);
        }
    }
    result
}

fn check_letter(c: &char, c1: &mut Vec<Chars>, res: &mut bool, i: &mut usize) {
    if !*res || *i >= c1.len() {
        return;
    }

    if !(*c).cmp(&c1[*i].next().unwrap()).is_eq() {
        *res = false;
    }

    *i += 1;

    check_letter(c, c1, res, i)
}
