pub mod language {
    use std::fmt;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Expr<'a> {
        Int32(i32),
        String(&'a str),
        Pair(&'a Expr<'a>, &'a Expr<'a>),
        Scope(&'a Expr<'a>),
        Let(&'a str, &'a Expr<'a>),
        LetMut(&'a str, &'a Expr<'a>),
        Reference(&'a str),
        First(&'a Expr<'a>),
        Second(&'a Expr<'a>),
        Add(&'a str, &'a str),
        Get(&'a str),
    }

    impl<'a> fmt::Display for Expr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Int32(i) => write!(f, "Int32({})", i),
                Expr::String(s) => write!(f, "String(\"{}\")", s),
                Expr::Pair(l, r) => write!(f, "Pair(({},{}))", l, r),
                Expr::Scope(expr) => write!(f, "Scope({})", expr),
                Expr::Let(var, val) => write!(f, "Let({}, {})", var, val),
                Expr::LetMut(var, val) => write!(f, "LetMut({}, {})", var, val),
                Expr::Reference(var) => write!(f, "Reference({})", var),
                Expr::First(expr) => write!(f, "First({})", expr),
                Expr::Second(expr) => write!(f, "Second({})", expr),
                Expr::Add(left, right) => write!(f, "Add({}, {})", left, right),
                Expr::Get(val) => write!(f, "Get({})", val),
            }
        }
    }
}
