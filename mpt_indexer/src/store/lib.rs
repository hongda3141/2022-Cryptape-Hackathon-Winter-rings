use std::marker::Sized;

trait Store {
    type Error;

    fn get<K: AsRef<[u8]>, V: Sized>(&self, key: K) -> Result<Option<V>, Self::Error>;
    fn set<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) -> Result<(), Self::Error>;
}
