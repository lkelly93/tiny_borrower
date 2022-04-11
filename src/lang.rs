pub mod language {
    use std::fmt;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Type<'a> {
        // A primitive int
        Int32(i32),
        // A dynamically allocated String
        String(&'a str),
        // A pair of types, it should take ownership of a string if it has one.
        Pair(&'a Type<'a>, &'a Type<'a>),
    }

    impl<'a> fmt::Display for Type<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Type::Int32(i) => write!(f, "Type::In32({})", i),
                Type::String(s) => write!(f, "Type::String(\"{}\")", s),
                Type::Pair(a, b) => write!(f, "Type::Pair({}, {})", a, b),
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Statement<'a> {
        // Gets the first item from a Type::Pair, if provided argument is not a pair panic
        First(&'a Type<'a>),
        // Gets the second item from a Type::Pair, if provided argument is not a pair panic
        Second(&'a Type<'a>),
        // Adds a new Scope Expressions created in scope should be "cleaned up" after it closes
        Scope(&'a Expr<'a>),
        // Creates a reference from to the provided variable. If no variable in environment, panic.
        Reference(&'a str),
        // Creates a new variable, evaluates the Expression and stores it in the environment with the provided name
        Let(&'a str, &'a Expr<'a>),
        // Creates a new mutable variable, evaluates the Expression and stores it in the environment with the provided name
        LetMut(&'a str, &'a Expr<'a>),
        // Add the two expressions together, behavior for pairs is undefined.
        Add(&'a Expr<'a>, &'a Expr<'a>),
        // Get the expression stored in the variable.
        Get(&'a str),
    }

    impl<'a> fmt::Display for Statement<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Statement::First(first) => write!(f, "Statement::First({})", first),
                Statement::Second(second) => write!(f, "Statement::Second({})", second),
                Statement::Scope(scope) => write!(f, "Statement::Scope({})", scope),
                Statement::Reference(r) => write!(f, "Statement::Reference({})", r),
                Statement::Let(var, val) => write!(f, "Statement::Let({}, {})", var, val),
                Statement::LetMut(var, val) => write!(f, "Statement::LetMut({}, {})", var, val),
                Statement::Add(a, b) => write!(f, "Statement::Add({}, {})", a, b),
                Statement::Get(a) => write!(f, "Statement::Get({})", a),
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Expr<'a> {
        Type(&'a Type<'a>),
        Statement(&'a Statement<'a>),
    }
    impl<'a> fmt::Display for Expr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Expr::Type(t) => write!(f, "Expr::{}", t),
                Expr::Statement(s) => write!(f, "Expr::{}", s),
            }
        }
    }
}
