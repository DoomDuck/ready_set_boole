use core::fmt;
use std::io;

pub type Symbol = u8;

const SYMBOL_RANGE: core::ops::RangeInclusive<u8> = b'A'..=b'Z';

/// Expression representation
#[derive(Clone, Debug)]
pub enum Expression {
    Var(Symbol),
    Val(bool),
    Not(Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    Implies(Box<Expression>, Box<Expression>),
    Equivalent(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum ExpressionParseError {
    UnknownSymbol,
    MissingArgument,
    IncompleteComputation,
}

impl core::str::FromStr for Expression {
    type Err = ExpressionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Expression::*;
        use ExpressionParseError::*;

        type BExpr = Box<Expression>;
        type ParseResult<T> = Result<T, ExpressionParseError>;

        fn pop_arg(stack: &mut Vec<BExpr>) -> ParseResult<BExpr> {
            stack.pop().ok_or(MissingArgument)
        }

        fn build_bin_op(
            stack: &mut Vec<BExpr>,
            builder: fn(BExpr, BExpr) -> Expression,
        ) -> ParseResult<Expression> {
            let b = pop_arg(stack)?;
            let a = pop_arg(stack)?;
            Ok(builder(a, b))
        }

        let mut stack = Vec::new();
        let stack = &mut stack;
        for symbol in s.bytes() {
            let expression = match symbol {
                b'A'..=b'Z' => Var(symbol),
                b'0' => Val(false),
                b'1' => Val(true),
                b'!' => Not(pop_arg(stack)?),
                b'|' => build_bin_op(stack, Or)?,
                b'&' => build_bin_op(stack, And)?,
                b'^' => build_bin_op(stack, Xor)?,
                b'>' => build_bin_op(stack, Implies)?,
                b'=' => build_bin_op(stack, Equivalent)?,
                _ => return Err(UnknownSymbol),
            };
            stack.push(Box::new(expression))
        }
        if stack.len() != 1 {
            return Err(IncompleteComputation);
        }
        Ok(*stack.pop().unwrap())
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Expression::Var(symbol) => write!(f, "{}", symbol as char),
            Expression::Val(true) => write!(f, "1"),
            Expression::Val(false) => write!(f, "0"),
            Expression::Not(expr) => write!(f, "{expr}!"),
            Expression::Or(a, b) => write!(f, "{a}{b}|"),
            Expression::And(a, b) => write!(f, "{a}{b}&"),
            Expression::Xor(a, b) => write!(f, "{a}{b}^"),
            Expression::Implies(a, b) => write!(f, "{a}{b}>"),
            Expression::Equivalent(a, b) => write!(f, "{a}{b}="),
        }
    }
}

impl Expression {
    pub fn variable(symbol: Symbol) -> Self {
        Self::Var(symbol)
    }

    pub fn value(value: bool) -> Self {
        Self::Val(value)
    }

    pub fn not(expr: impl Into<Box<Expression>>) -> Self {
        Self::Not(expr.into())
    }

    pub fn or(a: impl Into<Box<Expression>>, b: impl Into<Box<Expression>>) -> Self {
        Self::Or(a.into(), b.into())
    }

    pub fn and(a: impl Into<Box<Expression>>, b: impl Into<Box<Expression>>) -> Self {
        Self::And(a.into(), b.into())
    }

    pub fn xor(a: impl Into<Box<Expression>>, b: impl Into<Box<Expression>>) -> Self {
        Self::Xor(a.into(), b.into())
    }

    pub fn implies(a: impl Into<Box<Expression>>, b: impl Into<Box<Expression>>) -> Self {
        Self::Implies(a.into(), b.into())
    }

    pub fn equivalent(a: impl Into<Box<Expression>>, b: impl Into<Box<Expression>>) -> Self {
        Self::Equivalent(a.into(), b.into())
    }

    pub fn envs(&self) -> impl Iterator<Item=Environment> {
        pub fn rec(expr: &Expression, env: &mut Environment) {
            use Expression::*;
            match expr {
                Val(_value) => {}
                Var(symbol) => env.enable(*symbol),
                Not(expr) => rec(expr, env),
                Or(a, b) | And(a, b) | Xor(a, b) | Implies(a, b) | Equivalent(a, b) => {
                    rec(a, env);
                    rec(b, env);
                }
            }
        }
        let mut env = Environment::default();
        rec(self, &mut env);

        std::iter::from_fn(move || {
            (env.mask != 0).then(|| {
                let value = env;
                match (env.values | !env.mask).checked_add(1) {
                    Some(x) => env.values = x & env.mask,
                    None => env.mask = 0,
                }
                value
            })
        })
    }

