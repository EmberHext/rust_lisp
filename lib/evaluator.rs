use crate::types::{Environment, Atom, Expr, Instructions};

fn eval_plus(evaluated_operands: Vec<Atom>) -> Result<Atom, String> {
    let sum: i32 = evaluated_operands
        .into_iter()
        .map(|atom| match atom {
            Atom::Int(i) => Ok(i),
            _ => Err("Expected integers for addition.".to_string()),
        })
        .collect::<Result<Vec<_>, String>>()?
        .iter()
        .sum();
    Ok(Atom::Int(sum))
}

fn eval_minus(evaluated_operands: Vec<Atom>) -> Result<Atom, String> {
    let mut operands_iter = evaluated_operands
        .into_iter()
        .map(|atom| match atom {
            Atom::Int(i) => Ok(i),
            _ => Err("Expected integers for subtraction.".to_string()),
        })
        .collect::<Result<Vec<_>, String>>()?
        .into_iter();

    let first_operand = operands_iter
        .next()
        .ok_or_else(|| "Expected at least one operand for subtraction.".to_string())?;

    let diff: i32 = operands_iter.fold(first_operand, |acc, x| acc - x);
    Ok(Atom::Int(diff))
}

fn eval_multiply(evaluated_operands: Vec<Atom>) -> Result<Atom, String> {
    let product: i32 = evaluated_operands
        .into_iter()
        .map(|atom| match atom {
            Atom::Int(i) => Ok(i),
            _ => Err("Expected integers for multiplication.".to_string()),
        })
        .collect::<Result<Vec<_>, String>>()?
        .iter()
        .product();
    Ok(Atom::Int(product))
}

fn eval_divide(evaluated_operands: Vec<Atom>) -> Result<Atom, String> {
    let mut operands_iter = evaluated_operands
        .into_iter()
        .map(|atom| match atom {
            Atom::Int(i) => Ok(i),
            _ => Err("Expected integers for division.".to_string()),
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

pub fn eval(expr: &Expr, env: &mut Environment) -> Result<Atom, String> {
    match expr {
        Expr::Constant(atom) => Ok(atom.clone()),
        Expr::Application(operator, operands) => {
            if let Expr::Constant(Atom::Instructions(instruction)) = **operator {
                let evaluated_operands: Result<Vec<Atom>, String> =
                    operands.iter().map(|operand| eval(operand, env)).collect();
                match instruction {
                    Instructions::Plus => eval_plus(evaluated_operands?),
                    Instructions::Minus => eval_minus(evaluated_operands?),
                    Instructions::Multiply => eval_multiply(evaluated_operands?),
                    Instructions::Divide => eval_divide(evaluated_operands?),
                    _ => Err("Unsupported instruction.".to_string()),
                }
            } else {
                Err("Expected an instruction as the operator.".to_string())
            }
        }
        _ => Err("Unsupported expression.".to_string()),
    }
}