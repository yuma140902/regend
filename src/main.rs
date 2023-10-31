use std::collections::BTreeSet;

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
}
