// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/math.egg:2
// Datatype 'Math' defined with variants:
#[eggplant::dsl]
enum Math {
}

// Source: examples/math.egg:13
// Pattern variables for rule matching
// Variables: a, b
#[eggplant::pat_vars]
struct rule_13Pat {
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:14
// Pattern variables for rule matching
// Variables: a, b
#[eggplant::pat_vars]
struct rule_14Pat {
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:15
// Pattern variables for rule matching
// Variables: a, b, c
#[eggplant::pat_vars]
struct rule_15Pat {
    a: Expr,
    b: Expr,
    c: Expr,
}

// Source: examples/math.egg:16
// Pattern variables for rule matching
// Variables: a, b, c
#[eggplant::pat_vars]
struct rule_16Pat {
    a: Expr,
    b: Expr,
    c: Expr,
}

// Source: examples/math.egg:17
// Pattern variables for rule matching
// Variables: a, b
#[eggplant::pat_vars]
struct rule_17Pat {
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:18
// Pattern variables for rule matching
// Variables: a, b
#[eggplant::pat_vars]
struct rule_18Pat {
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:19
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_19Pat {
    a: Expr,
}

// Source: examples/math.egg:20
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_20Pat {
    a: Expr,
}

// Source: examples/math.egg:21
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_21Pat {
    a: Expr,
}

// Source: examples/math.egg:22
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_22Pat {
    a: Expr,
}

// Source: examples/math.egg:23
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_23Pat {
    a: Expr,
}

// Source: examples/math.egg:24
// Pattern variables for rule matching
// Variables: a, b, c
#[eggplant::pat_vars]
struct rule_24Pat {
    a: Expr,
    b: Expr,
    c: Expr,
}

// Source: examples/math.egg:25
// Pattern variables for rule matching
// Variables: a, b, c
#[eggplant::pat_vars]
struct rule_25Pat {
    a: Expr,
    b: Expr,
    c: Expr,
}

// Source: examples/math.egg:26
// Pattern variables for rule matching
// Variables: a, b, c
#[eggplant::pat_vars]
struct rule_26Pat {
    a: Expr,
    b: Expr,
    c: Expr,
}

// Source: examples/math.egg:27
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_27Pat {
    x: Expr,
}

// Source: examples/math.egg:28
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_28Pat {
    x: Expr,
}

// Source: examples/math.egg:29
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_29Pat {
    x: Expr,
}

// Source: examples/math.egg:30
// Pattern variables for rule matching
// Variables: x, -1
#[eggplant::pat_vars]
struct rule_30Pat {
    x: Expr,
    -1: Expr,
}

// Source: examples/math.egg:31
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_31Pat {
    x: Expr,
}

// Source: examples/math.egg:32
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_32Pat {
    x: Expr,
}

// Source: examples/math.egg:34
// Pattern variables for rule matching
// Variables: x, a, b
#[eggplant::pat_vars]
struct rule_34Pat {
    x: Expr,
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:35
// Pattern variables for rule matching
// Variables: x, a, b
#[eggplant::pat_vars]
struct rule_35Pat {
    x: Expr,
    a: Expr,
    b: Expr,
}

// Source: examples/math.egg:36
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_36Pat {
    x: Expr,
}

// Source: examples/math.egg:37
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_37Pat {
    x: Expr,
}

// Source: examples/math.egg:38
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_38Pat {
    x: Expr,
}

// Source: examples/math.egg:39
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_39Pat {
    x: Expr,
}

// Source: examples/math.egg:40
// Pattern variables for rule matching
// Variables: x, c
#[eggplant::pat_vars]
struct rule_40Pat {
    x: Expr,
    c: Expr,
}

// Source: examples/math.egg:41
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_41Pat {
    x: Expr,
}

// Source: examples/math.egg:42
// Pattern variables for rule matching
// Variables: x
#[eggplant::pat_vars]
struct rule_42Pat {
    x: Expr,
}

// Source: examples/math.egg:43
// Pattern variables for rule matching
// Variables: f, g, x
#[eggplant::pat_vars]
struct rule_43Pat {
    f: Expr,
    g: Expr,
    x: Expr,
}

// Source: examples/math.egg:44
// Pattern variables for rule matching
// Variables: f, g, x
#[eggplant::pat_vars]
struct rule_44Pat {
    f: Expr,
    g: Expr,
    x: Expr,
}

// Source: examples/math.egg:45
// Pattern variables for rule matching
// Variables: a, b, x
#[eggplant::pat_vars]
struct rule_45Pat {
    a: Expr,
    b: Expr,
    x: Expr,
}

fn main() {
    env_logger::init();
    // Source: examples/math.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/math.egg:13
    // Rule: rule_13
    MyTx::add_rule(
        "rule_13",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let add_node1 = Add::query(&a, &b);
rule_13Pat::new(a, b)
        },
        |ctx, pat| {
            let result = Add::new(pat.b, pat.a);
ctx.union(pat.rule_13_node1, result);
        },
    );
    
