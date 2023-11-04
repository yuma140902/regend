use crate::regexpr::RegExpr;
use nom::{
    branch::alt,
    character::complete::{char, multispace0, one_of},
    combinator::{eof, map},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

pub fn parse_expr_until_end(input: &str) -> IResult<&str, RegExpr> {
    terminated(parse_expr, eof)(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, RegExpr> {
    map(
        pair(
            ws(parse_orterm),
            many0(preceded(ws(char('|')), parse_orterm)),
        ),
        |(r, mut rv)| {
            let mut v = vec![r];
            v.append(&mut rv);
            RegExpr::Or(v)
        },
    )(input)
}

fn parse_orterm(input: &str) -> IResult<&str, RegExpr> {
    map(
        pair(ws(parse_catterm), many0(parse_catterm)),
        |(r, mut rv)| {
            let mut v = vec![r];
            v.append(&mut rv);
            RegExpr::Cat(v)
        },
    )(input)
}

fn parse_catterm(input: &str) -> IResult<&str, RegExpr> {
    alt((
        map(terminated(parse_repterm, ws(char('*'))), |r| {
            RegExpr::Repeat(Box::new(r))
        }),
        parse_repterm,
    ))(input)
}

fn parse_repterm(input: &str) -> IResult<&str, RegExpr> {
    alt((parse_repterm_par, parse_repterm_empty, parse_repterm_char))(input)
}

fn parse_repterm_par(input: &str) -> IResult<&str, RegExpr> {
    delimited(ws(char('(')), parse_expr, ws(char(')')))(input)
}

fn parse_repterm_char(input: &str) -> IResult<&str, RegExpr> {
    map(
        ws(one_of(
            "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
        )),
        |c| RegExpr::Char(c),
    )(input)
}

fn parse_repterm_empty(input: &str) -> IResult<&str, RegExpr> {
    map(ws(char('φ')), |_| RegExpr::Empty)(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