    pub fn eval(&self, env: Environment) -> bool {
        use Expression::*;
        match self {
            Var(symbol) => env.get(*symbol).unwrap(),
            Val(value) => *value,
            Not(expr) => !expr.eval(env),
            Or(a, b) => a.eval(env) || b.eval(env),
            And(a, b) => a.eval(env) && b.eval(env),
            Xor(a, b) => a.eval(env) ^ b.eval(env),
            Implies(a, b) => a.eval(env) <= b.eval(env),
            Equivalent(a, b) => a.eval(env) == b.eval(env),
        }
    }

    pub fn negation_normal(&self) -> Expression {
        use Expression::*;

        fn norm(expr: &Expression) -> Expression {
            match expr {
                Var(_) | Val(_) => Expression::clone(expr),
                Or(a, b) => norm(a) | norm(b),
                And(a, b) => norm(a) & norm(b),
                Not(x) => neg(x),
                Xor(a, b) => norm(a) & neg(b) | neg(a) & norm(b),
                Implies(a, b) => neg(a) | norm(b),
                Equivalent(a, b) => norm(a) & norm(b) | neg(a) & neg(b),
            }
        }

        fn neg(expr: &Expression) -> Expression {
            match expr {
                Var(s) => !Var(*s),
                Val(x) => Expression::value(!x),
                Not(x) => norm(x),
                Or(a, b) => neg(a) & neg(b),
                And(a, b) => neg(a) | neg(b),
                Xor(a, b) => norm(a) & norm(b) | neg(a) & neg(b),
                Implies(a, b) => norm(a) & neg(b),
                Equivalent(a, b) => norm(a) & neg(b) | neg(a) & norm(b),
            }
        }
        norm(self)
    }

    pub fn conjonctive_normal(&self) -> Expression {
        use Expression::*;

        #[inline]
        fn cl(expr: &Expression) -> Expression {
            Expression::clone(expr)
        }

        fn norm(expr: &Expression) -> Expression {
            match expr {
                Var(_) | Val(_) => Expression::clone(expr),
                Not(x) => neg(x),
                Or(a, b) => or(cl(a), cl(b)),
                And(a, b) => and(norm(a), norm(b)),
                Xor(a, b) => and(or(norm(a), norm(b)), or(neg(a), neg(b))),
                Implies(a, b) => or(neg(a), norm(b)),
                Equivalent(a, b) => and(or(norm(a), neg(b)), or(neg(a), norm(b))),
            }
        }

        fn neg(expr: &Expression) -> Expression {
            match expr {
                Var(s) => !Var(*s),
                Val(x) => Expression::value(!x),
                Not(x) => norm(x),
                Or(a, b) => and(neg(a), neg(b)),
                And(a, b) => or(neg(a), neg(b)),
                Xor(a, b) => and(or(neg(a), norm(b)), or(norm(a), neg(b))),
                Implies(a, b) => and(norm(a), neg(b)),
                Equivalent(a, b) => and(or(neg(a), neg(a)), or(norm(a), norm(b))),
            }
        }

        fn or(a: Expression, b: Expression) -> Expression {
            match (a, b) {
                (And(x, y), b) => and(or(*x, cl(&b)), or(*y, b)),
                (a, And(x, y)) => and(or(cl(&a), *x), or(a, *y)),
                (a, b) => match (norm(&a), norm(&b)) {
                    (And(_, _), _) | (_, And(_, _)) => or(a, b),
                    (Or(a, b), c) => or(*a, or(*b, c)),
                    (a, b) => a | b,
                },
            }
        }

        fn and(a: Expression, b: Expression) -> Expression {
            match (a, b) {
                (And(a, b), c) => and(*a, and(*b, c)),
                (a, b) => a & b,
            }
        }

        norm(self)
    }

    pub fn sat(&self) -> bool {
        self.envs().any(|env| self.eval(env))
    }

