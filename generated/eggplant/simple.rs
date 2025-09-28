// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};

// Source: examples/simple.egg:1
// Datatype 'Math' defined with variants:
//   - Num: variant (defined at examples/simple.egg:1)
//   - Add: variant (defined at examples/simple.egg:2)
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

// Source: examples/simple.egg:2
// Pattern variables for rule matching
// Variables: l, r, p
#[eggplant::pat_vars]
struct AddPat {
    l: Num,
    r: Num,
    p: Add,
}

fn main() {
    // Source: examples/simple.egg:3
    // Rule: add_rule
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
    
    // Source: examples/simple.egg:4
    // Rewrite rule: add_simplify
    // Add(Num(1), Num(2)) => Num(3)
    // Source: examples/simple.egg:5
    // Assert: eval(Add(Num(1), Num(2))) == Num(3)
    println!("Eggplant program executed successfully!");
}
