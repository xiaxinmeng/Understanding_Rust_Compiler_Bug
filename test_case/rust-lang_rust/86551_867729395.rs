
    Checking helix-term v0.2.0 (/home/ivan/src/pickfire/rs/helix/helix-term)
error[E0432]: unresolved import `helix_view::input`
  --> helix-term/src/commands.rs:18:5
   |
18 |     input::{KeyCode, KeyEvent},
   |     ^^^^^ could not find `input` in `helix_view`

error[E0432]: unresolved import `helix_view::input`
 --> helix-term/src/keymap.rs:7:17
  |
7 | use helix_view::input::{KeyCode, KeyEvent, KeyModifiers};
  |                 ^^^^^ could not find `input` in `helix_view`

error[E0432]: unresolved import `helix_view::input`
  --> helix-term/src/ui/editor.rs:15:17
   |
15 | use helix_view::input::{KeyCode, KeyEvent, KeyModifiers};
   |                 ^^^^^ could not find `input` in `helix_view`

error[E0282]: type annotations needed
   --> helix-term/src/commands.rs:641:26
    |
641 |     cx.on_next_key(move |cx, event| {
    |                          ^^ consider giving this closure parameter a type
    |
    = note: type must be known at this point

error[E0282]: type annotations needed
   --> helix-term/src/keymap.rs:139:35
    |
139 | #[derive(Debug, Clone, PartialEq, Deserialize)]
    |                                   ^^^^^^^^^^^ consider giving this closure parameter a type
    |
    = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `remove` found for struct `Keymaps` in the current scope
   --> helix-term/src/keymap.rs:345:27
    |
141 | pub struct Keymaps(pub HashMap<Mode, HashMap<KeyEvent, Command>>);
    | ------------------------------------------------------------------ method `remove` not found for this
...
345 |         keys.extend(delta.remove(mode).unwrap_or_default());
    |                           ^^^^^^ method not found in `Keymaps`

error[E0608]: cannot index into a value of type `Keymaps`
   --> helix-term/src/ui/editor.rs:596:32
    |
596 |         if let Some(command) = self.keymaps[&Mode::Insert].get(&event) {
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0608]: cannot index into a value of type `Keymaps`
   --> helix-term/src/ui/editor.rs:634:40
    |
634 |                 if let Some(command) = self.keymaps[&mode].get(&event) {
    |                                        ^^^^^^^^^^^^^^^^^^^

error[E0608]: cannot index into a value of type `Keymaps`
   --> helix-term/src/ui/editor.rs:750:46
    |
750 |                         self.last_insert.0 = self.keymaps[&mode][&key];
    |                                              ^^^^^^^^^^^^^^^^^^^

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0282, E0432, E0599, E0608.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `helix-term`

To learn more, run the command again with --verbose.
