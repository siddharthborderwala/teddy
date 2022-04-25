use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut map = HashMap::<String, String>::new();
    let mut cmd = String::new();
    loop {
        print!("$ ");
        std::io::stdout()
            .flush()
            .expect("ERROR: Could not write to stdout");
        std::io::stdin()
            .read_line(&mut cmd)
            .expect("ERROR: Could not read input");
        let command_vec = cmd.trim().split_whitespace().collect::<Vec<&str>>();
        let op = match command_vec.get(0) {
            Some(v) => *v,
            None => {
                println!("ERROR: invalid operation");
                continue;
            }
        };
        match op {
            "SET" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(v) = command_vec.get(2) {
                        map.insert(k.to_string(), v.to_string());
                    } else {
                        println!("ERROR: No value supplied for key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "GET" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(v) = map.get(&k.to_string()) {
                        println!("{}", v);
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "DELETE" => {
                if let Some(k) = command_vec.get(1) {
                    map.remove(&k.to_string());
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "EXISTS" => {
                if let Some(k) = command_vec.get(1) {
                    println!("{}", map.contains_key(&k.to_string()));
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "QUIT" => {
                println!(">>> Goodbye from Teddy");
                break;
            }
            op => {
                println!("ERROR: Unrecognized operation {}", op);
            }
        };
        cmd.clear();
    }
}
