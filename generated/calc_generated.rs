use egglog::prelude::*;

fn main() {
    let G = egglog::Datatype::new("G");
    let IConst = egglog::Constructor::new("IConst", vec![], "G".into());
    let I = IConst();
    let AConst = egglog::Constructor::new("AConst", vec![], "G".into());
    let A = AConst();
    let BConst = egglog::Constructor::new("BConst", vec![], "G".into());
    let B = BConst();
    let g* = egglog::Constructor::new("g*", vec!["G".into(), "G".into()], "G".into());
    let inv = egglog::Constructor::new("inv", vec!["G".into()], "G".into());
    let default_birewrite = default::add_birewrite("default", |egraph| {
        let lhs = g*(g*(a, b), c);
        let rhs = g*(a, g*(b, c));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = g*(I, a);
        let rhs = a;
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = g*(a, I);
        let rhs = a;
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = g*(inv(a), a);
        let rhs = I;
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = g*(a, inv(a));
        let rhs = I;
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = g*(A, g*(A, g*(A, A)));
        let rhs = I;
        egraph.union(lhs, rhs);
    });
    let A2 = g*(A, A);
    let A4 = g*(A2, A2);
    let A8 = g*(A4, A4);
    {
        let eq_check = egraph.check_equal(g*(A4, A4), g*(g*(A2, A2), g*(A2, A2)));
        assert!(egraph.check_facts());
    }
    let aConst = egglog::Constructor::new("aConst", vec![], "G".into());
    let bConst = egglog::Constructor::new("bConst", vec![], "G".into());
    let a = aConst();
    let b = bConst();
    let _ = b;
    let _ = b;
    {
        let eq_check = egraph.check_equal(g*(b, inv(b)), I);
        assert!(egraph.check_facts());
    }
}
