use std::{collections::BTreeSet, fmt::Display};

use crate::dfa;

pub type State = i32;

pub struct GlobalEnv {
    pub current: State,
}

impl Default for GlobalEnv {
    fn default() -> Self {
        Self { current: 0 }
    }
}

impl GlobalEnv {
    pub fn new_state(&mut self) -> State {
        self.current += 1;
        self.current
    }
}

#[derive(Debug)]
pub struct Rule {
    pub from: State,
    pub to: State,
    pub alphabet: char,
}

#[derive(Debug)]
pub struct Nfa {
    pub start: State,
    pub finish: State,
    pub rules: Vec<Rule>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -- {} --> {}",
            self.from, self.alphabet, self.to
        ))
    }
}

impl Display for Nfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("== NFA ==\n")?;
        f.write_fmt(format_args!("start: {}\n", self.start))?;
        f.write_fmt(format_args!("finish: {}\n", self.finish))?;
        for rule in &self.rules {
            f.write_fmt(format_args!("{}\n", rule))?;
        }
        f.write_str("=========\n")
    }
}

impl Nfa {
    pub fn edge(&self, s: State, c: char) -> BTreeSet<State> {
        let mut ret = BTreeSet::new();
        for t in self
            .rules
            .iter()
            .filter(|r| r.from == s && r.alphabet == c)
            .map(|r| r.to)
        {
            ret.insert(t);
        }
        ret
    }

    pub fn closure(&self, states: &BTreeSet<State>) -> BTreeSet<State> {
        let mut t = states.clone();
        loop {
            let mut t_dash = t.clone();
            for s in &t {
                t_dash.extend(self.edge(*s, 'ε'));
            }
            if t.len() == t_dash.len() {
                break;
            }
            t = t_dash;
        }
        t
    }

    pub fn dfa_edge(&self, states: &BTreeSet<State>, c: char) -> BTreeSet<State> {
        let mut e = BTreeSet::new();
        for s in states {
            e.extend(self.edge(*s, c));
        }
        self.closure(&e)
    }

    pub fn to_dfa(&self, env: &mut dfa::GlobalEnv) {
        todo!()
    }
}
