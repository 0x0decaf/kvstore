use std::collections::{HashMap, BTreeMap};
type CompressionLevel = usize;

struct LSMIndex {
    memtable: Memtable,
    secondary_memtable: Memtable,
    disk_store: HashMap<CompressionLevel, String>,
    mem_flush_limit: usize,
    wal_file_store: String    
}

impl LSMIndex {
    fn new(capacity: usize, wal_file_path: String) -> Self {
        return Self { 
            memtable: Memtable::new(),
            secondary_memtable: Memtable::new(),  // For use when flushing primary memtable to disk.
            disk_store: HashMap::with_capacity(capacity),
            mem_flush_limit: capacity,
            wal_file_store: wal_file_path
        }       
    }

    fn search(&self, key: String) -> Option<Value> {
        unimplemented!()
    }

    // TODO: Change function signature to support multiple-types of "defined" errors later on.
    // Maybe even implement the error trait! Who knows what could happen! 
    fn put(&mut self, key: String, value: Value) -> Result<(), String> {
        unimplemented!()
    }
}

enum Value {
    INT(i64),
    FLOAT(f64),
    STRING(String),
}


enum MemtableError {
    ExceededCapacity
}

struct Memtable {
    cache: BTreeMap<String, Value>,
    bloomFilter: BloomFilter
}

impl<'a> Memtable {
    fn new() -> Self { 
        return Self { 
            cache: BTreeMap::new(), 
            bloomFilter: BloomFilter::new() 
        }
    }

    fn serialize(&'a self) -> &'a [u8] {
        unimplemented!()
    }

    fn deserialize(bytes: &'a [u8]) -> &'a Self {
        unimplemented!()
    }

    fn push(&mut self, key: String, value: Value) -> Result<(), MemtableError>{
        unimplemented!()
    }

    fn search(&self, key: String) -> Value {
        unimplemented!()
    }
}

struct BloomFilter{}
impl BloomFilter {
    fn new() -> Self {
        unimplemented!()
    }
}