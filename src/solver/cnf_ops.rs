use dimacs::{Clause, Lit, Sign};

pub fn resolve(lhs: &Clause, rhs: &Clause, pivot_lit: &Lit) -> Clause {
    let mut resolved_lits = vec![];
    let neg_pivot_lit = negate_literal(pivot_lit);
    let lhs_pivot_lit = if lhs.lits().contains(pivot_lit) {
        *pivot_lit
    } else {
        neg_pivot_lit
    };
    let rhs_pivot_lit = if rhs.lits().contains(pivot_lit) {
        *pivot_lit
    } else {
        neg_pivot_lit
    };
    assert_ne!(lhs_pivot_lit, rhs_pivot_lit);
    let lhs_iters = lhs.lits().iter().filter(|&lit| lit != &lhs_pivot_lit);
    let rhs_iters = rhs.lits().iter().filter(|&lit| lit != &rhs_pivot_lit);
    for &lit in lhs_iters.chain(rhs_iters) {
        if resolved_lits.contains(&lit) {
            continue;
        }
        resolved_lits.push(lit);
    }
    Clause::from_vec(resolved_lits)
}

pub fn negate_literal(lit: &Lit) -> Lit {
    let neg_i64 = lit.var().to_u64() as i64 * if lit.sign() == Sign::Pos { -1 } else { 1 };
    Lit::from_i64(neg_i64)
}
