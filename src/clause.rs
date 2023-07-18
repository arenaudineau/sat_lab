use std::fmt;

use bool_vec::BoolVec;

use crate::literal::*;

/// A Clause is a set of Literals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause(Vec<Literal>);

impl Clause {
    /// Creates a new clause from a collection of literals.
    pub fn new<T>(elems: T) -> Self
    where
        T: ToOwned<Owned = Vec<Literal>>,
    {
        // TODO: weird input type?
        Self(elems.to_owned())
    }

    /// Creates a new clause from a collection of variable indices and negation flags.
    pub fn from_indices<I, V>(var_indices: I, negates: V) -> Self
    where
        I: IntoIterator<Item = usize>,
        V: IntoIterator<Item = bool>,
    {
        std::iter::zip(var_indices.into_iter(), negates.into_iter())
            .map(|(i, n)| Literal::new(i, n))
            .collect()
    }

    /// Creates a new clause from a collection of CNF representations.
    pub fn from_cnf<I>(cnfs: I) -> Self
    where
        I: IntoIterator<Item = isize>,
    {
        cnfs.into_iter().map(Literal::from_cnf).collect()
    }

    /// Returns a clause with the literals negated.
    /// For inplace negation, use `negate`.
    pub fn negated(&self) -> Self {
        self.0.iter().map(|elem| elem.negated()).collect()
    }

    /// Negates the clause with the literals negated in place.
    /// For a non-inplace version, use `negated`.
    pub fn negate(&mut self) -> &Self {
        for elem in &mut self.0 {
            elem.negate();
        }

        self
    }

    /// Returns an iterator over the variables evaluated (that is, possibly negated)
    pub fn iter_eval<'a>(&'a self, vars: &'a BoolVec) -> impl Iterator<Item = bool> + 'a {
        self.0.iter().map(|elem| elem.eval_with(vars))
    }

    /// Returns an iterator over the negated variables evaluated (that is, possibly (un-)negated)
    pub fn iter_eval_negated<'a>(&'a self, vars: &'a BoolVec) -> impl Iterator<Item = bool> + 'a {
        self.0.iter().map(|elem| elem.negated().eval_with(vars))
    }

    /// Returns whether the clause is satisfied by the given variable values.
    pub fn test_sat(&self, vars: &BoolVec) -> bool {
        self.iter_eval(vars).any(|x| x)
    }

    /// Returns the Literals
    pub fn get_literals(&self) -> &[Literal] {
        &self.0
    }
}

impl FromIterator<Literal> for Clause {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Literal>,
    {
        Self(iter.into_iter().collect())
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, elem) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " ∨ ")?;
            }
            if elem.is_negated() {
                write!(f, "¬")?;
            }
            write!(f, "x{}", elem.index())?;
        }
        write!(f, ")")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bool_vec::BoolVec;

    #[test]
    fn construction() {
        let vref1 = Literal::new(0, false);
        let vref2 = Literal::new(1, true);

        let clause = Clause::new(vec![vref1, vref2]);
        let same_clause = Clause::from_indices(
            vec![vref1.index(), vref2.index()],
            vec![vref1.is_negated(), vref2.is_negated()],
        );
        let again_same_clause = Clause::from_cnf(vec![vref1.as_cnf(), vref2.as_cnf()]);

        assert_eq!(clause, same_clause);
        assert_eq!(clause, again_same_clause);
        assert_eq!(again_same_clause, clause); // Should be ok, but for reflexive property
    }

    #[test]
    fn operations() {
        let clause = Clause::from_cnf(vec![1, -2, 3]);
        let negated_clause = clause.negated();

        assert_eq!(negated_clause, Clause::from_cnf(vec![-1, 2, -3]));

        let mut bv = BoolVec::from([false, false, false]);
        assert!(clause.test_sat(&bv));
        bv.set(1, true).unwrap();
        assert!(!clause.test_sat(&bv));
        bv.set(2, true).unwrap();
        assert!(clause.test_sat(&bv));

        assert!(
            std::iter::zip(clause.iter_eval(&bv), clause.iter_eval_negated(&bv))
                .map(|(x, nx)| x != nx)
                .all(|x| x)
        );
    }
}
