rust
impl Option<String> {
    fn map_or(self: *Option<String>, default: i32) {
        let personalityslot: struct(*i8, i32);
        let t: String;
        let ret: i32;

        let _12 = true;
        let _11 = true;
        let _10 = true;

        let %3 = if self.ptr == null { 0i64 } else { 1i64 }
        match %3 {
            0 => goto 'bb2,
            1 => goto 'bb4,
            _ => goto 'bb3,
        }

    'bb1:
        panic!(personalityslot);

    'bb2:
        _11 = false;
        ret = default;
        goto 'bb5;

    'bb3:
        unreachable!();

    'bb4:
        _10 = false;
        t = *(self as *Some<String> as *String);
        _12 = false;

        try {
            let %20 = (main::{{closure}})(&t);
            goto 'bb7;
        } catch_unwind {
            goto 'cleanup;
        }

    'bb5:
        if _12 {
            goto 'bb15;
        } else {
            goto 'bb8;
        }

    'bb6:
        let %26 = if self.ptr == null { 0i64 } else { 1i64 };
        match %26 {
            1 => goto 'bb10,
            _ => goto 'bb12,
        }

    'bb7:
        ret = %20;
        goto 'bb5;

    'bb8:
        if _11 {
            goto 'bb16;
        } else {
            goto 'bb9;
        }

    'bb9:
        let %32 = if self.ptr == null { 0i64 } else { 1i64 };
        match %32 {
            1 => goto 'bb18,
            _ => goto 'bb20,
        }

    'bb10:
        if _10 {
            goto 'bb11;
        } else {
            goto 'bb1;
        }

    'bb11:
        _10 = false;
        ptr::drop_in_place::<String>(self as *Some<String> as *String);
        goto 'bb1;

    'bb12:
        ptr::drop_in_place::<Option<String>>(self);
        goto 'bb1;

    'bb13:
        _11 = false;
        goto 'bb6;

    'bb14:
        if _11 {
            goto 'bb13;
        } else {
            goto 'bb6;
        }

    'bb15:
        _12 = false;
        goto 'bb8;

    'bb16:
        _11 = false;
        goto 'bb9;

    'bb17:
        return ret;

    'bb18:
        if _10 {
            goto 'bb19;
        } else {
            goto 'bb17;
        }

    'bb19:
        _10 = false;
        ptr::drop_in_place::<String>(self as *Some<String> as *String)
        goto 'bb17;

    'bb20:
        ptr::drop_in_place::<Option<String>>(self);
        goto 'bb17;

    'cleanup:
        personalityslot = landingpad();
        goto 'bb14;
    }
}
