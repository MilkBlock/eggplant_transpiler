// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

fn main() {
    env_logger::init();
    // Source: examples/bool.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/bool.egg:2
    // Assert: and::new(&true, &true) == true
    // Source: examples/bool.egg:3
    // Assert: and::new(&true, &false) == false
    // Source: examples/bool.egg:4
    // Assert: or::new(&true, &false) == true
    // Source: examples/bool.egg:5
    // Assert: bool__::new(1, 1) == true
    // Source: examples/bool.egg:6
    // Assert: bool__::new(&_5, &_5) == true
    // Source: examples/bool.egg:7
    // Assert: bool__::new(1, 3) == false
    // Source: examples/bool.egg:8
    // Assert: bool__::new(3, 1) == false
    // Source: examples/bool.egg:9
    // Assert: bool__::new(1, 2) == true
    // Source: examples/bool.egg:10
    // Assert: bool__::new(2, 1) == false
    // Source: examples/bool.egg:11
    // Assert: bool__::new(1, 1) == false
    // Source: examples/bool.egg:12
    // Assert: bool___::new(1, 2) == true
    // Source: examples/bool.egg:13
    // Assert: bool___::new(2, 1) == false
    // Source: examples/bool.egg:14
    // Assert: bool___::new(1, 1) == true
    // Source: examples/bool.egg:15
    // Assert: bool__::new(1, 2) == false
    // Source: examples/bool.egg:16
    // Assert: bool__::new(2, 1) == true
    // Source: examples/bool.egg:17
    // Assert: bool__::new(1, 1) == false
    // Source: examples/bool.egg:18
    // Assert: bool___::new(1, 2) == false
    // Source: examples/bool.egg:19
    // Assert: bool___::new(2, 1) == true
    // Source: examples/bool.egg:20
    // Assert: bool___::new(1, 1) == true
    // Source: examples/bool.egg:23
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/bool.egg:23
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
