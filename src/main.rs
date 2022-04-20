mod lang;
use lang::language::Expr;
use lang::language::Statement;
use lang::language::Type;

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
            &Expr::String("Nobody Expects the Spanish Inquisition."),
        ),
        Statement::Let("str_ref", Type::String, &Expr::Reference("str")),
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
            &Expr::String("Nobody Expects the Spanish Inquisition."),
        ),
        Statement::Scope(vec![Statement::LetMut(
            "str_mut_ref",
            Type::String,
            &Expr::Reference("str"),
        )]),
        Statement::Let("str_ref", Type::String, &Expr::Reference("str")),
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
            &Expr::String("Nobody Expects the Spanish Inquisition."),
        ),
        Statement::Let("str_ref", Type::String, &Expr::Reference("str")),
        // Mutable reference while other reference exists.
        Statement::LetMut("str_mut_ref", Type::String, &Expr::Reference("str")),
    ])];
    println!("Printing good1");
    print_program(&good1[..]);
    println!("\n\nPrinting good2");
    print_program(&good2[..]);
    println!("\n\nPrinting bad");
    print_program(&bad[..]);

    println!("Valid:{}", type_check(&good1));
}

fn print_program(program: &[Statement]) {
    for e in program.iter() {
        println!("{}", e)
    }
}

/**
 * Attempts to type Check the provided program.... probably fails
 */
fn type_check(program: &[Statement]) -> bool {
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

fn check_individual(t: &Type, expr: &Expr) -> bool {
    match (t, expr) {
        (Type::Int32, Expr::Int32(_)) => return true,
        (Type::Int32, Expr::Add(a, b)) => {
            return check_individual(&Type::Int32, a) && check_individual(&Type::Int32, b)
        }
        (Type::String, Expr::String(_)) => return true,
        (Type::Pair(t_a, t_b), Expr::Pair(e_a, e_b)) => {
            check_individual(t_a, e_a) && check_individual(t_b, e_b)
        }
        // It is starting to get complicated...
        (_, _) => false,
    }
}
