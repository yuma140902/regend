use std::fmt::Display;

use crate::nfa::{GlobalEnv, Nfa, Rule};

#[derive(Debug)]
pub enum RegExpr {
    Char(char),
    Cat(Vec<RegExpr>),
    Or(Vec<RegExpr>),
    Repeat(Box<RegExpr>),
}

pub fn cat_char(s: &str) -> RegExpr {
    let mut v = vec![];
    for c in s.chars() {
        v.push(RegExpr::Char(c));
    }
    RegExpr::Cat(v)
}

impl RegExpr {
    pub fn to_nfa(&self, env: &mut GlobalEnv) -> Nfa {
        match self {
            RegExpr::Char(c) => {
                let start = env.new_state();
                let finish = env.new_state();
                let rules = vec![Rule {
                    from: start,
                    to: finish,
                    alphabet: *c,
                }];
                Nfa {
                    start,
                    finish,
                    rules,
                }
            }
            RegExpr::Cat(v) => {
                let start = env.new_state();
                let finish = env.new_state();
                let mut rules = vec![];
                let mut nfa_vec: Vec<_> = v.iter().map(|r| r.to_nfa(env)).collect();

                if nfa_vec.len() == 0 {
                    Nfa {
                        start,
                        finish,
                        rules: vec![],
                    }
                } else {
                    rules.push(Rule {
                        from: start,
                        to: nfa_vec[0].start,
                        alphabet: 'ε',
                    });
                    let mut i = 0;
                    while i < nfa_vec.len() {
                        rules.append(&mut nfa_vec[i].rules);
                        if i < nfa_vec.len() - 1 {
                            rules.push(Rule {
                                from: nfa_vec[i].finish,
                                to: nfa_vec[i + 1].start,
                                alphabet: 'ε',
                            });
                        }
                        i += 1;
                    }
                    rules.push(Rule {
                        from: nfa_vec[nfa_vec.len() - 1].finish,
                        to: finish,
                        alphabet: 'ε',
                    });

                    Nfa {
                        start,
                        finish,
                        rules,
                    }
                }
            }
            RegExpr::Or(v) => {
                let start = env.new_state();
                let finish = env.new_state();
                let mut rules = vec![];
                for mut nfa in v.iter().map(|r| r.to_nfa(env)) {
                    rules.push(Rule {
                        from: start,
                        to: nfa.start,
                        alphabet: 'ε',
                    });
                    rules.push(Rule {
                        from: nfa.finish,
                        to: finish,
                        alphabet: 'ε',
                    });
                    rules.append(&mut nfa.rules);
                }
                Nfa {
                    start,
                    finish,
                    rules,
                }
            }
            RegExpr::Repeat(r) => {
                let start = env.new_state();
                let mut rules = vec![];

                let mut nfa = r.to_nfa(env);
                rules.append(&mut nfa.rules);

                rules.push(Rule {
                    from: start,
                    to: nfa.finish,
                    alphabet: 'ε',
                });
                rules.push(Rule {
                    from: nfa.finish,
                    to: nfa.start,
                    alphabet: 'ε',
                });
                Nfa {
                    start,
                    finish: nfa.finish,
                    rules,
                }
            }
        }
    }
}

impl Display for RegExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegExpr::Char(c) => f.write_fmt(format_args!("{}", c))?,
            RegExpr::Cat(v) => {
                f.write_str("(")?;
                for r in v {
                    f.write_fmt(format_args!("{}", r))?;
                }
                f.write_str(")")?;
            }
            RegExpr::Or(v) => {
                f.write_str("(φ")?;
                for r in v {
                    f.write_fmt(format_args!("|{}", r))?;
                }
                f.write_str(")")?;
            }
            RegExpr::Repeat(r) => {
                f.write_fmt(format_args!("({})*", r))?;
            }
        }
        Ok(())
    }
}
