use egglog::prelude::*;

fn main() {
    let Math = egglog::Datatype::new("Math");
    let Diff = Math.add_variant("Diff", vec![]);
    let Integral = Math.add_variant("Integral", vec![]);
    let Add = Math.add_variant("Add", vec![]);
    let Sub = Math.add_variant("Sub", vec![]);
    let Mul = Math.add_variant("Mul", vec![]);
    let Div = Math.add_variant("Div", vec![]);
    let Pow = Math.add_variant("Pow", vec![]);
    let Ln = Math.add_variant("Ln", vec![]);
    let Sqrt = Math.add_variant("Sqrt", vec![]);
    let Sin = Math.add_variant("Sin", vec![]);
    let Cos = Math.add_variant("Cos", vec![]);
    let Const = Math.add_variant("Const", vec![]);
    let Var = Math.add_variant("Var", vec![]);
    let _ = e;
    let _ = e;
    let _ = e;
    let _ = e;
    let _ = e;
    let _ = e;
    let _ = e;
    let _ = x;
    let _ = Math;
    let _ = v;
    let _ = vv;
    let _ = vw;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Add(a, b);
        let rhs = Add(b, a);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Mul(a, b);
        let rhs = Mul(b, a);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Add(a, Add(b, c));
        let rhs = Add(Add(a, b), c);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Mul(a, Mul(b, c));
        let rhs = Mul(Mul(a, b), c);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Sub(a, b);
        let rhs = Add(a, Mul(Const(-1), b));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Div(a, b);
        let rhs = Mul(a, Pow(b, Const(-1)));
        egraph.union(lhs, rhs);
    });
    let _ = 0;
    let _ = 0;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Mul(a, Const(1));
        let rhs = a;
        egraph.union(lhs, rhs);
    });
    let _ = 0;
    let _ = 1;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Sub(a, a);
        let rhs = Const(0);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Div(a, a);
        let rhs = Const(1);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Add(Mul(a, b), Mul(a, c));
        let rhs = Mul(a, Add(b, c));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Mul(Pow(a, b), Pow(a, c));
        let rhs = Pow(a, Add(b, c));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Pow(x, Const(0));
        let rhs = Const(1);
        egraph.union(lhs, rhs);
    });
    let _ = 1;
    let _ = 2;
    let _ = -1;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Mul(x, Div(Const(1), x));
        let rhs = Const(1);
        egraph.union(lhs, rhs);
    });
    let _ = 1;
    let _ = x;
    let _ = 0;
    let _ = x;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Diff(x, Add(a, b));
        let rhs = Add(Diff(x, a), Diff(x, b));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Diff(x, Mul(a, b));
        let rhs = Add(Mul(a, Diff(x, b)), Mul(b, Diff(x, a)));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Diff(x, Sin(x));
        let rhs = Cos(x);
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Diff(x, Cos(x));
        let rhs = Mul(Const(-1), Sin(x));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Diff(x, Ln(x));
        let rhs = Div(Const(1), x);
        egraph.union(lhs, rhs);
    });
    let _ = f;
    let _ = g;
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Const(1), x);
        let rhs = x;
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Pow(x, c), x);
        let rhs = Div(Pow(x, Add(c, Const(1))), Add(c, Const(1)));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Sin(x), x);
        let rhs = Mul(Const(-1), Cos(x));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Add(f, g), x);
        let rhs = Add(Integral(f, x), Integral(g, x));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Sub(f, g), x);
        let rhs = Sub(Integral(f, x), Integral(g, x));
        egraph.union(lhs, rhs);
    });
    let default_rewrite = default::add_rewrite("default", |egraph| {
        let lhs = Integral(Mul(a, b), x);
        let rhs = Sub(Mul(a, Integral(b, x)), Integral(Mul(Diff(x, a), Integral(b, x)), x));
        egraph.union(lhs, rhs);
    });
    let start-expr2 = Add(Const(1), Sub(Var("a".into()), Mul(Sub(Const(2), Const(1)), Var("a".into()))));
    let _ = 6;
    let end-expr2 = Const(1);
    {
        let eq_check = egraph.check_equal(start-expr2, end-expr2);
        assert!(egraph.check_facts());
    }
    let _ = start-expr2;
}
