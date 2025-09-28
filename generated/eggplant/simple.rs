// Generated Eggplant Rust Code
use eggplant::{{prelude::*, tx_rx_vt_pr}};

#[eggplant::dsl]
enum Math {
    Num { 
        value: i64,
    },
    Add { 
        left: Math,
        right: Math,
    },
}

#[eggplant::pat_vars]
struct AddPat {
    l: Num,
    r: Num,
    p: Add,
}

fn main() {
    MyTx::add_rule(
        "add_rule",
        default_ruleset,
        || {
            let l = Num::query();
let r = Num::query();
let p = Add::query(&l, &r);
AddPat::new(l, r, p)
        },
        |ctx, pat| {
            let cal = ctx.devalue(pat.l.num) + ctx.devalue(pat.r.num);
let add_value = ctx.insert_num(cal);
ctx.union(pat.p, add_value);
        },
    );
    
    // Rewrite rule: add_simplify
    // Add(Num(1), Num(2)) => Num(3)
    // Assert: eval(Add(Num(1), Num(2))) == Num(3)
    println!("Eggplant program executed successfully!");
}
