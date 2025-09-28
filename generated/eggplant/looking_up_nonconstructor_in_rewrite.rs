// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/looking_up_nonconstructor_in_rewrite.egg:2
// Datatype 'E' defined with variants:
//   - Sum: variant (defined at looking_up_nonconstructor_in_rewrite.egg:1)
#[eggplant::dsl]
enum E {
    Sum { 
        arg_i64_00: i64,
        arg_i64_01: i64,
    },
}

// Source: examples/looking_up_nonconstructor_in_rewrite.egg:3
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct rule_3Pat {
}

fn main() {
    env_logger::init();
    // Source: examples/looking_up_nonconstructor_in_rewrite.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/looking_up_nonconstructor_in_rewrite.egg:3
    // Rule: rule_3
    MyTx::add_rule(
        "rule_3",
        default_ruleset,
        || {
            let sum_node1 = Sum::query(3, 4);
rule_3Pat::new()
        },
        |ctx, pat| {
            let result = Sum::new(5, &pat.__node2);
ctx.union(pat.rule_3_node1, result);
        },
    );
    
    // Source: examples/looking_up_nonconstructor_in_rewrite.egg:4
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/looking_up_nonconstructor_in_rewrite.egg:4
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
