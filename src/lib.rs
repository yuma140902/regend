use std::collections::BTreeSet;

use itertools::Itertools;
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
    pub rules: Vec<DfaRule>,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DfaState {
    pub id: dfa::State,
    pub finish: bool,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct DfaRule {
    pub from: dfa::State,
    pub to: dfa::State,
    pub alphabets: String,
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
        for ((from, to), rs) in value
            .rules
            .into_iter()
            .into_group_map_by(|r| (r.from, r.to))
        {
            let alphabets: String = rs.iter().map(|r| r.alphabet).collect();
            rules.push(DfaRule {
                from,
                to,
                alphabets,
            });
        }

        Self {
            start,
            states,
            rules,
        }
    }
}
