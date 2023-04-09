use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, 
        char, 
        digit1, 
        multispace0, 
        multispace1, 
        one_of,
    },
    combinator::{
        cut,
        map,
        map_res,
        opt,
    },
    error::{
        context,
        VerboseError, VerboseErrorKind,
    },
    multi::many1,
    sequence::{
        delimited,
        preceded,
        terminated,
    },
    IResult,
    Parser,
};
use std::collections::HashMap;

type Environment = HashMap<String, Atom>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Instructions {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Not,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
    Int(i32),
    Key(String),
    Boolean(bool),
    Instructions(Instructions),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    Application(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Quote(Vec<Expr>),
}

fn parse_instructions(input: &str) -> IResult<&str, Instructions, VerboseError<&str>> {
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

fn parse_atom(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
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

fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
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

fn eval(expr: &Expr, env: &mut Environment) -> Result<Atom, String> {
    match expr {
        Expr::Constant(atom) => Ok(atom.clone()),
        Expr::Application(operator, operands) => {
            if let Expr::Constant(Atom::Instructions(instruction)) = **operator {
                let evaluated_operands: Result<Vec<Atom>, String> =
                    operands.iter().map(|operand| eval(operand, env)).collect();
                match instruction {
                    Instructions::Plus => {
                        let sum: i32 = evaluated_operands?
                            .into_iter()
                            .map(|atom| match atom {
                                Atom::Int(i) => Ok(i),
                                _ => return Err("Expected integers for addition.".to_string()),
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .iter()
                            .sum();
                        Ok(Atom::Int(sum))
                    }
                    Instructions::Minus => {
                        let mut operands_iter = evaluated_operands?
                            .into_iter()
                            .map(|atom| match atom {
                                Atom::Int(i) => Ok(i),
                                _ => return Err("Expected integers for subtraction.".to_string()),
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .into_iter();
                    
                        let first_operand = operands_iter
                            .next()
                            .ok_or_else(|| "Expected at least one operand for subtraction.".to_string())?;
                    
                        let diff: i32 = operands_iter.fold(first_operand, |acc, x| acc - x);
                        Ok(Atom::Int(diff))
                    }
                    Instructions::Multiply => {
                        let sum: i32 = evaluated_operands?
                            .into_iter()
                            .map(|atom| match atom {
                                Atom::Int(i) => Ok(i),
                                _ => return Err("Expected integers for addition.".to_string()),
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .iter()
                            .product();
                        Ok(Atom::Int(sum))
                    }
                    Instructions::Divide => {
                        let mut operands_iter = evaluated_operands?
                            .into_iter()
                            .map(|atom| match atom {
                                Atom::Int(i) => Ok(i),
                                _ => return Err("Expected integers for division.".to_string()),
                            })
                            .collect::<Result<Vec<_>, String>>()?
                            .into_iter();
                    
                        let first_operand = operands_iter
                            .next()
                            .ok_or_else(|| "Expected at least one operand for division.".to_string())?;
                    
                        let result = operands_iter.try_fold(first_operand, |acc, x| {
                            if x == 0 {
                                Err("Division by zero.".to_string())
                            } else {
                                Ok(acc / x)
                            }
                        })?;
                    
                        Ok(Atom::Int(result))
                    }
                    // Implement other instructions here
                    _ => Err("Unsupported instruction.".to_string()),
                }
            } else {
                Err("Expected an instruction as the operator.".to_string())
            }
        }
        // Implement evaluation for other expression types here
        _ => Err("Unsupported expression.".to_string()),
    }
}

fn main() {
    let input = "(+ 1 2 3)";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    let mut env = Environment::new();
    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let input = "(- 1 2)";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    let mut env = Environment::new();
    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let input = "(* 5 4)";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    let mut env = Environment::new();
    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let input = "(/ 10 2)";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    let mut env = Environment::new();
    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }
}