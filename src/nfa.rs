use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    fmt::Display,
};

use crate::dfa::{self, Dfa};

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
            "{} -- '{}' --> {}",
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
        f.write_str("=========")
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

    pub fn closure_(&self, state: State) -> BTreeSet<State> {
        let mut set = BTreeSet::new();
        set.insert(state);
        self.closure(&set)
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

    pub fn to_dfa(&self, alphabets: &[char]) -> Dfa {
        let mut states = DfaStateProvider::default();
        let mut queue = VecDeque::new();
        let mut rules = BTreeSet::new();

        let start_closure = self.closure_(self.start);

        queue.push_back(start_closure);

        while let Some(nfa_states) = queue.pop_front() {
            let from = states.to_dfa_state(nfa_states.clone());
            for c in alphabets {
                let next_nfa_states = self.dfa_edge(&nfa_states, *c);
                let to = states.to_dfa_state(next_nfa_states.clone());
                let rule = dfa::Rule {
                    from,
                    to,
                    alphabet: *c,
                };
                if rules.contains(&rule) {
                    continue;
                }
                rules.insert(rule);
                queue.push_back(next_nfa_states);
            }
        }

        println!();
        println!("{states}");

        Dfa {
            start: states.to_dfa_state(self.closure_(self.start)),
            finish_states: states.get_dfa_finishes(self.finish),
            rules,
        }
    }
}

struct DfaStateProvider {
    states: HashMap<BTreeSet<State>, dfa::State>,
    current: dfa::State,
}

impl Default for DfaStateProvider {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
            current: 0,
        }
    }
}

impl DfaStateProvider {
    pub fn to_dfa_state(&mut self, nfa_state_set: BTreeSet<State>) -> dfa::State {
        if let Some(state) = self.states.get(&nfa_state_set) {
            *state
        } else {
            self.current += 1;
            self.states.insert(nfa_state_set, self.current);
            self.current
        }
    }

    pub fn get_dfa_finishes(&self, nfa_finish: State) -> BTreeSet<dfa::State> {
        let mut v = BTreeSet::new();
        for (nfa_states, dfa_state) in &self.states {
            if nfa_states.contains(&nfa_finish) {
                v.insert(*dfa_state);
            }
        }
        v
    }
}

impl Display for DfaStateProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NFA states <=> DFA state\n")?;
        for (nfa_states, dfa_state) in &self.states {
            f.write_fmt(format_args!("{:?}\t{}\n", nfa_states, dfa_state))?;
        }
        Ok(())
    }
}
