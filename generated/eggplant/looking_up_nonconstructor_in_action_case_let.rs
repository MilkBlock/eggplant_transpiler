// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/looking_up_nonconstructor_in_action_case_let.egg:2
// Datatype 'E' defined with variants:
#[eggplant::dsl]
enum E {
}

fn main() {
    env_logger::init();
    // Source: examples/looking_up_nonconstructor_in_action_case_let.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/looking_up_nonconstructor_in_action_case_let.egg:3
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/looking_up_nonconstructor_in_action_case_let.egg:3
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
