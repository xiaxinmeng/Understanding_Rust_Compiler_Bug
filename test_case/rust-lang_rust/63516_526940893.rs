
fn send_specific_msg() {
    send_specific_msg(0x8, &MyMessage::new());
}

fn send_generic_msg<'a, T: Message>(channel: u8, msg: &'a T) {
   write_gpu().and_then(|| {
     read_gpu().and_ten(|| { })
  });
}
