mod lang;
use lang::language::Expr;
use lang::language::Statement;
use lang::language::Type;
use std::collections::HashMap;

struct VariableInfo {
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
    //println!("Printing good1");
    //print_program(&good1[..]);
    //println!("\n\nPrinting good2");
    //print_program(&good2[..]);
    //println!("\n\nPrinting bad");
    //print_program(&bad[..]);

    //let mut env: HashMap<String, VariableInfo> = HashMap::new();
    //let check_1: bool = type_check(good1.as_slice(), &mut env);
    //println!("Valid:{}", check_1)
    // println!("Valid:{}", type_check(&good1));
    test_borrows();
}

fn test_borrows() {
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
    // let mut vars: HashMap<String, ReferenceType> = HashMap::new();
    // println!(
    //     "Single Reference.\n\tExpected true.\n\tActual = {}",
    //     borrow_check(&good1, &mut vars)
    // );

    let bad1 = vec![Statement::Scope(vec![
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
        Statement::LetMut(
            "str_ref_mut",
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference("str")),
        ),
    ])];
    let mut vars: HashMap<String, ReferenceType> = HashMap::new();
    println!(
        "Single Reference Then Mutable Reference.\n\tExpected false.\n\tActual = {}",
        borrow_check(&bad1, &mut vars)
    );
}

fn print_program(program: &[Statement]) {
    for e in program.iter() {
        println!("{:}", e)
    }
}

enum ReferenceType {
    Mutable,
    NotMutable,
}
impl Clone for ReferenceType {
    fn clone(&self) -> ReferenceType {
        match self {
            ReferenceType::Mutable => ReferenceType::Mutable,
            ReferenceType::NotMutable => ReferenceType::NotMutable,
        }
    }
}

fn borrow_check(program: &[Statement], vars: &mut HashMap<String, ReferenceType>) -> bool {
    for s in program.iter() {
        match s {
            Statement::Scope(vec) => {
                let mut new_vars: HashMap<String, ReferenceType> = HashMap::new();
                for (key, value) in &*vars {
                    new_vars.insert(key.clone(), value.clone());
                }
                borrow_check(vec, &mut new_vars);
            }
            Statement::Let(_, Type::Reference(_), expr) => {
                let expr_unboxed = *expr ;
                match expr_unboxed {
                   Expr::Reference(str)  => return true,
                    _ => return false,
                }

                //None => {
                    //vars.insert(str.to_string(), ReferenceType::NotMutable);
                //}
                //Some(rtype) => match rtype {
                    //ReferenceType::Mutable => return false,
                    //ReferenceType::NotMutable => continue,
                //},
            },
            Statement::LetMut(_, Type::Reference(str), _) => match vars.get(&str.to_string()) {
                None => {
                    vars.insert(str.to_string(), ReferenceType::Mutable);
                }
                Some(_) => return false,
            },
            // We only care about Let[Mut] if the type is a Reference, in all other
            // cases we just continue on.
            _ => ()
            // I don't know if I need to handle the last bullet point on the google doc... I don't
            // think I do because of the way the language is set up.
        }
    }
    return true;
}

/**
 * Attempts to type Check the provided program.... probably fails
 */
fn type_check(program: &[Statement], env: &mut HashMap<String, VariableInfo>) -> bool {
    // let mut env: HashMap<String, VariableInfo> = HashMap::new();
    for s in program.iter() {
        match s {
            Statement::Scope(vec) => {
                let mut new_env: HashMap<String, VariableInfo> = HashMap::new();
                for (key, value) in &*env {
                    new_env.insert(key.clone(), VariableInfo { t: value.t.clone() });
                }
                type_check(vec, &mut new_env);
            }
            Statement::Let(str, t, expr) => {
                let result = check_expr(&expr, env);
                match result {
                    Ok(type_result) => {
                        if type_result == *t {
                            env.insert(str.to_string(), VariableInfo { t: t.clone() });
                        } else {
                            return false;
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        return false;
                    }
                }
            }
            Statement::LetMut(str, t, expr) => {
                let result = check_expr(&expr, env);
                match result {
                    Ok(type_result) => {
                        if type_result == *t {
                            env.insert(str.to_string(), VariableInfo { t: t.clone() });
                        } else {
                            println!("{} != {}", type_result, t);
                            return false;
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn check_expr<'a>(
    expr: &Expr,
    static_env: &mut HashMap<String, VariableInfo>,
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
            Some(d) => return Ok(Type::Reference(Box::new(d.t.clone()))),
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
        // TODO: UNDO REFERENCE.
        Expr::Dereference(a) => check_expr(a, static_env),
    }
}
