plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: implementation has missing stability attribute
   --> library/core/src/ptr/alignment.rs:166:1
    |
166 | / impl const cmp::PartialEq for Alignment {
167 | |     #[inline]
168 | |     fn eq(&self, other: &Self) -> bool {
169 | |         self.as_nonzero().get() == other.as_nonzero().get()
171 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/core/src/ptr/alignment.rs:174:1
    |
174 | / impl const cmp::Ord for Alignment {
175 | |     #[inline]
176 | |     fn cmp(&self, other: &Self) -> cmp::Ordering {
177 | |         self.as_nonzero().get().cmp(&other.as_nonzero().get())
179 | | }
    | |_^

error: could not compile `core` due to 2 previous errors
