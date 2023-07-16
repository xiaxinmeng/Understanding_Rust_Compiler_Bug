 rust
impl str {
    pub fn to_lowercase(&self) -> String {
        let mut s = String::with_capacity(self.len());
        for (i, c) in self[..].char_indices() {
            if c == 'Σ' && is_final_sigma(self, i) {
                s.push_str("ς")
            } else {
                s.extend(c.to_lowercase());
            }
        }
        return s;

        fn is_final_sigma(s: &str, i: usize) -> bool {
            debug_assert!('Σ'.len_utf8() == 2);
            s[..i].chars().rev().skip_while(is_case_ignorable).any(is_cased_letter) &&
            !s[i + 2..].chars() .skip_while(is_case_ignorable).any(is_cased_letter)
        }
    }
}