    // Source: examples/math.egg:14
    // Rule: rule_14
    MyTx::add_rule(
        "rule_14",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let mul_node1 = Mul::query(&a, &b);
rule_14Pat::new(a, b)
        },
        |ctx, pat| {
            let result = Mul::new(pat.b, pat.a);
ctx.union(pat.rule_14_node1, result);
        },
    );
    
    // Source: examples/math.egg:15
    // Rule: rule_15
    MyTx::add_rule(
        "rule_15",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let c = Expr::query_leaf();
let add_node1 = Add::query(&b, &c);
let add_node3 = Add::query(&a, &add_node2);
rule_15Pat::new(a, b, c)
        },
        |ctx, pat| {
            let result = Add::new(pat.add_node2, pat.c);
ctx.union(pat.rule_15_node1, result);
        },
    );
    
    // Source: examples/math.egg:16
    // Rule: rule_16
    MyTx::add_rule(
        "rule_16",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let c = Expr::query_leaf();
let mul_node1 = Mul::query(&b, &c);
let mul_node3 = Mul::query(&a, &mul_node2);
rule_16Pat::new(a, b, c)
        },
        |ctx, pat| {
            let result = Mul::new(pat.mul_node2, pat.c);
ctx.union(pat.rule_16_node1, result);
        },
    );
    
    // Source: examples/math.egg:17
    // Rule: rule_17
    MyTx::add_rule(
        "rule_17",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let sub_node1 = Sub::query(&a, &b);
rule_17Pat::new(a, b)
        },
        |ctx, pat| {
            let result = Add::new(pat.a, pat.mul_node2);
ctx.union(pat.rule_17_node1, result);
        },
    );
    
