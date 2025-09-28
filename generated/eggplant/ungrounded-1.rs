// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

fn main() {
    env_logger::init();
    // Source: examples/ungrounded-1.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/ungrounded-1.egg:2
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/ungrounded-1.egg:2
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
