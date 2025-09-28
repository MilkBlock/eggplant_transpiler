// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/shadow-local-constructor.egg:2
// Datatype 'MySort' defined with variants:
//   - Wrap: variant (defined at shadow-local-constructor.egg:1)
#[eggplant::dsl]
enum MySort {
    Wrap { 
        arg_i64_00: i64,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/shadow-local-constructor.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/shadow-local-constructor.egg:2
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/shadow-local-constructor.egg:2
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
