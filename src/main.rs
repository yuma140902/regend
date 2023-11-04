use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use regend::{dfa::Dfa, nfa::GlobalEnv, parser};

#[derive(Debug, Parser)]
struct Args {
    /// 正規表現
    reg: String,

    #[clap(short = 't')]
    test: Option<PathBuf>,

    #[clap(short = 'k')]
    table: bool,
}

fn main() {
    let args = Args::parse();

    let reg = match parser::parse_expr_until_end(&args.reg) {
        Ok(reg) => reg.1,
        Err(e) => panic!("正規表現のパースエラー: {}", e),
    };

    let mut env = GlobalEnv::default();
    let nfa = reg.to_nfa(&mut env);
    println!("regexpr: {reg}");
    println!("{nfa}");

    let dfa = nfa.to_dfa(&reg.get_alphabets());
    println!();
    println!("{dfa}");

    if let Some(test_file) = args.test {
        println!();
        println!("Running tests...");
        let f =
            File::open(&test_file).expect(&format!("ファイル {} が開けない", test_file.display()));
        let reader = BufReader::new(f);
        for line in reader.lines() {
            dfa.run(&line.expect(&format!("ファイル {} を読み込めない", test_file.display())));
        }
    }

    dbg!(dfa.to_table());

    if args.table {
        println!();
        println!("TABLE");
        Dfa::print_table(&dfa.to_table());
    }
}
