// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};
use log::info;

fn main() {
    env_logger::init();
    // Source: examples/semi_naive_set_function.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/semi_naive_set_function.egg:12
    // Assert: f::new(0) == 3
    // Source: examples/semi_naive_set_function.egg:13
    // Assert: f::new(1) == 3
    // Source: examples/semi_naive_set_function.egg:14
    // Assert: f::new(2) == 3
    // Source: examples/semi_naive_set_function.egg:15
    // Assert: f::new(3) == 3
    // Source: examples/semi_naive_set_function.egg:20
    // Assert: f::new(0) == 3
    // Source: examples/semi_naive_set_function.egg:21
    // Assert: f::new(1) == 3
    // Source: examples/semi_naive_set_function.egg:22
    // Assert: f::new(2) == 3
    // Source: examples/semi_naive_set_function.egg:23
    // Assert: f::new(3) == 3
    // Source: examples/semi_naive_set_function.egg:25
    let y = x;
    // Source: examples/semi_naive_set_function.egg:27
    // Assert: f::new(0) == 3
    // Source: examples/semi_naive_set_function.egg:28
    // Assert: f::new(1) == 3
    // Source: examples/semi_naive_set_function.egg:29
    // Assert: f::new(2) == 3
    // Source: examples/semi_naive_set_function.egg:30
    // Assert: f::new(3) == 3
    // Source: examples/semi_naive_set_function.egg:33
    let y = x;
    // Source: examples/semi_naive_set_function.egg:35
    // Assert: f::new(0) == 3
    // Source: examples/semi_naive_set_function.egg:36
    // Assert: f::new(1) == 3
    // Source: examples/semi_naive_set_function.egg:37
    // Assert: f::new(2) == 3
    // Source: examples/semi_naive_set_function.egg:38
    // Assert: f::new(3) == 3
    // Source: examples/semi_naive_set_function.egg:52
    // Assert: 20 == g::new(20)
    // Source: examples/semi_naive_set_function.egg:52
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/semi_naive_set_function.egg:52
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
