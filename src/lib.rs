use std::sync::Arc;

use crossbeam_skiplist::SkipMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("NotFound")]
    NotFound,
    #[error("IOError")]
    IOError,
    #[error("Unknown")]
    Unknown,
}


#[allow(dead_code)]
pub struct DB {
    map: Arc<SkipMap<Vec<u8>, Vec<u8>>>,
    wal_path: String,
}

impl DB {
    pub fn open(wal_path: String) -> Self {
        Self {
            map: Arc::new(SkipMap::new()),
            wal_path,
        }
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error> {
        let entry = self.map.get(key.as_ref());
        Ok(entry.map(|item| item.value().clone()))
    }

    pub fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) -> Result<(), Error> {
        self.map
            .insert(key.as_ref().to_vec(), value.as_ref().to_vec());
        Ok(())
    }

    pub fn delete<K: AsRef<[u8]>>(&self, key: K) -> Result<(), Error> {
        self.map.remove(key.as_ref());
        Ok(())
    }

    pub fn key_may_exist<K: AsRef<[u8]>>(&self, key: K) -> bool {
        self.map.contains_key(key.as_ref())
    }
}
