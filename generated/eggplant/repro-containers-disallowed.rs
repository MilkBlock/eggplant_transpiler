// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/repro-containers-disallowed.egg:2
// Datatype 'IVec' defined with variants:
#[eggplant::dsl]
enum IVec {
}

fn main() {
    env_logger::init();
    // Source: examples/repro-containers-disallowed.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/repro-containers-disallowed.egg:3
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/repro-containers-disallowed.egg:3
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
