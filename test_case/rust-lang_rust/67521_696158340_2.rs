
pub fn pad_to_align(&self) -> Layout {
        let pad = self.padding_needed_for(self.align());
        let new_size = self.size() + pad;
        Layout::from_size_align(new_size, self.align()).unwrap()
}