    // Source: examples/math.egg:18
    // Rule: rule_18
    MyTx::add_rule(
        "rule_18",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let div_node1 = Div::query(&a, &b);
rule_18Pat::new(a, b)
        },
        |ctx, pat| {
            let result = Mul::new(pat.a, pat.pow_node2);
ctx.union(pat.rule_18_node1, result);
        },
    );
    
    // Source: examples/math.egg:19
    // Rule: rule_19
    MyTx::add_rule(
        "rule_19",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let const_node1 = Const::query(0);
let add_node3 = Add::query(&a, &const_node2);
rule_19Pat::new(a)
        },
        |ctx, pat| {
            ctx.union(pat.rule_19_node1, pat.a);
        },
    );
    
    // Source: examples/math.egg:20
    // Rule: rule_20
    MyTx::add_rule(
        "rule_20",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let const_node1 = Const::query(0);
let mul_node3 = Mul::query(&a, &const_node2);
rule_20Pat::new(a)
        },
        |ctx, pat| {
            let result = Const::new(0);
ctx.union(pat.rule_20_node1, result);
        },
    );
    
    // Source: examples/math.egg:21
    // Rule: rule_21
    MyTx::add_rule(
        "rule_21",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let const_node1 = Const::query(1);
let mul_node3 = Mul::query(&a, &const_node2);
rule_21Pat::new(a)
        },
        |ctx, pat| {
            ctx.union(pat.rule_21_node1, pat.a);
        },
    );
    
    // Source: examples/math.egg:22
    // Rule: rule_22
    MyTx::add_rule(
        "rule_22",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let sub_node1 = Sub::query(&a, &a);
rule_22Pat::new(a)
        },
        |ctx, pat| {
            let result = Const::new(0);
ctx.union(pat.rule_22_node1, result);
        },
    );
    
    // Source: examples/math.egg:23
    // Rule: rule_23
    MyTx::add_rule(
        "rule_23",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let div_node1 = Div::query(&a, &a);
rule_23Pat::new(a)
        },
        |ctx, pat| {
            let result = Const::new(1);
ctx.union(pat.rule_23_node1, result);
        },
    );
    
    // Source: examples/math.egg:24
    // Rule: rule_24
    MyTx::add_rule(
        "rule_24",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let c = Expr::query_leaf();
let add_node1 = Add::query(&b, &c);
let mul_node3 = Mul::query(&a, &add_node2);
rule_24Pat::new(a, b, c)
        },
        |ctx, pat| {
            let result = Add::new(pat.mul_node2, pat.mul_node3);
ctx.union(pat.rule_24_node1, result);
        },
    );
    
    // Source: examples/math.egg:25
    // Rule: rule_25
    MyTx::add_rule(
        "rule_25",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let mul_node1 = Mul::query(&a, &b);
let c = Expr::query_leaf();
let mul_node2 = Mul::query(&a, &c);
let add_node5 = Add::query(&mul_node3, &mul_node4);
rule_25Pat::new(a, b, c)
        },
        |ctx, pat| {
            let result = Mul::new(pat.a, pat.add_node2);
ctx.union(pat.rule_25_node1, result);
        },
    );
    
    // Source: examples/math.egg:26
    // Rule: rule_26
    MyTx::add_rule(
        "rule_26",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let pow_node1 = Pow::query(&a, &b);
let c = Expr::query_leaf();
let pow_node2 = Pow::query(&a, &c);
let mul_node5 = Mul::query(&pow_node3, &pow_node4);
rule_26Pat::new(a, b, c)
        },
        |ctx, pat| {
            let result = Pow::new(pat.a, pat.add_node2);
ctx.union(pat.rule_26_node1, result);
        },
    );
    
    // Source: examples/math.egg:27
    // Rule: rule_27
    MyTx::add_rule(
        "rule_27",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let const_node1 = Const::query(0);
let pow_node3 = Pow::query(&x, &const_node2);
rule_27Pat::new(x)
        },
        |ctx, pat| {
            let result = Const::new(1);
ctx.union(pat.rule_27_node1, result);
        },
    );
    
    // Source: examples/math.egg:28
    // Rule: rule_28
    MyTx::add_rule(
        "rule_28",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let const_node1 = Const::query(1);
let pow_node3 = Pow::query(&x, &const_node2);
rule_28Pat::new(x)
        },
        |ctx, pat| {
            ctx.union(pat.rule_28_node1, pat.x);
        },
    );
    
    // Source: examples/math.egg:29
    // Rule: rule_29
    MyTx::add_rule(
        "rule_29",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let const_node1 = Const::query(2);
let pow_node3 = Pow::query(&x, &const_node2);
rule_29Pat::new(x)
        },
        |ctx, pat| {
            let result = Mul::new(pat.x, pat.x);
ctx.union(pat.rule_29_node1, result);
        },
    );
    
    // Source: examples/math.egg:30
    // Rule: rule_30
    MyTx::add_rule(
        "rule_30",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let -1 = Expr::query_leaf();
let const_node1 = Const::query(&-1);
let pow_node3 = Pow::query(&x, &const_node2);
rule_30Pat::new(x, -1)
        },
        |ctx, pat| {
            let result = Div::new(pat.const_node2, pat.x);
ctx.union(pat.rule_30_node1, result);
        },
    );
    
    // Source: examples/math.egg:31
    // Rule: rule_31
    MyTx::add_rule(
        "rule_31",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let const_node1 = Const::query(1);
let div_node3 = Div::query(&const_node2, &x);
let mul_node5 = Mul::query(&x, &div_node4);
rule_31Pat::new(x)
        },
        |ctx, pat| {
            let result = Const::new(1);
ctx.union(pat.rule_31_node1, result);
        },
    );
    
    // Source: examples/math.egg:32
    // Rule: rule_32
    MyTx::add_rule(
        "rule_32",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let diff_node1 = Diff::query(&x, &x);
rule_32Pat::new(x)
        },
        |ctx, pat| {
            let result = Const::new(1);
ctx.union(pat.rule_32_node1, result);
        },
    );
    
    // Source: examples/math.egg:34
    // Rule: rule_34
    MyTx::add_rule(
        "rule_34",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let a = Expr::query_leaf();
let b = Expr::query_leaf();
let add_node1 = Add::query(&a, &b);
let diff_node3 = Diff::query(&x, &add_node2);
rule_34Pat::new(x, a, b)
        },
        |ctx, pat| {
            let result = Add::new(pat.diff_node2, pat.diff_node3);
ctx.union(pat.rule_34_node1, result);
        },
    );
    
    // Source: examples/math.egg:35
    // Rule: rule_35
    MyTx::add_rule(
        "rule_35",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let a = Expr::query_leaf();
let b = Expr::query_leaf();
let mul_node1 = Mul::query(&a, &b);
let diff_node3 = Diff::query(&x, &mul_node2);
rule_35Pat::new(x, a, b)
        },
        |ctx, pat| {
            let result = Add::new(pat.mul_node2, pat.mul_node3);
ctx.union(pat.rule_35_node1, result);
        },
    );
    
    // Source: examples/math.egg:36
    // Rule: rule_36
    MyTx::add_rule(
        "rule_36",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let sin_node1 = Sin::query(&x);
let diff_node3 = Diff::query(&x, &sin_node2);
rule_36Pat::new(x)
        },
        |ctx, pat| {
            let result = Cos::new(pat.x);
ctx.union(pat.rule_36_node1, result);
        },
    );
    
    // Source: examples/math.egg:37
    // Rule: rule_37
    MyTx::add_rule(
        "rule_37",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let cos_node1 = Cos::query(&x);
let diff_node3 = Diff::query(&x, &cos_node2);
rule_37Pat::new(x)
        },
        |ctx, pat| {
            let result = Mul::new(pat.const_node2, pat.sin_node3);
ctx.union(pat.rule_37_node1, result);
        },
    );
    
    // Source: examples/math.egg:38
    // Rule: rule_38
    MyTx::add_rule(
        "rule_38",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let ln_node1 = Ln::query(&x);
let diff_node3 = Diff::query(&x, &ln_node2);
rule_38Pat::new(x)
        },
        |ctx, pat| {
            let result = Div::new(pat.const_node2, pat.x);
ctx.union(pat.rule_38_node1, result);
        },
    );
    
    // Source: examples/math.egg:39
    // Rule: rule_39
    MyTx::add_rule(
        "rule_39",
        default_ruleset,
        || {
            let const_node1 = Const::query(1);
let x = Expr::query_leaf();
let integral_node3 = Integral::query(&const_node2, &x);
rule_39Pat::new(x)
        },
        |ctx, pat| {
            ctx.union(pat.rule_39_node1, pat.x);
        },
    );
    
    // Source: examples/math.egg:40
    // Rule: rule_40
    MyTx::add_rule(
        "rule_40",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let c = Expr::query_leaf();
let pow_node1 = Pow::query(&x, &c);
let integral_node3 = Integral::query(&pow_node2, &x);
rule_40Pat::new(x, c)
        },
        |ctx, pat| {
            let result = Div::new(pat.pow_node2, pat.add_node3);
ctx.union(pat.rule_40_node1, result);
        },
    );
    
    // Source: examples/math.egg:41
    // Rule: rule_41
    MyTx::add_rule(
        "rule_41",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let cos_node1 = Cos::query(&x);
let integral_node3 = Integral::query(&cos_node2, &x);
rule_41Pat::new(x)
        },
        |ctx, pat| {
            let result = Sin::new(pat.x);
ctx.union(pat.rule_41_node1, result);
        },
    );
    
    // Source: examples/math.egg:42
    // Rule: rule_42
    MyTx::add_rule(
        "rule_42",
        default_ruleset,
        || {
            let x = Expr::query_leaf();
let sin_node1 = Sin::query(&x);
let integral_node3 = Integral::query(&sin_node2, &x);
rule_42Pat::new(x)
        },
        |ctx, pat| {
            let result = Mul::new(pat.const_node2, pat.cos_node3);
ctx.union(pat.rule_42_node1, result);
        },
    );
    
    // Source: examples/math.egg:43
    // Rule: rule_43
    MyTx::add_rule(
        "rule_43",
        default_ruleset,
        || {
            let f = Expr::query_leaf();
let g = Expr::query_leaf();
let add_node1 = Add::query(&f, &g);
let x = Expr::query_leaf();
let integral_node3 = Integral::query(&add_node2, &x);
rule_43Pat::new(f, g, x)
        },
        |ctx, pat| {
            let result = Add::new(pat.integral_node2, pat.integral_node3);
ctx.union(pat.rule_43_node1, result);
        },
    );
    
    // Source: examples/math.egg:44
    // Rule: rule_44
    MyTx::add_rule(
        "rule_44",
        default_ruleset,
        || {
            let f = Expr::query_leaf();
let g = Expr::query_leaf();
let sub_node1 = Sub::query(&f, &g);
let x = Expr::query_leaf();
let integral_node3 = Integral::query(&sub_node2, &x);
rule_44Pat::new(f, g, x)
        },
        |ctx, pat| {
            let result = Sub::new(pat.integral_node2, pat.integral_node3);
ctx.union(pat.rule_44_node1, result);
        },
    );
    
    // Source: examples/math.egg:45
    // Rule: rule_45
    MyTx::add_rule(
        "rule_45",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let b = Expr::query_leaf();
let mul_node1 = Mul::query(&a, &b);
let x = Expr::query_leaf();
let integral_node3 = Integral::query(&mul_node2, &x);
rule_45Pat::new(a, b, x)
        },
        |ctx, pat| {
            let result = Sub::new(pat.mul_node2, pat.integral_node3);
ctx.union(pat.rule_45_node1, result);
        },
    );
    
    // Source: examples/math.egg:46
    let start_expr2 = Add(Const(1), Sub(Var('a'), Mul(Sub(Const(2), Const(1)), Var('a'))));
    // Source: examples/math.egg:48
    let end_expr2 = Const(1);
    // Source: examples/math.egg:49
    // Assert: start_expr2 == end_expr2
    // Source: examples/math.egg:50
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/math.egg:50
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
