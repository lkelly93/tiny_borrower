mod lang;
use lang::language::Expr;
use lang::language::Statement;
use lang::language::Type;
use std::collections::HashMap;

struct Variable_info {
    t: Type,
}
fn main() {
    /*
     *  Equivalent to the following code:
     *      let str = String::from("Nobody expects the spanish inquisition.")
     *      let str_ref = &str
     */
    let good1 = vec![Statement::Scope(vec![
        Statement::Let(
            "str",
            Type::String,
            Box::new(Expr::String("Nobody Expects the Spanish Inquisition.")),
        ),
        Statement::Let(
            "str_ref",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        ),
    ])];

    /*
     *  Equivalent to the following code:
     *      let str = String::from("Nobody expects the spanish inquisition.")
     *      {
     *          let mut str_mut_ref = &str
     *      }
     *      let str_ref = &str
     */
    let good2 = vec![Statement::Scope(vec![
        Statement::Let(
            "str",
            Type::String,
            Box::new(Expr::String("Nobody Expects the Spanish Inquisition.")),
        ),
        Statement::Scope(vec![Statement::LetMut(
            "str_mut_ref",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        )]),
        Statement::Let(
            "str_ref",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        ),
    ])];

    /*
     *  Equivalent to the following code:
     *      let str = String::from("Nobody expects the spanish inquisition.")
     *      let str_ref = &str
     *      let mut str_mut_ref = &str // BAD mutable reference when non-mutable references exist.
     */
    let bad = vec![Statement::Scope(vec![
        Statement::Let(
            "str",
            Type::String,
            Box::new(Expr::String("Nobody Expects the Spanish Inquisition.")),
        ),
        Statement::Let(
            "str_ref",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        ),
        // Mutable reference while other reference exists.
        Statement::LetMut(
            "str_mut_ref",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        ),
    ])];
    println!("Printing good1");
    print_program(&good1[..]);
    println!("\n\nPrinting good2");
    print_program(&good2[..]);
    println!("\n\nPrinting bad");
    print_program(&bad[..]);

    // println!("Valid:{}", type_check(&good1));
}

fn print_program(program: &[Statement]) {
    for e in program.iter() {
        println!("{:}", e)
    }
}

/**
 * Attempts to type Check the provided program.... probably fails
 */
fn type_check(program: &[Statement]) -> bool {
    let env: HashMap<String, Variable_info> = HashMap::new();
    // TODO: How do I implement scoping environment? Multiple HashMaps????
    for s in program.iter() {
        match s {
            Statement::Scope(vec) => {
                if !type_check(vec) {
                    return false;
                }
            }
            Statement::Let(_, t, expr) => {
                if !check_individual(t, expr) {
                    return false;
                }
            }
            Statement::LetMut(_, t, expr) => {
                if !check_individual(t, expr) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn check_expr<'a>(
    expr: &Expr,
    static_env: &mut HashMap<String, Variable_info>,
) -> Result<Type, String> {
    match expr {
        Expr::Int32(_) => return Ok(Type::Int32),
        Expr::String(_) => return Ok(Type::String),
        Expr::Pair(a, b) => {
            let type_a = check_expr(a, static_env);
            let type_b = check_expr(b, static_env);
            match (type_a, type_b) {
                (Ok(l), Ok(r)) => return Ok(Type::Pair(Box::new(l), Box::new(r))),
                (Err(e), _) => return Err(e),
                (_, Err(e)) => return Err(e),
            }
        }
        Expr::First(f) => {
            let possible = check_expr(f, static_env);
            if let Ok(type_f) = possible {
                match type_f {
                    Type::Pair(a, _) => return Ok(*a),
                    _ => {
                        return Err(String::from(
                            "first was called something that did not resolve to a Type::Pair",
                        ))
                    }
                }
            } else {
                return possible;
            }
        }
        Expr::Second(f) => {
            let possible = check_expr(f, static_env);
            if let Ok(type_f) = possible {
                match type_f {
                    Type::Pair(_, a) => return Ok(*a),
                    _ => {
                        return Err(String::from(
                            "second was called something that did not resolve to a Type::Pair",
                        ))
                    }
                }
            } else {
                return possible;
            }
        }
        Expr::Reference(a) => match static_env.get(*a) {
            None => {
                return Err(String::from(
                    "can't create a reference of a variable not in this scope.",
                ))
            }
            Some(d) => return Ok(d.t.clone()),
        },
        Expr::Add(left, right) => {
            let type_left = check_expr(left, static_env);
            let type_right = check_expr(right, static_env);
            match (type_left, type_right) {
                (Ok(Type::Int32), Ok(Type::Int32)) => return Ok(Type::Int32),
                (Err(e), _) => return Err(e),
                (_, Err(e)) => return Err(e),
                (_, _) => return Err(String::from("add on accepts integers.")),
            }
        }
        Expr::Get(s) => match static_env.get(*s) {
            None => {
                return Err(String::from(format!(
                    "the variable {} does not exist in this static environment.",
                    s
                )))
            }
            Some(d) => return Ok(d.t.clone()),
        },
        Expr::Dereference(a) => check_expr(a, static_env),
    }
}
