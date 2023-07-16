rust
pub fn simulate(&mut self, new_value: DataType) {
    RefCell::undo_leak(&mut self.data);
    *self.data.get_mut() = new_value;
}

pub fn wrapper_frame_data<'frame, 'sim: 'frame>(&'sim self, frame_data: &mut Vec<FrameData<'frame>>) {
    let guard = self.data.borrow();
    let data: &DataType = &*guard;
    
    // No more unsafe!                                                                                                   
    let data: &'sim DataType = Ref::leak(guard);
    frame_data.push(FrameData {
        data
    });
}
