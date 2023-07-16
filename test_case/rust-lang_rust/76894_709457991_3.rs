rust
&(*$GPIOX::ptr()).moder.modify(|r, w|
    w.bits((r.bits() & !(0b11 << offset)) | (input << offset))
);
