use bool_vec::BoolVec;

/// Stores a literal from a SAT instance problem.
/// A Literal is a Variable reference that may be negated, stored in a single isize for efficient storage.
///

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Literal(isize);

impl Literal {
    /// Creates a new literal from a variable index and a negation flag.
    pub fn new(var_index: usize, negated: bool) -> Self {
        assert!(var_index < isize::MAX as usize);

        let var_index = var_index as isize + 1;

        Self(if negated { -var_index } else { var_index })
    }

    /// Creates a new literal from a CNF representation.
    pub fn from_cnf(cnf: isize) -> Self {
        Self(cnf)
    }

    /// Returns the CNF representation of the literal.
    pub fn as_cnf(&self) -> isize {
        self.0
    }

    /// Returns the variable index of the literal.
    pub fn index(&self) -> usize {
        self.0.abs() as usize - 1
    }

    /// Returns whether the literal is negated.
    pub fn is_negated(&self) -> bool {
        self.0.is_negative()
    }

    /// Returns the negated literal.
    pub fn negated(&self) -> Literal {
        Self::from_cnf(-self.0)
    }

    /// Negates the literal in place.
    pub fn negate(&mut self) -> &Self {
        self.0 *= -1;
        self
    }

    /// Evaluates the literal with the given variable values (that is, possibly negated).
    /// Panics if the literal is not present in the given variables.
    /// Use `try_eval_with` for the faillible version.
    pub fn eval_with(&self, vars: &BoolVec) -> bool {
        self.try_eval_with(vars).unwrap()
    }

    /// Evaluates the literal with the given variable values (that is, possibly negated).
    /// Returns `None` if the literal is not present in the given variables.
    pub fn try_eval_with(&self, vars: &BoolVec) -> Option<bool> {
        vars.get(self.index()).map(|v| v ^ self.is_negated())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cnf() {
        let non_neg = Literal::from_cnf(1);
        let neg = Literal::from_cnf(-1);

        assert_eq!(non_neg.index(), 0);
        assert_eq!(non_neg.is_negated(), false);
        assert_eq!(neg.index(), 0);
        assert_eq!(neg.is_negated(), true);
    }

    #[test]
    fn new() {
        let non_neg = Literal::new(0, false);
        let neg = Literal::new(0, true);

        assert_eq!(non_neg.index(), 0);
        assert_eq!(non_neg.is_negated(), false);
        assert_eq!(neg.index(), 0);
        assert_eq!(neg.is_negated(), true);
    }

    #[test]
    #[should_panic]
    fn new_panic() {
        Literal::new(isize::MAX as usize, false);
    }

    #[test]
    fn new_max_index() {
        let non_neg = Literal::new((isize::MAX - 1) as usize, false);
        let neg = Literal::new((isize::MAX - 1) as usize, true);

        assert_eq!(non_neg.index(), (isize::MAX - 1) as usize);
        assert_eq!(neg.index(), (isize::MAX - 1) as usize);
    }
}
