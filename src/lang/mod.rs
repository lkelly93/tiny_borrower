pub mod language {
    use std::fmt;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Type<'a> {
        Int32,
        String,
        Pair(&'a Type<'a>, &'a Type<'a>),
    }

    impl<'a> fmt::Display for Type<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Type::Int32 => write!(f, "Int32"),
                Type::String => write!(f, "String"),
                Type::Pair(a, b) => write!(f, "{} * {}", a, b),
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Statement<'a> {
        Scope(Vec<Statement<'a>>),
        Let(&'a str, Type<'a>, &'a Expr<'a>),
        LetMut(&'a str, Type<'a>, &'a Expr<'a>),
    }

    impl<'a> fmt::Display for Statement<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Statement::Scope(a) => write!(f, "{:?}", a),
                Statement::Let(n, t, e) => write!(f, "Let({}, {}, {})", n, t, e),
                Statement::LetMut(n, t, e) => write!(f, "LetMut({}, {}, {})", n, t, e),
            }
        }
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Expr<'a> {
        Int32(i32),
        String(&'a str),
        Pair(&'a Expr<'a>, &'a Expr<'a>),
        First(&'a Expr<'a>),
        Second(&'a Expr<'a>),
        Reference(&'a str),
        Add(&'a Expr<'a>, &'a Expr<'a>),
        Get(&'a str),
    }

    impl<'a> fmt::Display for Expr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Int32(i) => write!(f, "Int32({})", i),
                Expr::String(s) => write!(f, "String({})", s),
                Expr::Pair(a, b) => write!(f, "Pair({}, {})", a, b),
                Expr::First(first) => write!(f, "First({})", first),
                Expr::Second(s) => write!(f, "Second({})", s),
                Expr::Reference(r) => write!(f, "Reference({})", r),
                Expr::Add(a, b) => write!(f, "Add({}, {})", a, b),
                Expr::Get(g) => write!(f, "Get({})", g),
            }
        }
    }
}
