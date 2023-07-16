rs
impl HeapFile {
    fn get_page(&mut self, page_num: PageNum) -> &mut [u8] {
        let page_loc = (self.db_id, page_num);
        let page = BUFFER_POOL.lock().unwrap().get_page(page_loc);
        page
    }
    // ...
}