    pub fn write_truth_table(&self, output: &mut impl io::Write) -> io::Result<()> {
        let mut envs = self.envs().peekable();
        let Some(first_env) = envs.peek() else {
            return Ok(());
        };
        write!(output, "|")?;
        let mut count = 1;
        for symbol in first_env.symbols() {
            count += 1;
            write!(output, " {} |", symbol as char)?;
        }
        write!(output, " = |\n|")?;
        for _ in 0..count {
            write!(output, "---|")?;
        }
        writeln!(output, "")?;
        for env in envs {
            write!(output, "|")?;
            for value in env.values() {
                write!(output, " {} |", value as u8)?;
            }
            writeln!(output, " {} |", self.eval(env) as u8)?;
        }
        Ok(())
    }
}

impl core::ops::Not for Expression {
    type Output = Expression;

    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl core::ops::BitOr for Expression {
    type Output = Expression;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Or(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::BitAnd for Expression {
    type Output = Expression;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::And(Box::new(self), Box::new(rhs))
    }
}

impl core::ops::BitXor for Expression {
    type Output = Expression;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::Xor(Box::new(self), Box::new(rhs))
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Environment {
    values: u32,
    mask: u32,
}

impl Environment {
    pub fn enable(&mut self, symbol: Symbol) {
        debug_assert!(SYMBOL_RANGE.contains(&symbol));
        self.mask |= 1 << symbol.wrapping_sub(b'A');
    }

    pub fn get(&self, symbol: Symbol) -> Option<bool> {
        let index = symbol.wrapping_sub(b'A');
        ((self.mask >> index) & 1 != 0).then_some((self.values >> index) & 1 != 0)
    }

    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        SYMBOL_RANGE.filter(|&s| (self.mask >> (s - b'A')) & 1 != 0)
    }

    pub fn values(&self) -> impl Iterator<Item = bool> + '_ {
        (0..u32::BITS)
            .filter(|i| (self.mask >> i) & 1 != 0)
            .map(|i| (self.values >> i) & 1 != 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::Expression;

    #[test]
    fn write_truth_table() {
        let expression: Expression = "AB|C&!".parse().unwrap();
        let mut output = Vec::new();
        expression.write_truth_table(&mut output).unwrap();
        let text = std::str::from_utf8(&output).unwrap();
        assert_eq!(
            text, "\
            | A | B | C | = |\n\
            |---|---|---|---|\n\
            | 0 | 0 | 0 | 1 |\n\
            | 1 | 0 | 0 | 1 |\n\
            | 0 | 1 | 0 | 1 |\n\
            | 1 | 1 | 0 | 1 |\n\
            | 0 | 0 | 1 | 1 |\n\
            | 1 | 0 | 1 | 0 |\n\
            | 0 | 1 | 1 | 0 |\n\
            | 1 | 1 | 1 | 0 |\n"
        );
    }

    #[test]
    fn negation_normal() {
        fn check(input: &str, expected: &str) {
            let input: Expression = input.parse().unwrap();
            assert_eq!(input.negation_normal().to_string(), expected);
        }
        check("AB&!", "A!B!|");
        check("AB|!", "A!B!&");
        check("AB>", "A!B|");
        check("AB=", "AB&A!B!&|");
        check("AB|C&!", "A!B!&C!|");
    }

    #[test]
    pub fn conjonction_normal() {
        fn check(input: &str, expected: &str) {
            let input: Expression = input.parse().unwrap();
            assert_eq!(input.conjonctive_normal().to_string(), expected);
        }
        check("AB&!", "A!B!|");
        check("AB|!", "A!B!&");
        check("AB|C&", "AB|C&");
        check("AB|C|D|", "ABCD|||");
        check("AB&C&D&", "ABCD&&&");
        check("AB&!C!|", "A!B!C!||");
        check("AB|!C!&", "A!B!C!&&");
    }

    #[test]
    fn sat() {
        fn sat(formula: &str) -> bool {
            let expr: Expression = formula.parse().unwrap();
            expr.sat()
        }
        assert_eq!(sat("AB|"), true);
        assert_eq!(sat("AB&"), true);
        assert_eq!(sat("AA!&"), false);
        assert_eq!(sat("AA^"), false);
    }
}
