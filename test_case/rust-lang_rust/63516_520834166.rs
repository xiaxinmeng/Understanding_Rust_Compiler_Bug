
#[inline(never)] // never inline, if inlined the compiler seem to mess up the | 0xC000_0000 and do a | 0xC000_0008?????
pub(crate) fn send_message<T: MailboxMessage>(channel: MailboxChannel, message: &T) -> MailboxResult<&T> {
