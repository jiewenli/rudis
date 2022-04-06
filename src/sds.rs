pub type Sds = Option<String>;

impl Sds {
    pub fn new(c: String) -> Self {
        Some(c)
    }

    pub fn empty() -> Self {
        None
    }

    pub fn free() {

    }

    pub fn clone(&self) {
        self.clone()
    }

    pub fn clear(&self) {
        self = None;
    }

    pub fn cat(&self, target: String) -> String {

    }

    pub fn cpy(&self, content: String) -> String {

    }

    pub fn trim(&self, pair: String) -> String {

    }
    
    pub fn cmp(&self, target: String) -> bool {
        
    }
}

