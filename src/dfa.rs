use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

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

impl Dfa {
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

    pub fn to_table(&self) -> BTreeMap<State, (bool, BTreeMap<char, State>)> {
        let mut table: BTreeMap<State, (bool, BTreeMap<char, State>)> = BTreeMap::new();
        for rule in &self.rules {
            if let Some((_, row)) = table.get_mut(&rule.from) {
                row.insert(rule.alphabet, rule.to);
            } else {
                let mut row = BTreeMap::new();
                let is_finish = self.finish_states.contains(&rule.from);
                row.insert(rule.alphabet, rule.to);
                table.insert(rule.from, (is_finish, row));
            }
        }
        table
    }

    pub fn minimize(&self) -> Self {
        todo!()
    }

    pub fn print_table(&self) {
        for rule in &self.rules {
            if rule.alphabet == '0' {
                print!("{}:", rule.from);
                if self.finish_states.contains(&rule.from) {
                    print!("f");
                } else {
                    print!("c");
                }
                print!(",{}", rule.to);
            } else if rule.alphabet == '1' {
                println!(",{}", rule.to);
            } else {
                panic!("aaa");
            }
        }
    }
}
