// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};

fn main() {
    // Source: examples/bool.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/bool.egg:2
    // Assert: and(true, true) == true
    // Source: examples/bool.egg:3
    // Assert: and(true, false) == false
    // Source: examples/bool.egg:4
    // Assert: or(true, false) == true
    // Source: examples/bool.egg:7
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/bool.egg:7
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    println!("Eggplant program executed successfully!");
}
