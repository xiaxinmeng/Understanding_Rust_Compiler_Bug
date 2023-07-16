
pub(crate) fn send_message<T: MailboxMessage>(channel: MailboxChannel, message: &T) -> MailboxResult<&T> {
    let msg_ptr: *const T = message;
    let msg_ptr_uncached: u32 = msg_ptr as u32 | 0xC000_0000;
    print("simple print\r\n"); // works
    print!("{}", "print in macro print!\r\n"); //works
    print(format!("{}\r\n", "simple print with format!().as_str()").as_str()); // works
    print(format!("{}", format!("{}\r\n", "print with format!(format!().as_str()).as_str()").as_str()).as_str()); // fails
