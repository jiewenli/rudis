
pub struct RudisServer {
    rudis_db: RudisDb,
    db_num: u32,
    time: Instant::now(),
}

impl RudisServer {
    pub fn select(&mut self, num: u32) -> self {
        self.db_num = num;
        self.rudis_db = RudisDb { dict: Dict { key: "-", val: Obj {}}};
        self
    }
}

struct RudisDb {
    dict: Dict,
}
