use std::collections::BTreeSet;

use nfa::GlobalEnv;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod dfa;
pub mod nfa;
pub mod parser;
pub mod regexpr;

#[wasm_bindgen]
pub fn str_to_dfa(s: &str) -> Dfa {
    let regex = parser::parse_expr_until_end(s).unwrap().1;
    let mut env = GlobalEnv::default();
    let nfa = regex.to_nfa(&mut env);
    let dfa = nfa.to_dfa(&regex.get_alphabets());
    dfa.into()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Dfa {
    pub start: dfa::State,
    pub states: Vec<DfaState>,
    pub rules: Vec<dfa::Rule>,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DfaState {
    pub id: dfa::State,
    pub finish: bool,
}

impl From<dfa::Dfa> for Dfa {
    fn from(value: dfa::Dfa) -> Self {
        let start = value.start;

        let mut states_set = BTreeSet::new();
        for rule in &value.rules {
            states_set.insert(DfaState {
                id: rule.from,
                finish: value.finish_states.contains(&rule.from),
            });
        }
        let mut states = Vec::new();
        states.extend(states_set);

        let mut rules = Vec::new();
        rules.extend(value.rules);

        Self {
            start,
            states,
            rules,
        }
    }
}
