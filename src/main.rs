use alien::regexpr::{cat_char, RegExpr::*};

fn main() {
    let reg = Or(vec![cat_char("00"), cat_char("11")]);
    println!("{reg}");
    println!("Hello, world!");
}
