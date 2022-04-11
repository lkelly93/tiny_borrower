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
    let good1 = [
        Statement::Let(
            "str",
            &Expr::Type(&Type::String("Nobody expects the spanish inquisition.")),
        ),
        Statement::Let("str_ref", &Expr::Statement(&Statement::Reference("str"))),
    ];

    /*
     *  Equivalent to the following code:
     *      let str = String::from("Nobody expects the spanish inquisition.")
     *      let str_ref = &str
     *      {
     *          let str_ref_in_scope = &str
     *      }
     */
    let good2 = [
        Statement::Let(
            "str",
            &Expr::Type(&Type::String("Nobody expects the spanish inquisition.")),
        ),
        Statement::Let("str_ref", &Expr::Statement(&Statement::Reference("str"))),
        Statement::Scope(&Expr::Statement(&Statement::Let(
            "str_ref_in_scope",
            &Expr::Statement(&Statement::Get("str")),
        ))),
    ];

    /*
     *  Equivalent to the following code:
     *      let str = String::from("Nobody expects the spanish inquisition.")
     *      let str_ref = &str
     *      let mut str_mut_ref = &str // BAD mutable reference when non-mutable references exist.
     */
    let bad = [
        Statement::Let(
            "str",
            &Expr::Type(&Type::String("Its merely a flesh wound.")),
        ),
        Statement::Let("str_ref", &Expr::Statement(&Statement::Reference("str"))),
        Statement::LetMut(
            "str_mut_ref",
            &Expr::Statement(&Statement::Reference("str")),
        ),
    ];
    println!("Printing good1");
    check(&good1[..]);
    println!("\n\nPrinting good2");
    check(&good2[..]);
    println!("\n\nPrinting bad");
    check(&bad[..]);
}

fn check(program: &[Statement]) -> bool {
    for e in program.iter() {
        println!("{}", e)
    }
    return true;
}
