mod lang;
use lang::language::Expr;

fn main() {
    let good1 = [
        Expr::Let("str", &Expr::String("This program is fine.")),
        Expr::Let("str_ref1", &Expr::Reference("str")),
        Expr::Let("str_ref1", &Expr::Reference("str")),
        Expr::Let("str_ref3", &Expr::Reference("str")),
    ];
    let good2 = [
        Expr::Let("str", &Expr::String("This program is fine.")),
        Expr::LetMut("str_mut_ref1", &Expr::Reference("str")),
    ];
    let bad = [
        Expr::Let("str", &Expr::String("This program is not.")),
        Expr::Let("str_ref1", &Expr::Reference("str")),
        Expr::LetMut("str_mut_ref1", &Expr::Reference("str")),
    ];
    println!("Printing good1");
    check(&good1[..]);
    println!("\n\nPrinting good2");
    check(&good2[..]);
    println!("\n\nPrinting bad");
    check(&bad[..]);
}

fn check(program: &[Expr]) -> bool {
    for e in program.iter() {
        println!("{}", e)
    }
    return true;
}

// #[derive(Debug)]
// enum Type<'a> {
//     Int32(i32),
//     String(&'a str),
//     Pair(&'a Type<'a>, &'a Type<'a>),
// }

// impl<'a> fmt::Display for Type<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Type::Int32(i) => write!(f, "Int32({})", i),
//             Type::String(s) => write!(f, "String(\"{}\")", s),
//             Type::Pair(l, r) => write!(f, "Pair(({},{}))", l, r),
//         }
//     }
// }
