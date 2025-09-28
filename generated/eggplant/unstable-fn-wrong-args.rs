// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/unstable-fn-wrong-args.egg:2
// Datatype 'Math' defined with variants:
//   - Inc: variant (defined at unstable-fn-wrong-args.egg:1)
#[eggplant::dsl]
enum Math {
    Inc { 
        arg_Math_00: Math,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/unstable-fn-wrong-args.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/unstable-fn-wrong-args.egg:2
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/unstable-fn-wrong-args.egg:2
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
