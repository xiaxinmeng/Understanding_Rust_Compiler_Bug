rust

struct Cmd {
    token: String,
    port: String,
}

struct Ttyd<'a, 'b> {
    mutex: Arc<Mutex<BTreeMap<String,  Option<&'a mut Cmd>>>>,
}

impl<'a, 'b> Ttyd<'a, 'b> {
    fn init(&mut self) {
        let db: Arc<Mutex<BTreeMap<String,Option<&'a mut Cmd>>>> = Arc::new(Mutex::new(BTreeMap::new()));
        self.mutex = db;
        return;
    }

    fn link(&mut self, token: &'a str) -> String {
        let hmp = self.mutex.lock().unwrap();

        let val = hmp.get(token).unwrap().unwrap();
        let res = val.port.to_string();
        val.port = "".to_string() ; // error:  `val` is a `&` reference, so the data it refers to cannot be written
        return res;
    }
}
