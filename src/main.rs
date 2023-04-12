use lisplib::parser::parse_expr;
use lisplib::evaluator::eval;
use lisplib::types::Environment;

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

    let input = "(define x 42)";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    let mut env = Environment::new();
    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let input = "x";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }

    let input = "'x'";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }
    
    let input = "\"Hello Strings\"";
    let (_, parsed_expr) = parse_expr(input).unwrap();
    println!("Parsed expression: {:?}", parsed_expr);

    match eval(&parsed_expr, &mut env) {
        Ok(result) => println!("Evaluated result: {:?}", result),
        Err(error) => println!("Error: {}", error),
    }
}