use eggplant_transpiler::ast::parse::Parser;
use std::env;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        args[1].clone()
    } else {
        "test_new_handle_style.egg".to_string()
    };

    let program = std::fs::read_to_string(&filename).expect("Failed to read file");

    // Parse the program - this will trigger tokenization internally
    let mut parser = Parser::default();
    let commands = parser.get_program_from_string(None, &program).unwrap();

    println!("\nParsed {} commands:", commands.len());
    for (i, cmd) in commands.iter().enumerate() {
        println!("  Command {}: {:?}", i, cmd);
    }

    // Check if we have a rewrite command and inspect its conditions
    if commands.len() > 1 {
        if let eggplant_transpiler::ast::Command::Rewrite(_, rewrite, _) = &commands[1] {
            println!("\nRewrite conditions: {:?}", rewrite.conditions);
            println!("Rewrite conditions length: {}", rewrite.conditions.len());
        }
    }
}
