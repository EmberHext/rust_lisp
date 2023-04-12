use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1,
        char,
        digit1,
        multispace0,
        one_of,
    },
    combinator::{
        map,
        map_res,
    },
    error::{
        context,
        VerboseError,
    },
    multi::many1,
    sequence::{
        delimited,
        preceded,
        terminated,
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
        )),
    )(input)
}

pub fn parse_atom(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    context(
        "atom",
        alt((
            map_res(digit1, |s: &str| s.parse::<i32>().map(Atom::Int)),
            map(alpha1, |s: &str| Atom::Key(s.to_string())),
            map_res(
                terminated(one_of("tf"), multispace0),
                |s: char| match s {
                    't' => Ok(Atom::Boolean(true)),
                    'f' => Ok(Atom::Boolean(false)),
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
            map(parse_instructions, Atom::Instructions),
        )),
    )(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "expr",
        alt((
            map(parse_atom, Expr::Constant),
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
            // Add parsers for other Expr variants here.
        )),
    )(input)
}