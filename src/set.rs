use core::fmt;

#[derive(Debug, Clone)]
pub struct Set<T>(Vec<T>);

impl<T> Default for Set<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T: Clone> Set<&T> {
    pub fn cloned(&self) -> Set<T> {
        Set(self.0.iter().map(|&e| e.clone()).collect())
    }
}

impl<T: fmt::Display> fmt::Display for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut elements = self.0.iter();
        if let Some(first) = elements.next() {
            write!(f, " {first}")?;
        }
        for element in elements {
            write!(f, ", {element}")?;
        }
        write!(f, " }}")
    }
}

impl<T: Eq> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0.iter().all(|x| other.0.contains(x))
    }
}

impl<T: Eq> PartialEq<Set<&T>> for Set<T> {
    fn eq(&self, other: &Set<&T>) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        other.0.iter().all(|&x| self.0.contains(x))
    }
}

impl<T: Eq> PartialEq<Set<T>> for Set<&T> {
    fn eq(&self, other: &Set<T>) -> bool {
        other.eq(self)
    }
}

impl<T: Eq> Eq for Set<T> {}

fn has_duplicates(mut set: &[impl Eq]) -> bool {
    while let Some((first, rest)) = set.split_first() {
        if rest.iter().any(|v| first == v) {
            return true;
        }
        set = rest;
    }
    false
}

impl<T: Eq> TryFrom<Vec<T>> for Set<T> {
    type Error = Vec<T>;

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match has_duplicates(&vec) {
            true => Err(vec),
            false => Ok(Self(vec)),
        }
    }
}

impl<T> From<Set<T>> for Vec<T> {
    fn from(set: Set<T>) -> Self {
        set.0
    }
}

impl<T> Set<T> {
    pub fn powerset<'a>(&'a self) -> impl Iterator<Item = Set<&'a T>> {
        (0..1 << self.0.len()).map(|mut i| {
            Set(self
                .0
                .iter()
                .filter(|_| {
                    let keep = i & 1 != 0;
                    i >>= 1;
                    keep
                })
                .collect())
        })
    }
}

impl<T: Eq + Clone> Set<T> {
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for value in &other.0 {
            if !result.0.contains(value) {
                result.0.push(value.clone())
            }
        }
        result
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|x| other.0.contains(x))
                .cloned()
                .collect(),
        )
    }

    pub fn xunion(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|x| !other.0.contains(x))
                .chain(other.0.iter().filter(|x| !self.0.contains(x)))
                .cloned()
                .collect(),
        )
    }

    pub fn without(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|x| !other.0.contains(x))
                .cloned()
                .collect(),
        )
    }
}

#[derive(Debug)]
pub enum EvaluationError {
    MissingArgument,
    IncompleteComputation,
    UnspecifiedVar,
    UnknownSymbol,
}

pub fn try_evaluate<T: Eq + Clone>(
    expression: &str,
    environment: Vec<Set<T>>,
) -> Result<Set<T>, EvaluationError> {
    use EvaluationError::*;
    let mut stack = Vec::<Set<T>>::new();
    let mut all = Set::default();
    for set in &environment {
        all = all.union(set);
    }
    for symbol in expression.bytes() {
        let result = match symbol {
            b'A'..=b'Z' => {
                let index = (symbol - b'A') as usize;
                let set = environment.get(index).ok_or(UnspecifiedVar)?;
                set.clone()
            }
            b'!' => all.without(&stack.pop().ok_or(MissingArgument)?),
            b'|' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a.union(&b)
            }
            b'&' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a.intersection(&b)
            }
            b'^' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                a.xunion(&b)
            }
            b'>' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                all.without(&a.without(&b))
            }
            b'=' => {
                let b = stack.pop().ok_or(MissingArgument)?;
                let a = stack.pop().ok_or(MissingArgument)?;
                all.without(&a.intersection(&b))
            }
            _ => return Err(UnknownSymbol),
        };
        stack.push(result)
    }
    if stack.len() != 1 {
        return Err(IncompleteComputation);
    }
    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn powerset() {
        fn check(input: &[i32], expected: &[&[i32]]) {
            let set = Set::try_from(input.to_vec()).unwrap();
            let expected = expected.iter().map(|s| Set::try_from(s.to_vec()).unwrap());
            assert!(set.powerset().eq(expected));
        }

        check(&[], &[&[]]);
        check(&[1], &[&[], &[1]]);
        check(&[1, 2], &[&[], &[1], &[2], &[1, 2]]);
        check(
            &[1, 2, 3],
            &[&[], &[1], &[2], &[1, 2], &[3], &[1, 3], &[2, 3], &[1, 2, 3]],
        )
    }

    #[test]
    fn evaluate() {
        fn check<T: Eq + Clone + std::fmt::Debug + Eq>(
            expression: &str,
            input: &[&[T]],
            expected: &[T],
        ) {
            let input: Vec<_> = input
                .iter()
                .map(|s| Set::try_from(s.to_vec()).unwrap())
                .collect();
            let expected = Set::try_from(expected.to_vec()).unwrap();
            let output = super::try_evaluate(expression, input).unwrap();
            assert_eq!(output, expected);
        }

        check("AB&", &[&[0, 1, 2], &[0, 3, 4]], &[0]);

        check("AB|", &[&[0, 1, 2], &[3, 4, 5]], &[0, 1, 2, 3, 4, 5]);

        check("A!", &[&[0, 1, 2]], &[]);
    }
}
