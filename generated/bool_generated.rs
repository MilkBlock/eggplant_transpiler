use egglog::prelude::*;

fn main() {
    {
        let eq_check = egraph.check_equal(and(true, true), true);
        assert!(egraph.check_facts());
    }
    {
        let eq_check = egraph.check_equal(and(true, false), false);
        assert!(egraph.check_facts());
    }
    {
        let eq_check = egraph.check_equal(or(true, false), true);
        assert!(egraph.check_facts());
    }
    let _ = i;
    let _ = 0;
    let _ = 3;
}
