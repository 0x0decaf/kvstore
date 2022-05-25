// use bitvec::prelude::*;
// use log::{debug, error, info};

// use core::num;
// use std::collections::{BTreeMap, HashMap};
// use std::fmt;
// use std::io::Write;

// type CompressionLevel = usize;

// pub struct LSMIndex {
//     memtable: Memtable,
//     secondary_memtable: Memtable,
//     disk_store: HashMap<CompressionLevel, String>,
//     mem_flush_limit: usize,
//     wal_file_store: String,
// }

// impl LSMIndex {
//     pub fn new(capacity: usize, wal_file_path: String) -> Self {
//         return Self {
//             memtable: Memtable::new(),
//             secondary_memtable: Memtable::new(), // For use when flushing primary memtable to disk.
//             disk_store: HashMap::with_capacity(capacity),
//             mem_flush_limit: capacity,
//             wal_file_store: wal_file_path,
//         };
//     }

//     pub fn search(&self, key: String) -> Option<Value> {
//         unimplemented!()
//     }

//     // TODO: Change function signature to support multiple-types of "defined" errors later on.
//     // Maybe even implement the error trait! Who knows what could happen!
//     pub fn put(&mut self, key: String, value: Value) -> Result<(), String> {
//         self.memtable.put(key, value);
//         if self.memtable.len() > self.mem_flush_limit {
//             /*
//             TODO: swapping the memtables during run is risky. For now, writes when cache is full will simply block until the memtable is flushed onto the disk.
//             */

//             self.memtable.flush("");
//         }

//         Ok(())
//     }

//     fn swap_memtable(&mut self, new_memtable: &Memtable) {
//         unimplemented!()
//     }
// }

// pub enum Value {
//     INT(i64),
//     FLOAT(f64),
//     STRING(String),
// }

// #[derive(Debug)]
// pub enum MemtableError {
//     ExceededCapacity,
//     KeyNotFound,
//     InsertError,
// }

// // Implement display trait for debug purpose. This trait is also required for std::error::Error trait implementation.
// impl std::fmt::Display for MemtableError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// // Implements error.
// impl std::error::Error for MemtableError {}

// struct Memtable {
//     size: usize,
//     cache: BTreeMap<String, Value>,
//     // bloomFilter: BloomFilter,
// }

// impl<'a> Memtable {
//     fn new(size: usize) -> Self {
//         return Self {
//             size,
//             cache: BTreeMap::new(),
//             // bloomFilter: BloomFilter::new(size),
//         };
//     }

//     fn with_capacity(capacity: usize) -> Self {
//         return Self { size: capacity, cache: BTreeMap::new() }
//     }

//     fn len(&self) -> usize {
//         self.cache.len()
//     }

//     fn serialize(&self) -> Vec<u8> {
//         // First, prepare the cache metadata to write into the disk.
//         let mut flushable = Vec::<u8>::new(); // vec of u8 is expected to be written to the disk.

//         // return flushable.
//         flushable
//     }

//     fn deserialize(bytes: &'a [u8]) -> &'a Self {
//         unimplemented!()
//     }

//     fn put(&mut self, key: String, value: Value) -> Result<(), MemtableError> {
//         match self.cache.insert(key, value) {
//             Some(_) => Ok(()),
//             None => Err(MemtableError::InsertError),
//         }
//     }

//     fn search(&self, key: String) -> Value {
//         unimplemented!()
//     }

//     fn flush(&self, diskpath: &str) -> Result<(), Box<dyn std::error::Error>> {
//         let mut segment_file_handle = std::fs::File::create(diskpath)?;
//         segment_file_handle.write(&self.serialize()[..])?;
//         Ok(())
//     }
// }
