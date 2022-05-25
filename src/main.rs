use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use serde::{Serialize, Deserialize};

mod memtable;

#[derive(Debug, Serialize, Deserialize)]
enum Values {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
}

fn main() -> Result<(), Box<dyn Error>> {

    // let mut file_handle = File::create("./dumped").expect("Error opening dumped file.");
    // let mut index = BTreeMap::<String, Values>::new();
    // index.insert("Hello".to_string(), Values::INT(32));
    // index.insert("Alpha".to_string(), Values::FLOAT(5.9967));
    // index.insert(
    //     "Balloon".to_string(),
    //     Values::STRING(String::from("Something tells me this is a long motherfucking string. Still test?")),
    // );

    // index.insert("booleantest".to_string(), Values::BOOL(false));

    // let serialized_data = bincode::serialize::<BTreeMap<String, Values>>(&index)?;
    // file_handle.write(&serialized_data).expect("Error in writing serialized data to file.");

    // // Now read from the same file and try to deserialize into proper structures.

    // file_handle.flush()?;
    // let mut file_handle = File::open("./dumped").expect("Error while opening written bincode file.");

    // let converted_from_memory = bincode::deserialize::<BTreeMap<String, Values>>(&serialized_data).expect
    // ("Memory recovery failed.");

    // for entry in converted_from_memory {
    //     println!("Entry deserialized: {} is {:?}", entry.0, entry.1); 
    // }

    // let mut readbuffer: Vec<u8> = vec![];
    // let count = file_handle.read_to_end(&mut readbuffer).expect("Error while reading file the second time.");
    // println!("Read {} bytes from file.", count);
    // let converted = match bincode::deserialize::<BTreeMap<String, Values>>(&readbuffer) {
    //     Ok(converted) => converted,
    //     Err(err) => panic!("Error in deserialization:: {:?}", err)
    // };
    
    // for entry in converted {
    //     println!("Size of entry in memory: {}", std::mem::size_of::<(String, Values)>());
    //     println!("Entry deserialized: {} is {:?}", entry.0, entry.1);
    // }
    Ok(())
}
