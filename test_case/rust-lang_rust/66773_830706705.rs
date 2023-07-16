rust
pub fn panic_with(info: &PanicInfo, message: &str) -> ! {
    if CheckPanicMessage::starts_with(info, message) {
        // Ok
    } else {
        // Failed.
    }
}

use core::cmp;
use core::fmt;
use core::str::Bytes;

pub struct CheckPanicMessage<'a> {
    bytes: Bytes<'a>,
}

impl CheckPanicMessage<'_> {
    pub fn starts_with(info: &PanicInfo, message: &str) -> bool {
        let mut check = CheckPanicMessage { bytes: message.bytes() };
        use core::fmt::Write;
        write!(&mut check, "{}", info).is_ok() && check.bytes.len() == 0
    }
}

impl fmt::Write for CheckPanicMessage<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let len = cmp::min(self.bytes.len(), s.len());
        self.bytes.by_ref().take(len).eq(s.bytes().take(len))
            .then(|| ()).ok_or(fmt::Error)
    }
}
