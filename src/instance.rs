use crate::{clause::Clause, literal::Literal};

use bool_vec::{boolvec, BoolVec};

#[cfg(feature = "rand")]
use rand::{distributions::Standard, Rng, SeedableRng};

use std::{fs, io::Write, path::Path};

/// A SAT instance
#[derive(Debug)]
pub struct Instance {
    pub vars: BoolVec,
    clauses: Vec<Clause>,
}

impl Instance {
    /// Creates a new instance with the given variables and clauses.
    pub fn new(vars: BoolVec, clauses: Vec<Clause>) -> Self {
        Self { vars, clauses }
    }

    /// Creates a new instance with the given number of variables initialized at 0 and clauses.
    pub fn with_clauses(n: usize, clauses: Vec<Clause>) -> Self {
        Self {
            vars: boolvec![false; n],
            clauses,
        }
    }

    /// Creates a new instance from a file in Conjunctive Normal Form.
    /// Returns an error if the file is not in CNF or is malformed.
    pub fn from_file<P>(path: P) -> std::io::Result<Self>
    where
        P: AsRef<Path>,
    {
        // TODO: Custom errors

        let content = fs::read_to_string(path)?;

        let mut lines = content.trim().lines().skip_while(|x| x.starts_with('c'));

        let mut param_line = lines
            .next()
            .ok_or(std::io::ErrorKind::InvalidInput)?
            .split_whitespace()
            .skip(1);

        let problem_type = param_line.next().ok_or(std::io::ErrorKind::InvalidInput)?;
        if problem_type != "cnf" {
            return Err(std::io::ErrorKind::InvalidInput.into());
        }

        let n = param_line
            .next()
            .ok_or(std::io::ErrorKind::InvalidInput)?
            .parse()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;
        let m = param_line
            .next()
            .ok_or(std::io::ErrorKind::InvalidInput)?
            .parse()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        let clauses = lines
            .take(m)
            .map(str::split_whitespace)
            .map(|clause| {
                clause
                    .map(|x| x.parse())
                    .take_while(|r| r.as_ref().map_or(false, |x| *x != 0))
                    .map(|r| r.map(Literal::from_cnf))
                    .collect::<Result<Clause, _>>()
            })
            .collect::<Result<_, _>>()
            .map_err(|_| std::io::ErrorKind::InvalidInput)?;

        Ok(Self {
            vars: boolvec![false; n],
            clauses,
        })
    }

    /// Creates a new random instance with the given number of variables, clauses, and clause length.
    #[cfg(feature = "rand")]
    pub fn new_random<R: Rng + SeedableRng>(n: usize, m: usize, k: usize) -> Self {
        let mut rng = R::from_entropy();

        let mut chosen_indices = vec![];

        let vars = BoolVec::from((&mut rng).sample_iter(Standard).take(n).collect::<Vec<_>>());
        let clauses = (0..m)
            .map(|_| {
                let mut var_indices: Vec<usize> = vec![0; k];
                let mut negates = boolvec![false; k];

                for i in 0..k {
                    let mut idx = rng.gen_range(0..n);
                    while chosen_indices.contains(&idx) {
                        idx = rng.gen_range(0..n);
                    }
                    chosen_indices.push(idx);

                    var_indices[i] = idx;
                    negates.set(i, rng.gen());
                }
                chosen_indices.clear();

                Clause::from_indices(var_indices, &negates)
            })
            .collect();

        Self { vars, clauses }
    }

    /// Save the instance to a file in Conjunctive Normal Form.
    pub fn to_file<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = fs::File::create(path)?;

        writeln!(file, "p cnf {} {}", self.vars.len(), self.clauses.len())?;
        for clause in &self.clauses {
            for elem in clause.get_literals() {
                write!(file, "{} ", elem.as_cnf())?;
            }
            writeln!(file, "0")?;
        }

        Ok(())
    }

    /// Randomly sample new variables
    #[cfg(feature = "rand")]
    pub fn sample_new_variables(&mut self) -> &BoolVec {
        let mut rng = rand::thread_rng();
        self.sample_new_variables_with(&mut rng)
    }

    /// Randomly sample new variables with a given RNG
    #[cfg(feature = "rand")]
    pub fn sample_new_variables_with<T: Rng>(&mut self, rng: &mut T) -> &BoolVec {
        self.vars = BoolVec::from(
            rng.sample_iter(Standard)
                .take(self.vars.len())
                .collect::<Vec<_>>(),
        );

        &self.vars
    }

    /// Returns the number of satisfied clauses
    pub fn count_sat(&self) -> usize {
        self.clauses
            .iter()
            .filter(|clause| clause.test_sat(&self.vars))
            .count()
    }

    /// Returns true if all clauses are satisfied
    pub fn is_sat(&self) -> bool {
        self.count_sat() == self.clauses.len()
    }

    /// Returns the ratio of clauses to variables
    pub fn clause_to_vars(&self) -> f32 {
        self.clauses.len() as f32 / self.vars.len() as f32
    }

    /// Returns a reference to the clauses
    pub fn get_clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
