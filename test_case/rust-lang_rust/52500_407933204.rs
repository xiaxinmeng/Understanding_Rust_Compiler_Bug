
   Compiling ggez v0.5.0 (file:///home/icefox/tmp/ggez)
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: internal compiler error: cat_expr Errd
   --> src/context.rs:141:75
    |
141 |       pub fn process_event(&mut self, event: &winit::Event) -> winit::Event {
    |  ___________________________________________________________________________^
142 | |         let event = self.gfx_context.hack_event_hidpi(event);
143 | |         match event.clone() {
144 | |             winit_event::Event::WindowEvent { event, .. } => match event {
...   |
188 | |         event
189 | |     }
    | |_____^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:143:9
    |
143 | /         match event.clone() {
144 | |             winit_event::Event::WindowEvent { event, .. } => match event {
145 | |                 winit_event::WindowEvent::Resized(_) => {
146 | |                     self.gfx_context.resize_viewport();
...   |
186 | |             _ => (),
187 | |         };
    | |_________^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:144:62
    |
144 |               winit_event::Event::WindowEvent { event, .. } => match event {
    |  ______________________________________________________________^
145 | |                 winit_event::WindowEvent::Resized(_) => {
146 | |                     self.gfx_context.resize_viewport();
147 | |                 }
...   |
162 | |                 _ => (),
163 | |             },
    | |_____________^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:144:68
    |
144 |             winit_event::Event::WindowEvent { event, .. } => match event {
    |                                                                    ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:155:79
    |
155 |                   winit_event::WindowEvent::MouseInput { button, state, .. } => {
    |  _______________________________________________________________________________^
156 | |                     let pressed = match state {
157 | |                         winit_event::ElementState::Pressed => true,
158 | |                         winit_event::ElementState::Released => false,
159 | |                     };
160 | |                     self.mouse_context.set_button(button, pressed);
161 | |                 }
    | |_________________^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:156:41
    |
156 |                     let pressed = match state {
    |                                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:160:51
    |
160 |                     self.mouse_context.set_button(button, pressed);
    |                                                   ^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:164:62
    |
164 |               winit_event::Event::DeviceEvent { event, .. } => match event {
    |  ______________________________________________________________^
165 | |                 winit_event::DeviceEvent::MouseMotion { delta: (x, y) } => {
166 | |                     self.mouse_context
167 | |                         .set_last_delta(Point2::new(x as f32, y as f32));
...   |
183 | |                 _ => (),
184 | |             },
    | |_____________^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:164:68
    |
164 |             winit_event::Event::DeviceEvent { event, .. } => match event {
    |                                                                    ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:167:53
    |
167 |                         .set_last_delta(Point2::new(x as f32, y as f32));
    |                                                     ^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:167:63
    |
167 |                         .set_last_delta(Point2::new(x as f32, y as f32));
    |                                                               ^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:174:23
    |
174 |                   }) => {
    |  _______________________^
175 | |                     let pressed = match state {
176 | |                         winit_event::ElementState::Pressed => true,
177 | |                         winit_event::ElementState::Released => false,
...   |
181 | |                     self.keyboard_context.set_key(keycode, pressed);
182 | |                 }
    | |_________________^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:175:41
    |
175 |                     let pressed = match state {
    |                                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:180:40
    |
180 |                         .set_modifiers(keyboard::KeyMods::from(modifiers));
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/context.rs:180:64
    |
180 |                         .set_modifiers(keyboard::KeyMods::from(modifiers));
    |                                                                ^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:150:33
    |
150 |           events_loop.poll_events(|event| {
    |  _________________________________^
151 | |             let event = ctx.process_event(&event);
152 | |             match event {
153 | |                 Event::WindowEvent { event, .. } => match event {
...   |
231 | |             }
232 | |         });
    | |_________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:150:41
    |
150 |           events_loop.poll_events(|event| {
    |  _________________________________________^
151 | |             let event = ctx.process_event(&event);
152 | |             match event {
153 | |                 Event::WindowEvent { event, .. } => match event {
...   |
231 | |             }
232 | |         });
    | |_________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:151:25
    |
151 |             let event = ctx.process_event(&event);
    |                         ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:152:13
    |
152 | /             match event {
153 | |                 Event::WindowEvent { event, .. } => match event {
154 | |                     WindowEvent::Resized(dpi::LogicalSize { width, height }) => {
155 | |                         state.resize_event(ctx, width as f32, height as f32);
...   |
230 | |                 Event::Suspended(_) => (),
231 | |             }
    | |_____________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:153:53
    |
153 |                   Event::WindowEvent { event, .. } => match event {
    |  _____________________________________________________^
154 | |                     WindowEvent::Resized(dpi::LogicalSize { width, height }) => {
155 | |                         state.resize_event(ctx, width as f32, height as f32);
156 | |                     }
...   |
224 | |                     }
225 | |                 },
    | |_________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:153:59
    |
153 |                 Event::WindowEvent { event, .. } => match event {
    |                                                           ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:155:25
    |
155 |                         state.resize_event(ctx, width as f32, height as f32);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:155:44
    |
155 |                         state.resize_event(ctx, width as f32, height as f32);
    |                                            ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:158:29
    |
158 |                         if !state.quit_event(ctx) {
    |                             ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:158:46
    |
158 |                         if !state.quit_event(ctx) {
    |                                              ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:159:29
    |
159 |                             ctx.quit();
    |                             ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:162:53
    |
162 |                       WindowEvent::Focused(gained) => {
    |  _____________________________________________________^
163 | |                         state.focus_event(ctx, gained);
164 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:163:25
    |
163 |                         state.focus_event(ctx, gained);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:163:43
    |
163 |                         state.focus_event(ctx, gained);
    |                                           ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:163:48
    |
163 |                         state.focus_event(ctx, gained);
    |                                                ^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:165:59
    |
165 |                       WindowEvent::ReceivedCharacter(ch) => {
    |  ___________________________________________________________^
166 | |                         state.text_input_event(ctx, ch);
167 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:166:25
    |
166 |                         state.text_input_event(ctx, ch);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:166:48
    |
166 |                         state.text_input_event(ctx, ch);
    |                                                ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:166:53
    |
166 |                         state.text_input_event(ctx, ch);
    |                                                     ^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:177:26
    |
177 |                       } => {
    |  __________________________^
178 | |                         let repeat = keyboard::is_key_repeated(ctx);
179 | |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
180 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:178:64
    |
178 |                         let repeat = keyboard::is_key_repeated(ctx);
    |                                                                ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:179:25
    |
179 |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:179:46
    |
179 |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
    |                                              ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:179:60
    |
179 |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
    |                                                            ^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:179:60
    |
179 |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
    |                                                            ^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:190:26
    |
190 |                       } => {
    |  __________________________^
191 | |                         state.key_up_event(ctx, keycode, modifiers.into());
192 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:191:25
    |
191 |                         state.key_up_event(ctx, keycode, modifiers.into());
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:191:44
    |
191 |                         state.key_up_event(ctx, keycode, modifiers.into());
    |                                            ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:191:58
    |
191 |                         state.key_up_event(ctx, keycode, modifiers.into());
    |                                                          ^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:191:58
    |
191 |                         state.key_up_event(ctx, keycode, modifiers.into());
    |                                                          ^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:193:62
    |
193 |                       WindowEvent::MouseWheel { delta, .. } => {
    |  ______________________________________________________________^
194 | |                         let (x, y) = match delta {
195 | |                             MouseScrollDelta::LineDelta(x, y) => (x, y),
196 | |                             MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => {
...   |
200 | |                         state.mouse_wheel_event(ctx, x, y);
201 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:194:38
    |
194 |                           let (x, y) = match delta {
    |  ______________________________________^
195 | |                             MouseScrollDelta::LineDelta(x, y) => (x, y),
196 | |                             MouseScrollDelta::PixelDelta(dpi::LogicalPosition { x, y }) => {
197 | |                                 (x as f32, y as f32)
198 | |                             }
199 | |                         };
    | |_________________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:194:44
    |
194 |                         let (x, y) = match delta {
    |                                            ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:195:66
    |
195 |                             MouseScrollDelta::LineDelta(x, y) => (x, y),
    |                                                                  ^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:195:67
    |
195 |                             MouseScrollDelta::LineDelta(x, y) => (x, y),
    |                                                                   ^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:195:70
    |
195 |                             MouseScrollDelta::LineDelta(x, y) => (x, y),
    |                                                                      ^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:200:25
    |
200 |                         state.mouse_wheel_event(ctx, x, y);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:200:49
    |
200 |                         state.mouse_wheel_event(ctx, x, y);
    |                                                 ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:206:26
    |
206 |                       } => {
    |  __________________________^
207 | |                         let position = mouse::get_position(ctx);
208 | |                         match element_state {
209 | |                             ElementState::Pressed => {
...   |
215 | |                         }
216 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:207:60
    |
207 |                         let position = mouse::get_position(ctx);
    |                                                            ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:208:25
    |
208 | /                         match element_state {
209 | |                             ElementState::Pressed => {
210 | |                                 state.mouse_button_down_event(ctx, button, position.x, position.y)
211 | |                             }
...   |
214 | |                             }
215 | |                         }
    | |_________________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:208:31
    |
208 |                         match element_state {
    |                               ^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:209:54
    |
209 |                               ElementState::Pressed => {
    |  ______________________________________________________^
210 | |                                 state.mouse_button_down_event(ctx, button, position.x, position.y)
211 | |                             }
    | |_____________________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:210:33
    |
210 |                                 state.mouse_button_down_event(ctx, button, position.x, position.y)
    |                                 ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:210:63
    |
210 |                                 state.mouse_button_down_event(ctx, button, position.x, position.y)
    |                                                               ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:210:68
    |
210 |                                 state.mouse_button_down_event(ctx, button, position.x, position.y)
    |                                                                    ^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:212:55
    |
212 |                               ElementState::Released => {
    |  _______________________________________________________^
213 | |                                 state.mouse_button_up_event(ctx, button, position.x, position.y)
214 | |                             }
    | |_____________________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:213:33
    |
213 |                                 state.mouse_button_up_event(ctx, button, position.x, position.y)
    |                                 ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:213:61
    |
213 |                                 state.mouse_button_up_event(ctx, button, position.x, position.y)
    |                                                             ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:213:66
    |
213 |                                 state.mouse_button_up_event(ctx, button, position.x, position.y)
    |                                                                  ^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:218:60
    |
218 |                         let position = mouse::get_position(ctx);
    |                                                            ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:219:54
    |
219 |                         let delta = mouse::get_delta(ctx);
    |                                                      ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:220:25
    |
220 |                         state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
    |                         ^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:220:50
    |
220 |                         state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
    |                                                  ^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:222:26
    |
222 |                       x => {
    |  __________________________^
223 | |                         trace!("ignoring window event {:?}", x);
224 | |                     }
    | |_____________________^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:25
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:25
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:25
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:32
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:62
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                                                              ^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:25
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
   --> src/event.rs:223:62
    |
223 |                         trace!("ignoring window event {:?}", x);
    |                                                              ^

error: internal compiler error: cat_expr Errd
   --> src/event.rs:226:59
    |
226 |                 Event::DeviceEvent { event, .. } => match event {
    |                                                           ^^^^^

error: internal compiler error: no kind for cast
   --> src/context.rs:167:53
    |
167 |                         .set_last_delta(Point2::new(x as f32, y as f32));
    |                                                     ^^^^^^^^

error: internal compiler error: no kind for cast
   --> src/context.rs:167:63
    |
167 |                         .set_last_delta(Point2::new(x as f32, y as f32));
    |                                                               ^^^^^^^^

error: internal compiler error: no type-dependent def for method call
   --> src/event.rs:179:60
    |
179 |                         state.key_down_event(ctx, keycode, modifiers.into(), repeat);
    |                                                            ^^^^^^^^^^^^^^^^

error: internal compiler error: no type-dependent def for method call
   --> src/event.rs:191:58
    |
191 |                         state.key_up_event(ctx, keycode, modifiers.into());
    |                                                          ^^^^^^^^^^^^^^^^

thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', librustc_errors/lib.rs:315:17
stack backtrace:
   0:     0x7fe924d5168e - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::he8af69b64cd628aa
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7fe924d28fc6 - std::sys_common::backtrace::print::hd007d8131d52db53
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7fe924d5a6ad - std::panicking::default_hook::{{closure}}::hd58f2759bdd5fc75
                               at libstd/panicking.rs:211
   3:     0x7fe924d5a420 - std::panicking::default_hook::h2c1bf51c9795af05
                               at libstd/panicking.rs:227
   4:     0x7fe9214f471d - rustc::util::common::panic_hook::h4747d9f4ae34e974
   5:     0x7fe924d5ad73 - std::panicking::rust_panic_with_hook::h28562f4ec57f2c02
                               at libstd/panicking.rs:479
   6:     0x7fe92019ac86 - std::panicking::begin_panic::h690932888d1ff855
   7:     0x7fe9201b4104 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::h48c7a30a16a46ddf
   8:     0x7fe925164e30 - core::ptr::drop_in_place::h2619d7a547886887
   9:     0x7fe9251736cb - rustc_driver::run_compiler_with_pool::h63554a8a1074e873
  10:     0x7fe925096f0c - <scoped_tls::ScopedKey<T>>::set::h2d0f977c46614714
  11:     0x7fe925096c01 - <scoped_tls::ScopedKey<T>>::set::h058ba6dfb6a57aa9
  12:     0x7fe9250f15ca - syntax::with_globals::h64800d12ee1ee8ca
  13:     0x7fe925095692 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hf130c8184a048d5d
  14:     0x7fe924d67be9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:106
  15:     0x7fe925170151 - rustc_driver::run::h49f3a1911b6c48bf
  16:     0x7fe92517d8da - rustc_driver::main::h000c0f11b2e94126
  17:     0x55cec2e98b52 - std::rt::lang_start::{{closure}}::h1492c8444d8ececb
  18:     0x7fe924d5a7b2 - std::panicking::try::do_call::hb494fa268bfac907
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:310
  19:     0x7fe924d67be9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:106
  20:     0x7fe924d3cf05 - std::rt::lang_start_internal::h910a443015422f4a
                               at libstd/panicking.rs:289
                               at libstd/panic.rs:392
                               at libstd/rt.rs:58
  21:     0x55cec2e98bb3 - main
  22:     0x7fe924938a86 - __libc_start_main
  23:     0x55cec2e98a38 - <unknown>
thread panicked while panicking. aborting.
error: Could not compile `ggez`.

To learn more, run the command again with --verbose.
