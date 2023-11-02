use std::{collections::BTreeSet, fmt::Display};

pub type State = i32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
