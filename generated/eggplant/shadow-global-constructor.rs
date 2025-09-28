// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/shadow-global-constructor.egg:2
// Datatype 'MySort' defined with variants:
//   - Wrap: variant (defined at shadow-global-constructor.egg:1)
#[eggplant::dsl]
enum MySort {
    Wrap { 
        arg_i64_00: i64,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/shadow-global-constructor.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/shadow-global-constructor.egg:3
    let Wrap = 3;
    // Source: examples/shadow-global-constructor.egg:3
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/shadow-global-constructor.egg:3
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
