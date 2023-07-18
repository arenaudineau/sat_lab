use sat_lab::*;

fn main() {
    let mut instance = Instance::from_file("circuit_equiv_check.cnf").unwrap();
    instance.vars.negate(324).unwrap();
    dbg!(
        instance.count_sat() as f32,
        instance.get_clauses().len() as f32,
    );
}
