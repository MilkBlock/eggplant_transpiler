// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/unstable-fn-wrong-args-type.egg:2
// Datatype 'Math' defined with variants:
//   - Zero: variant (defined at unstable-fn-wrong-args-type.egg:1)
//   - Inc: variant (defined at unstable-fn-wrong-args-type.egg:1)
#[eggplant::dsl]
enum Math {
    Zero {},
    Inc { 
        arg_Math_10: Math,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/unstable-fn-wrong-args-type.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/unstable-fn-wrong-args-type.egg:3
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/unstable-fn-wrong-args-type.egg:3
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
