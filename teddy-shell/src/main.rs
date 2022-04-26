use std::io::Write;

use teddy_rs::{Operation, Query, Teddy};

fn main() {
    let mut teddy = Teddy::new();
    loop {
        let mut cmd = String::new();
        print!("$ ");
        std::io::stdout()
            .flush()
            .expect("ERROR: Could not write to stdout");
        std::io::stdin()
            .read_line(&mut cmd)
            .expect("ERROR: Could not read input");
        let query = match Query::new(cmd.as_str()) {
            Ok(q) => q,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        match query.operator {
            Operation::GET => match teddy.get(query.operand) {
                Ok(v) => println!("{:?}", v),
                Err(msg) => eprintln!("ERROR: {}", msg),
            },
            Operation::SET => teddy.set(query.operand, query.values.unwrap()),
            Operation::EXISTS => println!("{}", teddy.exists(query.operand)),
            Operation::DELETE => match teddy.delete(query.operand) {
                Ok(_) => {}
                Err(msg) => eprintln!("ERROR: {}", msg),
            },
        };
    }
}
