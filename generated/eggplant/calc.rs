// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/calc.egg:2
// Datatype 'G' defined with variants:
//   - IConst: variant (defined at examples/calc.egg:3)
//   - AConst: variant (defined at examples/calc.egg:5)
//   - BConst: variant (defined at examples/calc.egg:7)
//   - g_: variant (defined at examples/calc.egg:9)
//   - inv: variant (defined at examples/calc.egg:10)
//   - aConst: variant (defined at examples/calc.egg:22)
//   - bConst: variant (defined at examples/calc.egg:23)
#[eggplant::dsl]
enum G {
    IConst {},
    AConst {},
    BConst {},
    g_ { 
        arg0: G,
        arg1: G,
    },
    inv { 
        arg0: G,
    },
    aConst {},
    bConst {},
}

// Source: examples/calc.egg:12
// Pattern variables for rule matching
// Variables: I, a
#[eggplant::pat_vars]
struct rule_12Pat {
    I: Expr,
    a: Expr,
}

// Source: examples/calc.egg:13
// Pattern variables for rule matching
// Variables: a, I
#[eggplant::pat_vars]
struct rule_13Pat {
    a: Expr,
    I: Expr,
}

// Source: examples/calc.egg:14
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_14Pat {
    a: Expr,
}

// Source: examples/calc.egg:15
// Pattern variables for rule matching
// Variables: a
#[eggplant::pat_vars]
struct rule_15Pat {
    a: Expr,
}

// Source: examples/calc.egg:16
// Pattern variables for rule matching
// Variables: A
#[eggplant::pat_vars]
struct rule_16Pat {
    A: Expr,
}

fn main() {
    env_logger::init();
    // Source: examples/calc.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/calc.egg:4
    let I = IConst();
    // Source: examples/calc.egg:6
    let A = AConst();
    // Source: examples/calc.egg:8
    let B = BConst();
    // Source: examples/calc.egg:12
    // Rule: rule_12
    MyTx::add_rule(
        "rule_12",
        default_ruleset,
        || {
            let I = Expr::query_leaf();
let a = Expr::query_leaf();
let g__node1 = g_::query(&I, &a);
rule_12Pat::new(I, a)
        },
        |ctx, pat| {
            ctx.union(pat.rule_12_node1, pat.a);
        },
    );
    
    // Source: examples/calc.egg:13
    // Rule: rule_13
    MyTx::add_rule(
        "rule_13",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let I = Expr::query_leaf();
let g__node1 = g_::query(&a, &I);
rule_13Pat::new(a, I)
        },
        |ctx, pat| {
            ctx.union(pat.rule_13_node1, pat.a);
        },
    );
    
    // Source: examples/calc.egg:14
    // Rule: rule_14
    MyTx::add_rule(
        "rule_14",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let inv_node1 = inv::query(&a);
let g__node3 = g_::query(&inv_node2, &a);
rule_14Pat::new(a)
        },
        |ctx, pat| {
            ctx.union(pat.rule_14_node1, pat.I);
        },
    );
    
    // Source: examples/calc.egg:15
    // Rule: rule_15
    MyTx::add_rule(
        "rule_15",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let inv_node1 = inv::query(&a);
let g__node3 = g_::query(&a, &inv_node2);
rule_15Pat::new(a)
        },
        |ctx, pat| {
            ctx.union(pat.rule_15_node1, pat.I);
        },
    );
    
    // Source: examples/calc.egg:16
    // Rule: rule_16
    MyTx::add_rule(
        "rule_16",
        default_ruleset,
        || {
            let A = Expr::query_leaf();
let g__node1 = g_::query(&A, &A);
let g__node3 = g_::query(&A, &g__node2);
let g__node5 = g_::query(&A, &g__node4);
rule_16Pat::new(A)
        },
        |ctx, pat| {
            ctx.union(pat.rule_16_node1, pat.I);
        },
    );
    
    // Source: examples/calc.egg:17
    let A2 = g_(A, A);
    // Source: examples/calc.egg:18
    let A4 = g_(A2, A2);
    // Source: examples/calc.egg:19
    let A8 = g_(A4, A4);
    // Source: examples/calc.egg:20
    // Assert: g_(A4, A4) == g_(g_(A2, A2), g_(A2, A2))
    // Source: examples/calc.egg:21
    // Assert: g_(g_(A2, A2), g_(A2, A2)) == g_(A2, g_(A2, g_(A2, A2)))
    // Source: examples/calc.egg:24
    let a = aConst();
    // Source: examples/calc.egg:25
    let b = bConst();
    // Source: examples/calc.egg:27
    // Assert: g_(g_(b, g_(inv(a), a)), inv(b)) == g_(b, inv(b))
    // Source: examples/calc.egg:28
    // Assert: g_(b, inv(b)) == I
    // Source: examples/calc.egg:28
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/calc.egg:28
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
