// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{prelude::*, tx_rx_vt_pr};
use log::info;

// Source: examples/tests/web-demo/list.egg:2
// Datatype 'List' defined with variants:
//   - list_append: variant (defined at examples/tests/web-demo/list.egg:6)
//   - list_set: variant (defined at examples/tests/web-demo/list.egg:9)
//   - Nil: variant (defined at list.egg:1)
//   - Cons: variant (defined at list.egg:1)
#[eggplant::dsl]
enum List {
    list_append { arg0: List, arg1: List },
    list_set { arg0: List, arg1: i64, arg2: i64 },
    Nil {},
    Cons { arg_i64_10: i64, arg_List_11: List },
}

// Source: examples/tests/web-demo/list.egg:7
// Pattern variables for rule matching
// Variables: list
#[eggplant::pat_vars]
struct rule_7Pat {
    list: Expr,
}

// Source: examples/tests/web-demo/list.egg:8
// Pattern variables for rule matching
// Variables: head, tail, list
#[eggplant::pat_vars]
struct rule_8Pat {
    head: Expr,
    tail: Expr,
    list: Expr,
}

// Source: examples/tests/web-demo/list.egg:10
// Pattern variables for rule matching
// Variables: head, tail, item
#[eggplant::pat_vars]
struct rule_10Pat {
    head: Expr,
    tail: Expr,
    item: Expr,
}

fn main() {
    env_logger::init();
    // Source: examples/tests/web-demo/list.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);

    // Source: examples/tests/web-demo/list.egg:7
    // Rule: rule_7
    MyTx::add_rule(
        "rule_7",
        default_ruleset,
        || {
            let nil_node1 = Nil::query();
            let list = Expr::query_leaf();
            let list_append_node3 = list_append::query(&nil_node2, &list);
            rule_7Pat::new(list)
        },
        |ctx, pat| {
            ctx.union(pat.rule_7_node1, pat.list);
        },
    );

    // Source: examples/tests/web-demo/list.egg:8
    // Rule: rule_8
    MyTx::add_rule(
        "rule_8",
        default_ruleset,
        || {
            let head = Expr::query_leaf();
            let tail = Expr::query_leaf();
            let cons_node1 = Cons::query(&head, &tail);
            let list = Expr::query_leaf();
            let list_append_node3 = list_append::query(&cons_node2, &list);
            rule_8Pat::new(head, tail, list)
        },
        |ctx, pat| {
            let result = Cons::new(&pat.head, &pat.list_append_node2);
            ctx.union(pat.rule_8_node1, result);
        },
    );

    // Source: examples/tests/web-demo/list.egg:10
    // Rule: rule_10
    MyTx::add_rule(
        "rule_10",
        default_ruleset,
        || {
            let head = Expr::query_leaf();
            let tail = Expr::query_leaf();
            let cons_node1 = Cons::query(&head, &tail);
            let item = Expr::query_leaf();
            let list_set_node3 = list_set::query(&cons_node2, 0, &item);
            rule_10Pat::new(head, tail, item)
        },
        |ctx, pat| {
            let result = Cons::new(&pat.item, &pat.tail);
            ctx.union(pat.rule_10_node1, result);
        },
    );

    // Source: examples/tests/web-demo/list.egg:11
    let a = Cons::new(1, &Cons::new(2, &Nil::new()));
    // Source: examples/tests/web-demo/list.egg:12
    let b = Cons::new(3, &Nil::new());
    // Source: examples/tests/web-demo/list.egg:13
    let c = Cons::new(1, &Cons::new(2, &Cons::new(3, &Nil::new())));
    // Source: examples/tests/web-demo/list.egg:14
    let d = Cons::new(1, &Cons::new(4, &Nil::new()));
    // Source: examples/tests/web-demo/list.egg:15
    let e = list_append::new(&a, &b);
    // Source: examples/tests/web-demo/list.egg:16
    let f = list_set::new(&a, 1, 4);
    // Source: examples/tests/web-demo/list.egg:19
    // Assert: e == c
    // Source: examples/tests/web-demo/list.egg:20
    // Assert: list_length::new(&c) == 3
    // Source: examples/tests/web-demo/list.egg:21
    // Assert: list_get::new(&b, 0) == 3
    // Source: examples/tests/web-demo/list.egg:22
    // Assert: list_get::new(&a, 1) == 2
    // Source: examples/tests/web-demo/list.egg:23
    // Assert: f == d
    // Source: examples/tests/web-demo/list.egg:23
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/tests/web-demo/list.egg:23
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    info!("Eggplant program executed successfully!");
}
