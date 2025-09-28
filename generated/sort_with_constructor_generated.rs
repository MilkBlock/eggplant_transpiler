// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};

// Source: sort_with_constructor.egg:2
// Datatype 'Bool' defined with variants:
//   - True: variant (defined at sort_with_constructor.egg:3)
//   - False: variant (defined at sort_with_constructor.egg:4)
#[eggplant::dsl]
enum Bool {
    True {},
    False {},
}

// Source: sort_with_constructor.egg:5
// Datatype 'Int' defined with variants:
//   - Zero: variant (defined at sort_with_constructor.egg:6)
//   - Succ: variant (defined at sort_with_constructor.egg:7)
#[eggplant::dsl]
enum Int {
    Zero {},
    Succ { 
        arg0: Int,
    },
}

fn main() {
    // Source: sort_with_constructor.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: sort_with_constructor.egg:7
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: sort_with_constructor.egg:7
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
