// Generated Eggplant Rust Code
use eggplant::{{prelude::*, tx_rx_vt_pr}};

#[eggplant::dsl]
enum G {
    IConst, {}
    AConst, {}
    BConst, {}
    g* { 
        arg0: G,
        arg1: G,
    },
    inv { 
        arg0: G,
    },
}

#[eggplant::pat_vars]
struct defaultPat {
    I: Expr,
    a: Expr,
}

#[eggplant::pat_vars]
struct defaultPat {
    a: Expr,
    I: Expr,
}

#[eggplant::pat_vars]
struct defaultPat {
    a: Expr,
}

#[eggplant::pat_vars]
struct defaultPat {
    a: Expr,
}

#[eggplant::pat_vars]
struct defaultPat {
    A: Expr,
}

fn main() {
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    let I = IConst();
    let A = AConst();
    let B = BConst();
    MyTx::add_rule(
        "default",
        default_ruleset,
        || {
            let I = Expr::query_leaf();
let a = Expr::query_leaf();
let g*_node = g*::query(&I, &a);
        },
        |ctx, pat| {
            ctx.union(pat.default_node, pat.a);
        },
    );
    
    MyTx::add_rule(
        "default",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let I = Expr::query_leaf();
let g*_node = g*::query(&a, &I);
        },
        |ctx, pat| {
            ctx.union(pat.default_node, pat.a);
        },
    );
    
    MyTx::add_rule(
        "default",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let inv_node = inv::query(&a);
let g*_node = g*::query(&inv_node, &a);
        },
        |ctx, pat| {
            ctx.union(pat.default_node, pat.I);
        },
    );
    
    MyTx::add_rule(
        "default",
        default_ruleset,
        || {
            let a = Expr::query_leaf();
let inv_node = inv::query(&a);
let g*_node = g*::query(&a, &inv_node);
        },
        |ctx, pat| {
            ctx.union(pat.default_node, pat.I);
        },
    );
    
    MyTx::add_rule(
        "default",
        default_ruleset,
        || {
            let A = Expr::query_leaf();
let g*_node = g*::query(&A, &A);
let g*_node = g*::query(&A, &g*_node);
let g*_node = g*::query(&A, &g*_node);
        },
        |ctx, pat| {
            ctx.union(pat.default_node, pat.I);
        },
    );
    
    let A2 = g*(A, A);
    let A4 = g*(A2, A2);
    let A8 = g*(A4, A4);
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    println!("Eggplant program executed successfully!");
}
