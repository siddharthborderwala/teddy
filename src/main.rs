use std::collections::{HashMap, VecDeque};
use std::io::Write;

fn main() {
    let mut map = HashMap::<String, String>::new();
    let mut lmap = HashMap::<String, VecDeque<String>>::new();
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
                    if map.contains_key(&k.to_string()) || lmap.contains_key(&k.to_string()) {
                        println!("yes");
                    } else {
                        println!("no");
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LSET" => {
                if let Some(k) = command_vec.get(1) {
                    let size = command_vec.len() - 2;
                    let mut vec = VecDeque::<String>::with_capacity(size);
                    for v in command_vec.iter().skip(2) {
                        vec.push_back(v.to_string());
                    }
                    lmap.insert(k.to_string(), vec);
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LGET" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get(&k.to_string()) {
                        let mut response = String::new();
                        for v in vec.iter() {
                            response.push_str(format!("{}, ", v).as_str());
                        }
                        println!("{}", response.trim_end_matches(", "));
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LLEN" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get(&k.to_string()) {
                        println!("{}", vec.len());
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LPOP" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get_mut(&k.to_string()) {
                        vec.pop_front();
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LPUSH" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get_mut(&k.to_string()) {
                        if let Some(v) = command_vec.get(2) {
                            vec.push_front(v.to_string());
                        } else {
                            println!("ERROR: No value supplied for for LPUSH on {}", k);
                        }
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "RPOP" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get_mut(&k.to_string()) {
                        vec.pop_back();
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "RPUSH" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get_mut(&k.to_string()) {
                        if let Some(v) = command_vec.get(2) {
                            vec.push_back(v.to_string());
                        } else {
                            println!("ERROR: No value supplied for for RPUSH on {}", k);
                        }
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
                } else {
                    println!("ERROR: No key supplied");
                }
            }
            "LRANGE" => {
                if let Some(k) = command_vec.get(1) {
                    if let Some(vec) = lmap.get_mut(&k.to_string()) {
                        if let Some([start, stop]) = command_vec.get(2..=3) {
                            if let Ok(from) = start.parse::<usize>() {
                                if let Ok(mut to) = stop.parse::<usize>() {
                                    if to >= vec.len() {
                                        to = to - vec.len();
                                    } else if to == vec.len() {
                                        to = to - 1;
                                    }
                                    let mut response = String::new();
                                    for v in vec.range(from..=to) {
                                        response.push_str(format!("{}, ", v).as_str());
                                    }
                                    println!("{}", response.trim_end_matches(", "));
                                } else {
                                    println!(
                                        "ERROR: Invalid 'stop' index supplied for for LRANGE on {}",
                                        k
                                    );
                                }
                            } else {
                                println!(
                                    "ERROR: Invalid 'start' index supplied for for LRANGE on {}",
                                    k
                                );
                            }
                        } else {
                            println!(
                                "ERROR: No 'start' 'stop' index supplied for for LRANGE on {}",
                                k
                            );
                        }
                    } else {
                        println!("ERROR: No value associated with key {}", k);
                    }
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
