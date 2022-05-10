pub mod language {
    use std::fmt;

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[allow(dead_code)]
    pub enum Type {
        Int32,
        String,
        Pair(Box<Type>, Box<Type>),
        Reference(Box<Type>),
    }

    impl<'a> fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Type::Int32 => write!(f, "Int32"),
                Type::String => write!(f, "String"),
                Type::Pair(a, b) => write!(f, "{} * {}", a, b),
                Type::Reference(r) => write!(f, "&{}", r),
            }
        }
    }

    impl Clone for Type {
        fn clone(&self) -> Type {
            match self {
                Type::Int32 => Type::Int32,
                Type::String => Type::String,
                Type::Pair(a, b) => Type::Pair(a.clone(), b.clone()),
                Type::Reference(t) => Type::Reference(t.clone()),
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Statement<'a> {
        Scope(Vec<Statement<'a>>),
        Let(&'a str, Type, Box<Expr<'a>>),
        LetMut(&'a str, Type, Box<Expr<'a>>),
    }

    impl<'a> fmt::Display for Statement<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Statement::Scope(a) => {
                    _ = write!(f, "Scope(");
                    for s in a.iter() {
                        _ = write!(f, "{}, ", s);
                    }
                    return write!(f, ")");
                }
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
        Pair(Box<Expr<'a>>, Box<Expr<'a>>),
        First(Box<Expr<'a>>),
        Second(Box<Expr<'a>>),
        Reference(&'a str),
        Add(Box<Expr<'a>>, Box<Expr<'a>>),
        Get(&'a str),
        Dereference(Box<Expr<'a>>),
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
                Expr::Dereference(g) => write!(f, "Dereference({})", g),
            }
        }
    }
}
