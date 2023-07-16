
pub fn kmp(text: &str, pattern: &str) -> i64 {
    let pattern_length = pattern.len();

    let mut shifts = vec![1; pattern_length+1];
    let mut shift = 1;

    for pos in 0..pattern_length {
        while shift <= pos &&
              pattern.char_at[pos] != pattern.char_at[pos-shift] {
            shift += shifts[pos-shift];
        }
        shifts[pos+1] = shift;
    }

    let mut start_pos = 0;
    let mut match_len: i64 = 0;
    let mut count = 0;
    for ch in text.chars() {
        while match_len == pattern_length as i64 ||
               match_len >= 0 &&
               pattern.char_at(match_len as usize) != ch {
            start_pos += shifts[match_len as usize];
            match_len -= shifts[match_len as usize] as i64;
        }
        match_len += 1;
        if match_len == pattern_length as i64 { count += 1; }
    }
    return count;
}
