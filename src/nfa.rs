use std::fmt::Display;

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
