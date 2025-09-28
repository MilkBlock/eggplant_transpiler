// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

// Source: examples/looking_up_nonconstructor_in_birewrite.egg:2
// Datatype 'E' defined with variants:
//   - Sum: variant (defined at looking_up_nonconstructor_in_birewrite.egg:1)
#[eggplant::dsl]
enum E {
    Sum { 
        arg_i64_00: i64,
        arg_i64_01: i64,
    },
}

fn main() {
    env_logger::init();
    // Source: examples/looking_up_nonconstructor_in_birewrite.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/looking_up_nonconstructor_in_birewrite.egg:4
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/looking_up_nonconstructor_in_birewrite.egg:4
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
