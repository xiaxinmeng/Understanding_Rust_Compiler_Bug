rs
pub struct Bigger<'a> {
    _marker: &'a (),
}
impl<'a> Bigger<'a> {
    pub fn get_addr(byte_list: &'a mut Vec<u8>) -> &mut u8 {
        let _ = byte_list.iter_mut().map(|item| {
            Self::other(item);
        }).collect::<Vec<_>>();

        byte_list.push(0);
        byte_list.last_mut().unwrap()
    }

    pub fn other<'b: 'a>(_value: &'b mut u8) {
        panic!()
    }
}
