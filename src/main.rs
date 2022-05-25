// use std::collections::BTreeMap;
// use std::error::Error;
// use std::fs::File;
// use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

mod memtable;

#[derive(Debug, Serialize, Deserialize)]
enum Values {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
}

fn main(){
    let mut cache = memtable::MemTable::with_capacity(usize::MAX, Some(String::from("./test_memory")));

    loop {
        let command_stdin = std::io::stdin();
        let mut command_buffer = String::new();
        command_stdin.read_line(&mut command_buffer).expect("Error reading from stdin. Crashing...");
        
        match command_buffer.as_str() {
            "exit" | "EXIT" => {
                break;
            },
            _ => {
                // Try to parse the command. 
                // command can take the following forms: 
                /*
                    set x integer 10
                    set y bool 5
                    set z string How YOU doin?
                    get x
                    get z
                    flush <- this writes to disk.
                */

                let iterator = command_buffer.split_once(' ');
                match iterator {
                    Some(optional_split) => {
                        let command = optional_split.0;

                        match command.to_ascii_lowercase().as_str() {
                            "set" => {
                                let mut iterator = optional_split.1.split(' ');
                                let variable_name = iterator.next().unwrap();
                                let variable_type = iterator.next();
                                // now parse the variable and check to see what kind it is. If int or float, then parse the thing, otherwise, just parse the string and check what kind of memtable::Values to generate from this. 

                                match variable_type {
                                    Some("integer") => {
                                        let variable_value = iterator.next();
                                        let parsed_int = variable_value.expect("Error getting\
                                                                                         integer value from iterator.")                      .parse::<i32>()
                                                                .expect("Error parsing integer value. Is this valid? ");

                                        // Construct a value and push it into the index. 
                                        let value = memtable::Values::INT(parsed_int);
                                        cache.insert(variable_name.to_owned(), &value).expect(format!("Error inserting {}, {:?} into the cache.", variable_name, value).as_str());
                                    },
                                    Some("float") => {
                                        let variable_value = iterator.next();
                                        let parsed_int = variable_value.expect("Error getting\
                                                                                         integer value from iterator.")                      .parse::<f64>()
                                                                .expect("Error parsing integer value. Is this valid? ");

                                        // Construct a value and push it into the index. 
                                        let value = memtable::Values::FLOAT(parsed_int);
                                        cache.insert(variable_name.to_owned(), &value).expect(format!("Error inserting {}, {:?} into the cache.", variable_name, value).as_str());
                                    },
                                    Some("string") => {
                                        // This is tricky. TODO::
                                    },
                                    Some("bool") => {
                                        let variable_name = iterator.next().expect("Could not get variable name");
                                        let criteria = iterator.next().expect("Could not get boolean value from the command");

                                        let value = match criteria.to_ascii_lowercase().as_str() {
                                            "true" => { memtable::Values::BOOL(true) },
                                            "false" => { memtable::Values::BOOL(false) },
                                            _ => {
                                                panic!("not a boolean value passed. Pass either `true` or `false`");
                                            }
                                        };

                                        cache.insert(variable_name.to_owned(), &value).expect("Could not write boolean value to the cache");
                                    },
                                    _ => {
                                        panic!("Unknown type of variable.")
                                    }
                                }

                            },
                            "get" => {},
                            "flush" => {},
                            _ => {
                                panic!("Unknown command");
                            }
                        };
                    },
                    None => {
                        panic!("Invalid command passed!")
                    }
                }
            }
        }
    }
}