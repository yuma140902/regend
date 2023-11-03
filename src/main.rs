use std::collections::BTreeSet;

use alien::{nfa::GlobalEnv, parser};

fn main() {
    let reg = parser::parse_expr_until_end("11(11|0)*|(11|0)*11|(00|1)*00")
        .unwrap()
        .1;

    let test_inputs = vec![
        "110000", "00", "0000100", "1111", "1101111", "1101100", "000000", "1001100", "001100",
        "11100", "1100110", "0010000", "110", "11110", "10000",
    ];

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

    println!();
    println!("KADAI");
    dfa.print_for_kadai();
}
