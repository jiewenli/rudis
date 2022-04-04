pub struct ZipList<T> {
    encoding: ZipListEncoding,
    content: Vec<Option<T>>,
}

enum ZipListEncoding {
    Nil,
    Encode(i32, i32),
}

impl <T> ZipList<T> {
    pub fn new(&self) -> self {
        ZipList { encoding: ZipListEncoding::Nil, content: Vec::new()}
    }
}