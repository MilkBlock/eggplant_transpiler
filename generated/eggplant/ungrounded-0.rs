// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/ungrounded-0.egg:2
// Datatype 'S' defined with variants:
//   - S0: variant (defined at examples/ungrounded-0.egg:3)
//   - S1: variant (defined at examples/ungrounded-0.egg:4)
#[eggplant::dsl]
enum S {
    S0 {},
    S1 { 
        arg0: S,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/ungrounded-0.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/ungrounded-0.egg:6
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/ungrounded-0.egg:6
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
