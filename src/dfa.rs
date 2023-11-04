use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use itertools::Itertools;

use wasm_bindgen::prelude::*;

use wasm_bindgen::prelude::*;

pub type State = i32;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Rule {
    pub from: State,
    pub alphabet: char,
    pub to: State,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -- '{}' --> {}",
            self.from, self.alphabet, self.to
        ))
    }
}

#[derive(Debug)]
pub struct Dfa {
    pub start: State,
    pub finish_states: BTreeSet<State>,
    pub rules: BTreeSet<Rule>,
}

pub type Table = BTreeMap<State, (bool, bool, BTreeMap<char, State>)>;

impl Display for Dfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const UNDERLINE: &str = ""; //"\x1b[4m";
        const BOLD: &str = ""; //"\x1b[1m";
        const RESET: &str = ""; //"\x1b[0m";
        f.write_str("== DFA ==\n")?;
        f.write_fmt(format_args!("start: {}\n", self.start))?;
        f.write_fmt(format_args!("finish_states: {:?}\n", self.finish_states))?;
        for rule in &self.rules {
            if self.finish_states.contains(&rule.from) {
                f.write_fmt(format_args!("{}{}{}{}", UNDERLINE, BOLD, rule.from, RESET))?;
            } else {
                f.write_fmt(format_args!("{}", rule.from))?;
            }
            f.write_fmt(format_args!(" -- '{}' --> ", rule.alphabet))?;
            if self.finish_states.contains(&rule.to) {
                f.write_fmt(format_args!("{}{}{}{}", UNDERLINE, BOLD, rule.to, RESET))?;
            } else {
                f.write_fmt(format_args!("{}", rule.to))?;
            }
            f.write_str("\n")?;
        }
        f.write_str("=========")?;
        Ok(())
    }
}

impl Dfa {
    pub fn from_table(table: &Table) -> Self {
        let mut start = None;
        let mut finish_states = BTreeSet::new();
        let mut rules = BTreeSet::new();

        for (from, (is_finish, is_start, row)) in table {
            if *is_start {
                if start.is_some() {
                    panic!("複数の開始状態がある")
                }
                start = Some(*from);
            }
            if *is_finish {
                finish_states.insert(*from);
            }

            for (alphabet, to) in row {
                rules.insert(Rule {
                    from: *from,
                    alphabet: *alphabet,
                    to: *to,
                });
            }
        }

        Self {
            start: start.expect("開始状態がない"),
            finish_states,
            rules,
        }
    }

    pub fn run(&self, input: &str) -> State {
        print!("\"{}\"\t", input);
        let mut current = self.start;
        print!("{}", current);

        for c in input.chars() {
            let mut rule = None;
            for r in &self.rules {
                if r.alphabet == c && r.from == current {
                    rule = Some(r);
                    break;
                }
            }
            if let Some(rule) = rule {
                current = rule.to;
                print!("->{}", current);
            } else {
                panic!("no rule from={}, alphabet={}. DFA is invalid.", current, c);
            }
        }

        const GREEN: &str = "\x1b[32m";
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";

        if self.finish_states.contains(&current) {
            println!("\t{GREEN}Accepted{RESET}");
        } else {
            println!("\t{RED}Rejected{RESET}");
        }

        current
    }

    pub fn to_table(&self) -> Table {
        let mut table: Table = BTreeMap::new();
        for rule in &self.rules {
            if let Some((_, _, row)) = table.get_mut(&rule.from) {
                row.insert(rule.alphabet, rule.to);
            } else {
                let mut row = BTreeMap::new();
                let is_start = rule.from == self.start;
                let is_finish = self.finish_states.contains(&rule.from);
                row.insert(rule.alphabet, rule.to);
                table.insert(rule.from, (is_finish, is_start, row));
            }
        }
        table
    }

    pub fn print_table(table: &Table) {
        for (state, (is_finish, is_start, row)) in table {
            if *is_start {
                print!("{state}:");
                print!("{},", if *is_finish { "f" } else { "c" });
                println!("{}", row.values().map(|s| format!("{s}")).join(","));
            }
        }
        for (state, (is_finish, is_start, row)) in table {
            if !is_start {
                print!("{state}:");
                print!("{},", if *is_finish { "f" } else { "c" });
                println!("{}", row.values().map(|s| format!("{s}")).join(","));
            }
        }
    }
}
