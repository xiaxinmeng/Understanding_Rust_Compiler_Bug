plain
   Compiling std v0.0.0 (/checkout/library/std)
error: unknown start of token: `
   --> library/core/src/task/wake.rs:195:8
    |
195 |     /# `clone`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
195 |     /# 'clone`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:195:14
    |
    |
195 |     /# `clone`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
195 |     /# `clone'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:205:8
    |
    |
205 |     /# `wake`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
205 |     /# 'wake`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:205:13
    |
    |
205 |     /# `wake`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
205 |     /# `wake'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:214:8
    |
    |
214 |     /# `wake_by_ref`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
214 |     /# 'wake_by_ref`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:214:20
    |
    |
214 |     /# `wake_by_ref`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
214 |     /# `wake_by_ref'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:222:8
    |
    |
222 |     /# `drop`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
222 |     /# 'drop`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:222:13
    |
    |
222 |     /# `drop`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
222 |     /# `drop'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:270:8
    |
    |
270 |     /# `clone`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
270 |     /# 'clone`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:270:14
    |
    |
270 |     /# `clone`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
270 |     /# `clone'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:280:8
    |
    |
280 |     /# `wake`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
280 |     /# 'wake`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:280:13
    |
    |
280 |     /# `wake`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
280 |     /# `wake'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:289:8
    |
    |
289 |     /# `wake_by_ref`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
289 |     /# 'wake_by_ref`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:289:20
    |
    |
289 |     /# `wake_by_ref`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
289 |     /# `wake_by_ref'

error: unknown start of token: `
   --> library/core/src/task/wake.rs:297:8
    |
    |
297 |     /# `drop`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
297 |     /# 'drop`

error: unknown start of token: `
   --> library/core/src/task/wake.rs:297:13
    |
    |
297 |     /# `drop`
    |
    |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
    |
297 |     /# `drop'

error: expected item after doc comment
   --> library/core/src/task/wake.rs:194:5
    |
    |
185 |   impl RawWakerVTable {
    |                       - while parsing this item list starting here
186 | /     /// Creates a new `RawWakerVTable` from the provided `clone`, `wake`,
187 | |     /// `wake_by_ref`, and `drop` functions.
188 | |     ///
189 | |     /// These functions must all be thread-safe (even though [`RawWaker`] is
...   |
192 | |     /// arbitrary threads or invoked by `&` reference. For example, this means that if the
193 | |     /// `clone` and `drop` functions manage a reference count, they must do so atomically.
    | |__________________________________________________________________________________________- other attributes here
    |       ^^^ this doc comment doesn't document anything
...
321 |   }
    |   - the item list ends here
    |   - the item list ends here

   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: union has missing stability attribute
   --> library/core/src/task/wake.rs:164:1
    |
164 | / pub union RawWakerVTable {
165 | |     v1: RawWakerVTableV1,
166 | |     v2: RawWakerVTableV2,
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/task/wake.rs:163:10
---

error: implementation has missing stability attribute
   --> library/core/src/task/wake.rs:169:1
    |
169 | / impl PartialEq for RawWakerVTable {
170 | |     fn eq(&self, other: &Self) -> bool {
171 | |         unsafe { self.v2 == other.v2 }
173 | | }
    | |_^

error: implementation has missing stability attribute
error: implementation has missing stability attribute
   --> library/core/src/task/wake.rs:174:1
    |
174 | / impl fmt::Debug for RawWakerVTable {
175 | |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
177 | |             match self {
...   |
182 | |     }
183 | | }
