use std::collections::HashMap;

struct MemoryStore {
    data: HashMap<Vec<u8>, Vec<u8>>,
}

impl Store for MemoryStore {
    type Error = &'static str;

    fn get<K: AsRef<[u8]>, V: Sized>(&self, key: K) -> Result<Option<V>, Self::Error> {
        let key = key.as_ref();
        let value = self.data.get(key).map(|v| v.as_slice());
        Ok(value.map(|v| v as &[u8] as &V))
    }

    fn set<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) -> Result<(), Self::Error> {
        let key = key.as_ref().to_vec();
        let value = value.as_ref().to_vec();
        self.data.insert(key, value);
        Ok(())
    }
}
