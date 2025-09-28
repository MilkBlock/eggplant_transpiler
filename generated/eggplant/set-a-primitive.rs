// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

fn main() {
    env_logger::init();
    // Source: examples/set-a-primitive.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/set-a-primitive.egg:1
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/set-a-primitive.egg:1
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
