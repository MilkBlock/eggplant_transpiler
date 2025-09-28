// Generated Eggplant Rust Code
use eggplant::{{prelude::*, tx_rx_vt_pr}};

#[eggplant::dsl]
enum Math {
}

fn main() {
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    println!("Eggplant program executed successfully!");
}
