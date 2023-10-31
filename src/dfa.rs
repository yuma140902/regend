use std::fmt::Display;

pub type State = i32;

#[derive(Debug)]
pub struct Rule {
    pub from: State,
    pub to: State,
    pub alphabet: char,
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
    pub finish_vec: Vec<State>,
    pub rules: Vec<Rule>,
}

impl Display for Dfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("== DFA ==\n")?;
        f.write_fmt(format_args!("start: {}\n", self.start))?;
        f.write_fmt(format_args!("finish_vec: {:?}\n", self.finish_vec))?;
        for rule in &self.rules {
            f.write_fmt(format_args!("{}\n", rule))?;
        }
        f.write_str("=========")?;
        Ok(())
    }
}
