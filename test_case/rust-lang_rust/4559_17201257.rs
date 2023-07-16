

        trait Pushable {
            fn push_ch(&mut self, ch: char);
        }

        impl Pushable for ~str {
            fn push_ch(&mut self, ch: char) { str::push_char(self, ch) }
        }

        fn main() {
            let mut string: ~str = ~"";
            string.push_ch('z')
        }

