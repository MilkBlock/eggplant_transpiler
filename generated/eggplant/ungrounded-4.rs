// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/ungrounded-4.egg:2
// Datatype 'StringVec' defined with variants:
#[eggplant::dsl]
enum StringVec {
}

// Source: examples/ungrounded-4.egg:3
// Datatype 'E' defined with variants:
//   - F: variant (defined at ungrounded-4.egg:1)
#[eggplant::dsl]
enum E {
    F { 
        arg_StringVec_00: StringVec,
        arg_StringVec_01: StringVec,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/ungrounded-4.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/ungrounded-4.egg:3
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/ungrounded-4.egg:3
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
