use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

use crate::nfa;

pub type State = i32;

pub struct GlobalEnv {
    pub past_states: HashMap<BTreeSet<nfa::State>, State>,
    pub current: State,
}

impl Default for GlobalEnv {
    fn default() -> Self {
        Self {
            past_states: HashMap::new(),
            current: 0,
        }
    }
}

impl GlobalEnv {
    pub fn has_state_for(&self, nfa_states: &BTreeSet<nfa::State>) -> bool {
        self.past_states.contains_key(nfa_states)
    }

    fn new_state(&mut self) -> State {
        self.current += 1;
        self.current
    }

    pub fn get_state_for(&mut self, nfa_states: BTreeSet<nfa::State>) -> State {
        if let Some(state) = self.past_states.get(&nfa_states) {
            *state
        } else {
            let state = self.new_state();
            self.past_states.insert(nfa_states, state);
            state
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub from: State,
    pub to: State,
    pub alphabet: char,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -- {} --> {}",
            self.from, self.alphabet, self.to
        ))
    }
}

#[derive(Debug)]
pub struct Dfa {
    pub start: State,
    pub finish_vec: Vec<State>,
    pub rules: Vec<Rule>,
}

impl Display for Dfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("== DFA ==\n")?;
        f.write_fmt(format_args!("start: {}", self.start))?;
        f.write_fmt(format_args!("finish_vec: {:?}", self.finish_vec))?;
        for rule in &self.rules {
            f.write_fmt(format_args!("{}\n", rule))?;
        }
        f.write_str("=========\n")?;
        Ok(())
    }
}
