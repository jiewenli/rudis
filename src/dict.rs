use std::{collections::HashMap, borrow::Borrow};
use core::hash::Hash;


struct Dict<K,V> {
    dict_type: DictType,
    ht: DictHashTable<K,V>,
}

enum DictType {
    Default,
}

type DictHashTable<K,V> = HashMap<K,V>;

impl<K,V> Default for Dict<K,V> {
    fn default() -> Self {
        Self { dict_type: DictType::Default, ht: Default::default() }
    }
}

impl<K,V> Dict<K,V> 
where
    K: Hash + Eq
{

    pub fn new() -> Dict<K,V> {
        Default::default()
    }
    // find
    pub fn find<Q: ?Sized>(&self, k: &Q) -> Option<&V>  
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.ht.get(k)
    }

    pub fn add(&mut self, k: K, v: V) -> u32 
    where
        K: Eq,
    {
        let v = self.ht.insert(k, v);
        match v {
            Some(_) => 1,
            None => 0,
        }
    }

    pub fn replace(&mut self, k: K, v: V) -> u32 {
        match self.ht.remove(&k) {
            Some(_) => {
                self.add(k, v);
                1
            }
            None => 0,
        }
    }

    pub fn delete(&mut self, k: K) {
        self.ht.remove(&k);
    }
}

#[cfg(test)]
mod test {
    use crate::dict::Dict;

    #[test]
    pub fn dict_test() {
        let mut dic = Dict::new();
        dic.add("test1", "test1");
        assert_eq!(dic.find("test1"), Some(&"test1"));
        dic.delete("test1");
    }
}
