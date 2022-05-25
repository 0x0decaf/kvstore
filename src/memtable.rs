use std::fmt::Display;
use std::io::Read;
use std::{fs::File, io::Write};
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Values {
    INT(i32),
    FLOAT(f64),
    STRING(String),
    BOOL(bool),
}

pub struct MemTable {
    count: usize,
    capacity: usize,
    persistence: bool,
    persistence_path: Option<String>,
    index: BTreeMap<String, Values>
}

impl MemTable {
    pub fn with_capacity(capacity: usize, persistence_path: Option<String>) -> Self {
        // if persistence path is set to Some(path), then it means we want to use persistence. This can work out in one of two ways - the path already exists (in which case we just read from the path and load into index), or it does not. If the path is set to None, we will just create an in-memory index and return it.
        if persistence_path.is_some() {
            if let Some(persistence_path) = persistence_path {
                // Path is set. Now check if it already exists. 
                return match File::open(&persistence_path) {
                    Ok (file_handle) => {
                        let mut file_handle = File::open(&persistence_path).expect("Could not open file.");

                        // Load from persistence path into index, and return self. 
                        let mut buffer: Vec<u8> = vec![];
                        file_handle.read_to_end(&mut buffer).expect("Could not read existing persistence buffer on disk.");
                        let index = bincode::deserialize::<BTreeMap<String, Values>>(&buffer).expect("The file exists at the persistence path, but seems to be corrupt - bincode cannot read it. Aborting...");
                        
                        return Self {
                            count: index.len(),
                            capacity: capacity,
                            index: index,
                            persistence: true,
                            persistence_path: Some(persistence_path)
                        };
                    },
                    Err (_) => {
                        // No file was found in the persistence path location. Create an in-memory index and return it as in below 'else' arm.
                        return Self {
                            count: 0,
                            persistence: true,
                            persistence_path: Some(persistence_path),
                            capacity,
                            index: BTreeMap::new()
                        };
                    }
                }
            } else {
                panic!("Could not decipher persistence path on creating the index. Try again?");
            }
        } else {
            return Self {
                count: 0,
                persistence: false,
                persistence_path: None,
                capacity,
                index: BTreeMap::new()
            };
        }
    }

    pub fn insert(&mut self, key: String, value: &Values) -> Result<(), String> {
        if self.count == self.capacity && !self.persistence{
            return Err(String::from("Could not write to memtable. Already full."));
        } else if self.persistence && self.count == self.capacity {
            match self.dump() {
                Ok(_) => {

                },
                Err(_) => {
                    return Err(String::from("Could not dump to disk. Quitting."));
                }
            };
        } 
        else {
            // Write to internal cache. 
            self.index.insert(key, value.clone());
            self.count += 1;
        }

        Ok(())
    }

    pub fn get(&self, key: String) -> Option<&Values> {
        return self.index.get(&key);
    }
    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        // Check if persistence and path both enabled.
        if self.persistence {
            if let Some(persistence_path) = &self.persistence_path {
                let mut dump_handle = File::create(&persistence_path)?;
                let serialized = bincode::serialize(&self.index)?;
                dump_handle.write(&serialized)?;
            } else {
                panic!("Could not parse persistence path. Quitting the appplication");
            }
        }

        Ok(())
    }
}

impl Display for MemTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted = format!("Metadata:\ncount: {}\ncapacity: {}\npersistence: {}\npersistence_path: {:?}\n contents:: \n---\n", &self.count, &self.capacity, &self.persistence, &self.persistence_path);


        for entry in &self.index {
            formatted = format!("{}\n{}, {:?}", formatted, entry.0, entry.1);
        }

        f.write_str(&formatted)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::MemTable;

    #[test]
    fn test_in_memory(){
        use super::Values;

        let mut cache = MemTable::with_capacity(3, None);

        cache.insert(String::from("test1"), &Values::INT(10)).expect("Failed test on first insertion");
        cache.insert(String::from("test2"), &Values::INT(10)).expect("Failed test on second insertion");
        cache.insert(String::from("test3"), &Values::INT(10)).expect("Failed test on third insertion");

        match cache.insert(String::from("last one"), &Values::STRING(String::from("last value"))) {
            Ok(_) => { 
                panic!("This should not happen --- \n {}", cache);
            },
            Err(_) => { /*  Test passed  */ }
        }
    }

    #[test]
    fn test_disk() {
        use super::Values;

        let mut cache = MemTable::with_capacity(3, Some(String::from("./test_dump")));

        cache.insert(String::from("test1"), &Values::INT(10)).expect("Failed test on first insertion");
        cache.insert(String::from("test2"), &Values::INT(10)).expect("Failed test on second insertion");
        cache.insert(String::from("test3"), &Values::INT(10)).expect("Failed test on third insertion");

        match cache.insert(String::from("last one"), &Values::STRING(String::from("last value"))) {
            Ok(_) => { 
                /* Passed test :) */
            },
            Err(_) => { 
                panic!("Failed test at disk check dump:: \n{}", cache);
            }
        } 
    }

}