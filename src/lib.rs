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
pub fn str_to_dfa(s: &str) -> DfaWasm {
    let regex = parser::parse_expr_until_end(s).unwrap().1;
    let mut env = GlobalEnv::default();
    let nfa = regex.to_nfa(&mut env);
    let dfa = nfa.to_dfa(&regex.get_alphabets());
    dfa.into()
}

#[wasm_bindgen(getter_with_clone)]
pub struct DfaWasm {
    pub start: dfa::State,
    pub finish_states: Vec<dfa::State>,
    pub rules: Vec<dfa::Rule>,
}

impl From<dfa::Dfa> for DfaWasm {
    fn from(value: dfa::Dfa) -> Self {
        let start = value.start;
        let mut finish_states = vec![];
        finish_states.extend(value.finish_states);
        let mut rules = vec![];
        rules.extend(value.rules);

        Self {
            start,
            finish_states,
            rules,
        }
    }
}
