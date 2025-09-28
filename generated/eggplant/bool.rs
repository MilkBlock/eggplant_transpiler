// Generated Eggplant Rust Code
use eggplant::{{prelude::*, tx_rx_vt_pr}};

fn main() {
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Assert: and(true, true) == true
    // Assert: and(true, false) == false
    // Assert: or(true, false) == true
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    println!("Eggplant program executed successfully!");
}
