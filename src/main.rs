use std::collections::BTreeSet;

use alien::{
    nfa::GlobalEnv,
    regexpr::{cat_char, RegExpr::*},
};

fn main() {
    let reg = Or(vec![
        cat_char("00"),
        cat_char("11"),
        Repeat(Box::new(Char('1'))),
    ]);

    /*let reg = Or(vec![
        Cat(vec![
            Repeat(Box::new(Or(vec![
                cat_char("00"),
                cat_char("11"),
                cat_char("111111"),
                cat_char("111000"),
                cat_char("000111"),
                cat_char("000000"),
            ]))),
            Or(vec![cat_char("111"), cat_char("000")]),
            Repeat(Box::new(Or(vec![
                cat_char("00"),
                cat_char("11"),
                cat_char("111111"),
                cat_char("111000"),
                cat_char("000111"),
                cat_char("000000"),
            ]))),
            Or(vec![Char('0'), Char('1')]),
        ]),
        Cat(vec![
            Or(vec![
                cat_char("00"),
                cat_char("11"),
                cat_char("111111"),
                cat_char("111000"),
                cat_char("000111"),
                cat_char("000000"),
            ]),
            Repeat(Box::new(Or(vec![
                cat_char("00"),
                cat_char("11"),
                cat_char("111111"),
                cat_char("111000"),
                cat_char("000111"),
                cat_char("000000"),
            ]))),
        ]),
    ]);*/

    let mut env = GlobalEnv::default();
    let nfa = reg.to_nfa(&mut env);
    println!("regexpr: {reg}");
    println!("{nfa}");
    println!("edge(1, ε) = {:?}", nfa.edge(1, 'ε'));
    {
        let mut t = BTreeSet::new();
        t.insert(1);
        println!("closure(1) = {:?}", nfa.closure(&t));
        println!(
            "DFAedge(closure(1), '0') = {:?}",
            nfa.dfa_edge(&nfa.closure(&t), '0')
        );
    }

    let dfa = nfa.to_dfa(&['0', '1']);
    println!();
    println!("{dfa}");
}
