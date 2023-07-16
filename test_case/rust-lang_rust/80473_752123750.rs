rust

#[derive(Debug)]
pub struct StructRegistry {
    structs: HashMap<String, Arc<Struct>>
}
impl Drop for StructRegistry {
    fn drop(&mut self) {
        println!("StructRegistry dropped");
    }
}

impl StructRegistry {
    pub fn new() -> StructRegistry {
        StructRegistry { structs: Default::default() }
    }
    pub fn add_struct(&mut self, struct_: Arc<Struct>) {
        self.structs.insert(struct_.ident.to_string(), struct_);
    }
    pub fn get_struct(&self, name: &String) -> Option<Arc<Struct>> {
        self.structs.get(name).map(Arc::clone)
    }
}

#[derive(Debug)]
pub struct StructRegistry {
    structs: HashMap<String, Arc<Struct>>
}
impl Drop for StructRegistry {
    fn drop(&mut self) {
        println!("StructRegistry dropped");
    }
}

impl StructRegistry {
    pub fn new() -> StructRegistry {
        StructRegistry { structs: Default::default() }
    }
    pub fn add_struct(&mut self, struct_: Arc<Struct>) {
        self.structs.insert(struct_.ident.to_string(), struct_);
    }
    pub fn get_struct(&self, name: &String) -> Option<Arc<Struct>> {
        self.structs.get(name).map(Arc::clone)
    }
}

#[derive(Debug)]
pub struct StructRegistry {
    structs: HashMap<String, Arc<Struct>>
}
impl Drop for StructRegistry {
    fn drop(&mut self) {
        println!("StructRegistry dropped");
    }
}

impl StructRegistry {
    pub fn new() -> StructRegistry {
        StructRegistry { structs: Default::default() }
    }
    pub fn add_struct(&mut self, struct_: Arc<Struct>) {
        self.structs.insert(struct_.ident.to_string(), struct_);
    }
    pub fn get_struct(&self, name: &String) -> Option<Arc<Struct>> {
        self.structs.get(name).map(Arc::clone)
    }
}

// I have to, since it is gonna be wrapped only in RwLock
unsafe impl Sync for StructRegistry {}
unsafe impl Send for StructRegistry {}
