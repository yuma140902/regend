use alien::{
    nfa::GlobalEnv,
    regexpr::{cat_char, RegExpr::*},
};

fn main() {
    let reg = Or(vec![cat_char("00"), cat_char("11")]);
    let mut env = GlobalEnv::default();
    let nfa = reg.to_nfa(&mut env);
    println!("{reg}");
    println!("{nfa}");
    println!("Hello, world!");
}
