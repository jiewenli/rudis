
struct Dict {
    dict_type: DictType,
    ht: DictHashTable<K,V>,
}

enum DictType {
    Default,
}

type DictHashTable<K,V> = HashMap<K,V>;

impl<K,V> Dict<K,V> {

    // find
    pub fn find(&self, k: K) -> V {
        self.ht[k]
    }

    pub fn add(&mut self, k: K, v: V) -> u32 {
        self.ht[k] = v;
        1
    }

    pub fn replace(&mut self, k: K, v: V) -> u32 {
        self.ht[k] = v;
        1
    }

    pub fn delete(&mut self, k: K) {
        
    }
}

#[cfg(test)]
mod test {
    pub fn dict_test() {
        let dic = Dict::new();
        dic.add("test1", "test1");
        assert_eq!(dic.find("test1"), "test1");
    }
}
