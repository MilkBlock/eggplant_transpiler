// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/unbound.egg:2
// Datatype 'Math' defined with variants:
//   - Add: variant (defined at unbound.egg:1)
//   - Sub: variant (defined at unbound.egg:1)
#[eggplant::dsl]
enum Math {
    Add { 
        arg_Math_00: Math,
        arg_Math_01: Math,
    },
    Sub { 
        arg_Math_10: Math,
        arg_Math_11: Math,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/unbound.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/unbound.egg:2
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/unbound.egg:2
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
