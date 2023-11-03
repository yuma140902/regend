use std::collections::BTreeSet;

use alien::{
    nfa::GlobalEnv,
    regexpr::{cat_char, RegExpr::*},
};

fn main() {
    /*let reg = Or(vec![
        cat_char("00"),
        cat_char("11"),
        Repeat(Box::new(Char('1'))),
    ]);*/

    let reg = Cat(vec![
        Repeat(Box::new(Or(vec![cat_char("00"), cat_char("11")]))),
        Or(vec![Char('0'), Char('1'), Empty]),
        Repeat(Box::new(Or(vec![cat_char("00"), cat_char("11")]))),
    ]);

    let reg = Or(vec![
        Cat(vec![
            Or(vec![cat_char("00"), cat_char("11")]),
            Repeat(Box::new(Or(vec![Char('0'), Char('1')]))),
        ]),
        Cat(vec![
            Repeat(Box::new(Or(vec![Char('0'), Char('1')]))),
            Or(vec![cat_char("00"), cat_char("11")]),
        ]),
    ]);

    let reg = Or(vec![
        Cat(vec![
            cat_char("11"),
            Repeat(Box::new(Or(vec![cat_char("11"), Char('0')]))),
        ]),
        Cat(vec![
            Repeat(Box::new(Or(vec![cat_char("11"), Char('0')]))),
            cat_char("11"),
        ]),
        Cat(vec![
            Repeat(Box::new(Or(vec![cat_char("00"), Char('1')]))),
            cat_char("00"),
        ]),
    ]);

    let test_inputs = vec![
        "110000", "00", "0000100", "1111", "1101111", "1101100", "000000", "1001100", "001100",
        "11100", "1100110", "0010000", "110", "11110", "10000",
    ];

    /*let reg = Cat(vec![
        Or(vec![
            Repeat(Box::new(cat_char("00"))),
            Repeat(Box::new(cat_char("11"))),
        ]),
        Or(vec![Char('0'), Char('1'), Empty]),
        Or(vec![
            Repeat(Box::new(cat_char("00"))),
            Repeat(Box::new(cat_char("11"))),
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

    println!();
    println!("Running tests...");
    for test_input in test_inputs {
        dfa.run(test_input);
    }
}
