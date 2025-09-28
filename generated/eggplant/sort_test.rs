// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/sort_test.egg:2
// Datatype 'Bool' defined with variants:
#[eggplant::dsl]
enum Bool {
}

// Source: examples/sort_test.egg:3
// Datatype 'Int' defined with variants:
#[eggplant::dsl]
enum Int {
}

// Source: examples/sort_test.egg:4
// Datatype 'String' defined with variants:
#[eggplant::dsl]
enum String {
}

fn main() {
    env_logger::init();
    // Source: examples/sort_test.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/sort_test.egg:4
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/sort_test.egg:4
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
