rust
use std::sync::Mutex;

struct FunctionIcon {
    get_icon: Mutex<Box<dyn FnMut() -> u32>>,
}

impl FunctionIcon {
    fn get_icon(&self) -> impl '_ + std::ops::DerefMut<Target=Box<dyn FnMut() -> u32>> {
        self.get_icon.lock().unwrap()
    }
    
    fn load_icon(&self)  {
        let mut get_icon = self.get_icon();
        let rgba_icon = (*get_icon)();
    }
}
