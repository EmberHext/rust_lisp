use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        char,
        digit1,
        multispace0,
        one_of,
        alphanumeric1,
    },
    combinator::{
        map,
        map_res,
        recognize,
    },
    error::{
        context,
        VerboseError,
    },
    multi::{
        many1,
        many0,
    },
    sequence::{
        delimited,
        preceded,
        terminated,
        tuple,
        pair,
    },
    IResult,
};
use crate::types::{Instructions, Atom, Expr};

pub fn parse_instructions(input: &str) -> IResult<&str, Instructions, VerboseError<&str>> {
    context(
        "instructions",
        alt((
            map(tag("+"), |_| Instructions::Plus),
            map(tag("-"), |_| Instructions::Minus),
            map(tag("*"), |_| Instructions::Multiply),
            map(tag("/"), |_| Instructions::Divide),
            map(tag("="), |_| Instructions::Equal),
            map(tag("not"), |_| Instructions::Not),
            map(tag("define"), |_| Instructions::Define),
        )),
    )(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    context(
        "atom",
        alt((
            map(parse_integer, Atom::Int),
            map(parse_key, Atom::Key),
            map(parse_boolean, Atom::Boolean),
            map(parse_instructions, Atom::Instructions),
        )),
    )(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, i32, VerboseError<&str>> {
    context(
        "integer",
        map_res(digit1, |s: &str| s.parse::<i32>()),
    )(input)
}

pub fn parse_key(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    context(
        "key",
        map(
            recognize(tuple((
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            ))),
            String::from,
        ),
    )(input)
}

pub fn parse_boolean(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
    context(
        "boolean",
        map_res(
            terminated(one_of("tf"), multispace0),
            |s: char| match s {
                't' => Ok(true),
                'f' => Ok(false),
                _ => {
                    Err(nom::Err::Failure(VerboseError {
                        errors: vec![(
                            input,
                            nom::error::VerboseErrorKind::Context("Invalid boolean"),
                        )],
                    }))
                }
            },
        ),
    )(input)
}


pub fn parse_constant(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context("constant", map(parse_atom, Expr::Constant))(input)
}

pub fn parse_define(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "define",
        preceded(
            tag("define"),
            delimited(
                char('('),
                map(
                    pair(
                        preceded(multispace0, parse_key),
                        preceded(
                            pair(char('('), multispace0),
                            terminated(parse_expr, pair(multispace0, char(')'))),
                        ),
                    ),
                    |(key, expr)| Expr::Application(Box::new(Expr::Constant(Atom::Key(key))), vec![expr]),
                ),
                char(')'),
            ),
        ),
    )(input)
}

pub fn parse_application(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "application",
        map(
            delimited(
                char('('),
                terminated(
                    preceded(multispace0, many1(preceded(multispace0, parse_expr))),
                    multispace0,
                ),
                char(')'),
            ),
            |exprs| {
                let (head, tail) = exprs.split_first().unwrap();
                Expr::Application(Box::new(head.clone()), tail.to_vec())
            },
        ),
    )(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "expr",
        alt((
            parse_constant,
            parse_application,
            parse_define,
        )),
    )(input)
}
