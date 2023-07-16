rust
    use std::cmp::Ordering::*;
    // Note: can exit at the "ghost" element.
    macro_rules! move_cursor_by {
        ($by:expr) => {
            match ($by).cmp(&0) {
                Equal => (),
                Greater => {
                    for _ in 0..($by) {
                        if cursor.current().is_none() {
                            cursor = list.cursor_front_mut();
                        }
                        cursor.move_next();
                    }
                }
                Less => {
                    for _ in 0..-($by) {
                        if cursor.current().is_none() {
                            cursor = list.cursor_back_mut();
                        }
                        cursor.move_prev();
                    }
                }
            };
        };
    }
