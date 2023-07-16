plain
2019-09-05T13:32:06.3236544Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T13:32:07.1241418Z ##[command]git config gc.auto 0
2019-09-05T13:32:07.1246868Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T13:32:07.1248435Z ##[command]git config --get-all http.proxy
2019-09-05T13:32:07.1251346Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64010/merge:refs/remotes/pull/64010/merge
---
2019-09-05T14:30:26.6919180Z .................................................................................................... 1500/8994
2019-09-05T14:30:31.8846911Z .................................................................................................... 1600/8994
2019-09-05T14:30:43.6916067Z ..................................................i...............i................................. 1700/8994
2019-09-05T14:30:51.4144082Z .................................................................................................... 1800/8994
2019-09-05T14:31:04.5956717Z .........................................iiiii...................................................... 1900/8994
2019-09-05T14:31:14.6278129Z .................................................................................................... 2100/8994
2019-09-05T14:31:16.9109096Z .................................................................................................... 2200/8994
2019-09-05T14:31:20.4255561Z .................................................................................................... 2300/8994
2019-09-05T14:31:27.4032560Z .................................................................................................... 2400/8994
---
2019-09-05T14:34:12.5618266Z .............................i...............i...................................................... 4700/8994
2019-09-05T14:34:23.4516612Z .................................................................................................... 4800/8994
2019-09-05T14:34:29.2855326Z .................................................................................................... 4900/8994
2019-09-05T14:34:39.7013013Z .................................................................................................... 5000/8994
2019-09-05T14:34:44.9522950Z ..........ii.ii..................................................................................... 5100/8994
2019-09-05T14:34:57.0051023Z .................................................................................................... 5300/8994
2019-09-05T14:35:04.7816264Z .........................................................................i.......................... 5400/8994
2019-09-05T14:35:11.7117441Z .................................................................................................... 5500/8994
2019-09-05T14:35:17.8922543Z .................................................................................................... 5600/8994
2019-09-05T14:35:17.8922543Z .................................................................................................... 5600/8994
2019-09-05T14:35:27.6597937Z ...................................................................ii...i..ii...........i........... 5700/8994
2019-09-05T14:35:51.6114002Z .................................................................................................... 5900/8994
2019-09-05T14:36:00.7849383Z .................................................................................................... 6000/8994
2019-09-05T14:36:00.7849383Z .................................................................................................... 6000/8994
2019-09-05T14:36:07.8473879Z .....................................................................i..ii.......................... 6100/8994
2019-09-05T14:36:34.6186440Z .................................................................................................... 6300/8994
2019-09-05T14:36:36.4903170Z ........................i........................................................................... 6400/8994
2019-09-05T14:36:38.4869745Z ................................................................................................i... 6500/8994
2019-09-05T14:36:40.9373972Z .................................................................................................... 6600/8994
---
2019-09-05T14:40:27.9238191Z 
2019-09-05T14:40:27.9239565Z ---- [ui] ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs stdout ----
2019-09-05T14:40:27.9239944Z diff of stderr:
2019-09-05T14:40:27.9240185Z 
2019-09-05T14:40:27.9240429Z 1 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9241454Z +   --> $DIR/proc-macro-cannot-be-used.rs:10:21
2019-09-05T14:40:27.9241757Z 3    |
2019-09-05T14:40:27.9241757Z 3    |
2019-09-05T14:40:27.9241971Z 4 LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
2019-09-05T14:40:27.9242395Z 
2019-09-05T14:40:27.9242395Z 
2019-09-05T14:40:27.9242612Z 8    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9243039Z 9 
2019-09-05T14:40:27.9243820Z 10 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9245408Z +   --> $DIR/proc-macro-cannot-be-used.rs:10:38
2019-09-05T14:40:27.9245842Z 12    |
2019-09-05T14:40:27.9245842Z 12    |
2019-09-05T14:40:27.9247400Z 13 LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
2019-09-05T14:40:27.9247849Z 
2019-09-05T14:40:27.9247849Z 
2019-09-05T14:40:27.9248060Z 17    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9248284Z 18 
2019-09-05T14:40:27.9248506Z 19 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9252142Z +   --> $DIR/proc-macro-cannot-be-used.rs:14:38
2019-09-05T14:40:27.9252800Z 21    |
2019-09-05T14:40:27.9252800Z 21    |
2019-09-05T14:40:27.9252976Z 22 LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
2019-09-05T14:40:27.9253283Z 
2019-09-05T14:40:27.9253283Z 
2019-09-05T14:40:27.9253442Z 26    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9253883Z 27 
2019-09-05T14:40:27.9254036Z 28 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9255012Z +   --> $DIR/proc-macro-cannot-be-used.rs:17:28
2019-09-05T14:40:27.9255186Z 30    |
2019-09-05T14:40:27.9255186Z 30    |
2019-09-05T14:40:27.9255347Z 31 LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
2019-09-05T14:40:27.9255616Z 
2019-09-05T14:40:27.9255616Z 
2019-09-05T14:40:27.9255934Z 35    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9256057Z 36 
2019-09-05T14:40:27.9256182Z 37 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9256876Z +   --> $DIR/proc-macro-cannot-be-used.rs:17:38
2019-09-05T14:40:27.9257028Z 39    |
2019-09-05T14:40:27.9257028Z 39    |
2019-09-05T14:40:27.9258747Z 40 LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
2019-09-05T14:40:27.9259412Z 
2019-09-05T14:40:27.9259412Z 
2019-09-05T14:40:27.9259570Z 44    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9259691Z 45 
2019-09-05T14:40:27.9259818Z 46 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9260629Z +   --> $DIR/proc-macro-cannot-be-used.rs:21:9
2019-09-05T14:40:27.9260800Z 48    |
2019-09-05T14:40:27.9260800Z 48    |
2019-09-05T14:40:27.9260922Z 49 LL | fn free(#[id] arg1: u8) {
2019-09-05T14:40:27.9261156Z 
2019-09-05T14:40:27.9261156Z 
2019-09-05T14:40:27.9261282Z 53    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9261549Z 54 
2019-09-05T14:40:27.9261748Z 55 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9262457Z +   --> $DIR/proc-macro-cannot-be-used.rs:23:16
2019-09-05T14:40:27.9262628Z 57    |
2019-09-05T14:40:27.9262628Z 57    |
2019-09-05T14:40:27.9263102Z 58 LL |     let lam = |#[id] W(x), #[id] y| ();
2019-09-05T14:40:27.9263442Z 
2019-09-05T14:40:27.9263442Z 
2019-09-05T14:40:27.9263586Z 62    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9263724Z 63 
2019-09-05T14:40:27.9263892Z 64 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9264658Z +   --> $DIR/proc-macro-cannot-be-used.rs:23:28
2019-09-05T14:40:27.9264852Z 66    |
2019-09-05T14:40:27.9264852Z 66    |
2019-09-05T14:40:27.9265006Z 67 LL |     let lam = |#[id] W(x), #[id] y| ();
2019-09-05T14:40:27.9265308Z 
2019-09-05T14:40:27.9265308Z 
2019-09-05T14:40:27.9265474Z 71    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9265636Z 72 
2019-09-05T14:40:27.9265787Z 73 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9266847Z +   --> $DIR/proc-macro-cannot-be-used.rs:29:18
2019-09-05T14:40:27.9267165Z 75    |
2019-09-05T14:40:27.9267165Z 75    |
2019-09-05T14:40:27.9267286Z 76 LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9267530Z 
2019-09-05T14:40:27.9267530Z 
2019-09-05T14:40:27.9267649Z 80    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9267782Z 81 
2019-09-05T14:40:27.9267920Z 82 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9269037Z +   --> $DIR/proc-macro-cannot-be-used.rs:29:30
2019-09-05T14:40:27.9269222Z 84    |
2019-09-05T14:40:27.9269222Z 84    |
2019-09-05T14:40:27.9269344Z 85 LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9269591Z 
2019-09-05T14:40:27.9269591Z 
2019-09-05T14:40:27.9269821Z 89    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9269971Z 90 
2019-09-05T14:40:27.9270095Z 91 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9270770Z +   --> $DIR/proc-macro-cannot-be-used.rs:32:18
2019-09-05T14:40:27.9270915Z 93    |
2019-09-05T14:40:27.9270915Z 93    |
2019-09-05T14:40:27.9271062Z 94 LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9271304Z 
2019-09-05T14:40:27.9271304Z 
2019-09-05T14:40:27.9271441Z 98    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9271562Z 99 
2019-09-05T14:40:27.9271686Z 100 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9272337Z +   --> $DIR/proc-macro-cannot-be-used.rs:32:31
2019-09-05T14:40:27.9272484Z 102    |
2019-09-05T14:40:27.9272484Z 102    |
2019-09-05T14:40:27.9272626Z 103 LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9276734Z 
2019-09-05T14:40:27.9276734Z 
2019-09-05T14:40:27.9276992Z 107    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9277129Z 108 
2019-09-05T14:40:27.9277646Z 109 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9279240Z +   --> $DIR/proc-macro-cannot-be-used.rs:35:22
2019-09-05T14:40:27.9279473Z 111    |
2019-09-05T14:40:27.9279473Z 111    |
2019-09-05T14:40:27.9281263Z 112 LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9281553Z 
2019-09-05T14:40:27.9281553Z 
2019-09-05T14:40:27.9281604Z 116    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9281649Z 117 
2019-09-05T14:40:27.9281720Z 118 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9282218Z +   --> $DIR/proc-macro-cannot-be-used.rs:35:42
2019-09-05T14:40:27.9282264Z 120    |
2019-09-05T14:40:27.9282264Z 120    |
2019-09-05T14:40:27.9282529Z 121 LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9283031Z 
2019-09-05T14:40:27.9283031Z 
2019-09-05T14:40:27.9283106Z 125    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9283154Z 126 
2019-09-05T14:40:27.9283209Z 127 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9283990Z +   --> $DIR/proc-macro-cannot-be-used.rs:38:22
2019-09-05T14:40:27.9284042Z 129    |
2019-09-05T14:40:27.9284042Z 129    |
2019-09-05T14:40:27.9284303Z 130 LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
2019-09-05T14:40:27.9284385Z 
2019-09-05T14:40:27.9284385Z 
2019-09-05T14:40:27.9284434Z 134    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9284500Z 135 
2019-09-05T14:40:27.9284564Z 136 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9285335Z +   --> $DIR/proc-macro-cannot-be-used.rs:38:45
2019-09-05T14:40:27.9285381Z 138    |
2019-09-05T14:40:27.9285381Z 138    |
2019-09-05T14:40:27.9285620Z 139 LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
2019-09-05T14:40:27.9285722Z 
2019-09-05T14:40:27.9285722Z 
2019-09-05T14:40:27.9285771Z 143    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9285817Z 144 
2019-09-05T14:40:27.9285890Z 145 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9286504Z +   --> $DIR/proc-macro-cannot-be-used.rs:44:15
2019-09-05T14:40:27.9286565Z 147    |
2019-09-05T14:40:27.9286565Z 147    |
2019-09-05T14:40:27.9286618Z 148 LL |     fn trait1(#[id] self, #[id] arg1: u8);
2019-09-05T14:40:27.9286770Z 
2019-09-05T14:40:27.9286770Z 
2019-09-05T14:40:27.9286817Z 152    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9286861Z 153 
2019-09-05T14:40:27.9286913Z 154 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9287703Z +   --> $DIR/proc-macro-cannot-be-used.rs:44:27
2019-09-05T14:40:27.9287746Z 156    |
2019-09-05T14:40:27.9287746Z 156    |
2019-09-05T14:40:27.9287803Z 157 LL |     fn trait1(#[id] self, #[id] arg1: u8);
2019-09-05T14:40:27.9287873Z 
2019-09-05T14:40:27.9287873Z 
2019-09-05T14:40:27.9287935Z 161    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9287978Z 162 
2019-09-05T14:40:27.9288136Z 163 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9288632Z +   --> $DIR/proc-macro-cannot-be-used.rs:47:15
2019-09-05T14:40:27.9288674Z 165    |
2019-09-05T14:40:27.9288674Z 165    |
2019-09-05T14:40:27.9288718Z 166 LL |     fn trait2(#[id] &self, #[id] arg1: u8);
2019-09-05T14:40:27.9288806Z 
2019-09-05T14:40:27.9288806Z 
2019-09-05T14:40:27.9288851Z 170    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9288911Z 171 
2019-09-05T14:40:27.9288961Z 172 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9289387Z +   --> $DIR/proc-macro-cannot-be-used.rs:47:28
2019-09-05T14:40:27.9289429Z 174    |
2019-09-05T14:40:27.9289429Z 174    |
2019-09-05T14:40:27.9289638Z 175 LL |     fn trait2(#[id] &self, #[id] arg1: u8);
2019-09-05T14:40:27.9289952Z 
2019-09-05T14:40:27.9289952Z 
2019-09-05T14:40:27.9289994Z 179    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9290033Z 180 
2019-09-05T14:40:27.9290096Z 181 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9290483Z +   --> $DIR/proc-macro-cannot-be-used.rs:50:19
2019-09-05T14:40:27.9290540Z 183    |
2019-09-05T14:40:27.9290540Z 183    |
2019-09-05T14:40:27.9290744Z 184 LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
2019-09-05T14:40:27.9290811Z 
2019-09-05T14:40:27.9290811Z 
2019-09-05T14:40:27.9290870Z 188    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9290909Z 189 
2019-09-05T14:40:27.9290961Z 190 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9291496Z +   --> $DIR/proc-macro-cannot-be-used.rs:50:39
2019-09-05T14:40:27.9291537Z 192    |
2019-09-05T14:40:27.9291537Z 192    |
2019-09-05T14:40:27.9291756Z 193 LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
2019-09-05T14:40:27.9291828Z 
2019-09-05T14:40:27.9291828Z 
2019-09-05T14:40:27.9291870Z 197    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9291927Z 198 
2019-09-05T14:40:27.9291972Z 199 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9292379Z +   --> $DIR/proc-macro-cannot-be-used.rs:53:19
2019-09-05T14:40:27.9292418Z 201    |
2019-09-05T14:40:27.9292418Z 201    |
2019-09-05T14:40:27.9292636Z 202 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9292736Z 
2019-09-05T14:40:27.9292736Z 
2019-09-05T14:40:27.9293272Z 206    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9293326Z 207 
2019-09-05T14:40:27.9293397Z 208 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9293892Z +   --> $DIR/proc-macro-cannot-be-used.rs:53:42
2019-09-05T14:40:27.9293953Z 210    |
2019-09-05T14:40:27.9293953Z 210    |
2019-09-05T14:40:27.9294202Z 211 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9294304Z 
2019-09-05T14:40:27.9294304Z 
2019-09-05T14:40:27.9294351Z 215    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9294398Z 216 
2019-09-05T14:40:27.9294588Z 217 error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9295094Z +   --> $DIR/proc-macro-cannot-be-used.rs:53:58
2019-09-05T14:40:27.9295142Z 219    |
2019-09-05T14:40:27.9295142Z 219    |
2019-09-05T14:40:27.9295410Z 220 LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9295497Z 
2019-09-05T14:40:27.9295540Z 
2019-09-05T14:40:27.9295585Z The actual stderr differed from the expected stderr.
2019-09-05T14:40:27.9295912Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
2019-09-05T14:40:27.9295912Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/proc-macro-cannot-be-used.stderr
2019-09-05T14:40:27.9296183Z To update references, rerun the tests and pass the `--bless` flag
2019-09-05T14:40:27.9296657Z To only update this specific test, also pass `--test-args rfc-2565-param-attrs/proc-macro-cannot-be-used.rs`
2019-09-05T14:40:27.9296737Z error: 1 errors occurred comparing output.
2019-09-05T14:40:27.9296792Z status: exit code: 1
2019-09-05T14:40:27.9296792Z status: exit code: 1
2019-09-05T14:40:27.9297509Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/proc-macro-cannot-be-used/auxiliary" "-A" "unused"
2019-09-05T14:40:27.9297970Z ------------------------------------------
2019-09-05T14:40:27.9297999Z 
2019-09-05T14:40:27.9298206Z ------------------------------------------
2019-09-05T14:40:27.9298245Z stderr:
2019-09-05T14:40:27.9298245Z stderr:
2019-09-05T14:40:27.9298548Z ------------------------------------------
2019-09-05T14:40:27.9298616Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9298893Z    |
2019-09-05T14:40:27.9298893Z    |
2019-09-05T14:40:27.9298948Z LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
2019-09-05T14:40:27.9299023Z    |
2019-09-05T14:40:27.9299023Z    |
2019-09-05T14:40:27.9299399Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9299454Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9299485Z 
2019-09-05T14:40:27.9299549Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9299925Z    |
2019-09-05T14:40:27.9299925Z    |
2019-09-05T14:40:27.9299984Z LL | extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
2019-09-05T14:40:27.9300115Z    |
2019-09-05T14:40:27.9300115Z    |
2019-09-05T14:40:27.9300408Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9300462Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9300492Z 
2019-09-05T14:40:27.9300556Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9300852Z    |
2019-09-05T14:40:27.9300852Z    |
2019-09-05T14:40:27.9300913Z LL | unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
2019-09-05T14:40:27.9300996Z    |
2019-09-05T14:40:27.9300996Z    |
2019-09-05T14:40:27.9301367Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9301438Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9301468Z 
2019-09-05T14:40:27.9301516Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9301870Z    |
2019-09-05T14:40:27.9301870Z    |
2019-09-05T14:40:27.9301912Z LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
2019-09-05T14:40:27.9302009Z    |
2019-09-05T14:40:27.9302009Z    |
2019-09-05T14:40:27.9302283Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9302336Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9302366Z 
2019-09-05T14:40:27.9302422Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9303100Z    |
2019-09-05T14:40:27.9303100Z    |
2019-09-05T14:40:27.9303147Z LL | type Alias = extern "C" fn(#[id] u8, #[id] ...);
2019-09-05T14:40:27.9303257Z    |
2019-09-05T14:40:27.9303257Z    |
2019-09-05T14:40:27.9303584Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9303661Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9303694Z 
2019-09-05T14:40:27.9303746Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9304081Z    |
2019-09-05T14:40:27.9304081Z    |
2019-09-05T14:40:27.9304123Z LL | fn free(#[id] arg1: u8) {
2019-09-05T14:40:27.9304237Z    |
2019-09-05T14:40:27.9304237Z    |
2019-09-05T14:40:27.9304504Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9304708Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9304742Z 
2019-09-05T14:40:27.9304793Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9305163Z    |
2019-09-05T14:40:27.9305163Z    |
2019-09-05T14:40:27.9305207Z LL |     let lam = |#[id] W(x), #[id] y| ();
2019-09-05T14:40:27.9305311Z    |
2019-09-05T14:40:27.9305311Z    |
2019-09-05T14:40:27.9305581Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9305638Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9305686Z 
2019-09-05T14:40:27.9305747Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9306087Z    |
2019-09-05T14:40:27.9306087Z    |
2019-09-05T14:40:27.9306131Z LL |     let lam = |#[id] W(x), #[id] y| ();
2019-09-05T14:40:27.9306405Z    |
2019-09-05T14:40:27.9306405Z    |
2019-09-05T14:40:27.9306801Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9306849Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9306891Z 
2019-09-05T14:40:27.9306936Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9307227Z    |
2019-09-05T14:40:27.9307227Z    |
2019-09-05T14:40:27.9307266Z LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9307428Z    |
2019-09-05T14:40:27.9307428Z    |
2019-09-05T14:40:27.9307735Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9307793Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9307821Z 
2019-09-05T14:40:27.9307882Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9308162Z    |
2019-09-05T14:40:27.9308162Z    |
2019-09-05T14:40:27.9308220Z LL |     fn inherent1(#[id] self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9308295Z    |
2019-09-05T14:40:27.9308295Z    |
2019-09-05T14:40:27.9308550Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9308841Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9308872Z 
2019-09-05T14:40:27.9308943Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9309280Z    |
2019-09-05T14:40:27.9309280Z    |
2019-09-05T14:40:27.9309335Z LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9309409Z    |
2019-09-05T14:40:27.9309409Z    |
2019-09-05T14:40:27.9309671Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9309719Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9309747Z 
2019-09-05T14:40:27.9309790Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9310083Z    |
2019-09-05T14:40:27.9310083Z    |
2019-09-05T14:40:27.9310121Z LL |     fn inherent2(#[id] &self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9310222Z    |
2019-09-05T14:40:27.9310222Z    |
2019-09-05T14:40:27.9310627Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9310695Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9310723Z 
2019-09-05T14:40:27.9310767Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9311061Z    |
2019-09-05T14:40:27.9311061Z    |
2019-09-05T14:40:27.9311265Z LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9311359Z    |
2019-09-05T14:40:27.9311359Z    |
2019-09-05T14:40:27.9311592Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9311655Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9311684Z 
2019-09-05T14:40:27.9311735Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9312038Z    |
2019-09-05T14:40:27.9312038Z    |
2019-09-05T14:40:27.9312242Z LL |     fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
2019-09-05T14:40:27.9312339Z    |
2019-09-05T14:40:27.9312339Z    |
2019-09-05T14:40:27.9312573Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9312637Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9312665Z 
2019-09-05T14:40:27.9313102Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9313512Z    |
2019-09-05T14:40:27.9313512Z    |
2019-09-05T14:40:27.9313878Z LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
2019-09-05T14:40:27.9314008Z    |
2019-09-05T14:40:27.9314008Z    |
2019-09-05T14:40:27.9314329Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9314387Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9314437Z 
2019-09-05T14:40:27.9314490Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9314823Z    |
2019-09-05T14:40:27.9314823Z    |
2019-09-05T14:40:27.9315057Z LL |     fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
2019-09-05T14:40:27.9315171Z    |
2019-09-05T14:40:27.9315171Z    |
2019-09-05T14:40:27.9315434Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9315500Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9315557Z 
2019-09-05T14:40:27.9315610Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9315942Z    |
2019-09-05T14:40:27.9315942Z    |
2019-09-05T14:40:27.9315988Z LL |     fn trait1(#[id] self, #[id] arg1: u8);
2019-09-05T14:40:27.9316074Z    |
2019-09-05T14:40:27.9316074Z    |
2019-09-05T14:40:27.9316356Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9316411Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9316444Z 
2019-09-05T14:40:27.9316685Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9317112Z    |
2019-09-05T14:40:27.9317112Z    |
2019-09-05T14:40:27.9317259Z LL |     fn trait1(#[id] self, #[id] arg1: u8);
2019-09-05T14:40:27.9317331Z    |
2019-09-05T14:40:27.9317331Z    |
2019-09-05T14:40:27.9317606Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9317654Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9317680Z 
2019-09-05T14:40:27.9317742Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9317997Z    |
2019-09-05T14:40:27.9317997Z    |
2019-09-05T14:40:27.9318049Z LL |     fn trait2(#[id] &self, #[id] arg1: u8);
2019-09-05T14:40:27.9318118Z    |
2019-09-05T14:40:27.9318118Z    |
2019-09-05T14:40:27.9318350Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9318408Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9318443Z 
2019-09-05T14:40:27.9318502Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9318780Z    |
2019-09-05T14:40:27.9318780Z    |
2019-09-05T14:40:27.9318818Z LL |     fn trait2(#[id] &self, #[id] arg1: u8);
2019-09-05T14:40:27.9318912Z    |
2019-09-05T14:40:27.9318912Z    |
2019-09-05T14:40:27.9319340Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9319387Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9319415Z 
2019-09-05T14:40:27.9319459Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9319831Z    |
2019-09-05T14:40:27.9319831Z    |
2019-09-05T14:40:27.9320067Z LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
2019-09-05T14:40:27.9320161Z    |
2019-09-05T14:40:27.9320161Z    |
2019-09-05T14:40:27.9320394Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9320460Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9320488Z 
2019-09-05T14:40:27.9320531Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9320812Z    |
2019-09-05T14:40:27.9320812Z    |
2019-09-05T14:40:27.9321004Z LL |     fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
2019-09-05T14:40:27.9321096Z    |
2019-09-05T14:40:27.9321096Z    |
2019-09-05T14:40:27.9321326Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9321389Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9321423Z 
2019-09-05T14:40:27.9321466Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9321752Z    |
2019-09-05T14:40:27.9321752Z    |
2019-09-05T14:40:27.9321957Z LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9322048Z    |
2019-09-05T14:40:27.9322048Z    |
2019-09-05T14:40:27.9322272Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9322334Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9322360Z 
2019-09-05T14:40:27.9322403Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9323201Z    |
2019-09-05T14:40:27.9323201Z    |
2019-09-05T14:40:27.9323502Z LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9323615Z    |
2019-09-05T14:40:27.9323615Z    |
2019-09-05T14:40:27.9323891Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9323948Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9323997Z 
2019-09-05T14:40:27.9324049Z error[E0658]: the attribute `id` is currently unknown to the compiler and may have meaning added to it in the future
2019-09-05T14:40:27.9324379Z    |
2019-09-05T14:40:27.9324379Z    |
2019-09-05T14:40:27.9324620Z LL |     fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
2019-09-05T14:40:27.9324750Z    |
2019-09-05T14:40:27.9324750Z    |
2019-09-05T14:40:27.9325018Z    = note: for more information, see ***/issues/29642
2019-09-05T14:40:27.9325072Z    = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-09-05T14:40:27.9325164Z error: aborting due to 25 previous errors
2019-09-05T14:40:27.9325194Z 
2019-09-05T14:40:27.9325432Z For more information about this error, try `rustc --explain E0658`.
2019-09-05T14:40:27.9325483Z 
---
2019-09-05T14:40:27.9326499Z test result: FAILED. 8955 passed; 1 failed; 38 ignored; 0 measured; 0 filtered out
2019-09-05T14:40:27.9326714Z 
2019-09-05T14:40:27.9336490Z 
2019-09-05T14:40:27.9336744Z 
2019-09-05T14:40:27.9338185Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-05T14:40:27.9338395Z 
2019-09-05T14:40:27.9338421Z 
2019-09-05T14:40:27.9338701Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-05T14:40:27.9338752Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-05T14:40:27.9338752Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-05T14:40:27.9338799Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-05T14:40:27.9338858Z Build completed unsuccessfully in 1:01:29
2019-09-05T14:40:27.9346781Z == clock drift check ==
2019-09-05T14:40:27.9361995Z   local time: Thu Sep  5 14:40:27 UTC 2019
2019-09-05T14:40:28.0859841Z   network time: Thu, 05 Sep 2019 14:40:28 GMT
2019-09-05T14:40:28.0860715Z == end clock drift check ==
2019-09-05T14:40:28.8569510Z ##[error]Bash exited with code '1'.
2019-09-05T14:40:28.8629188Z ##[section]Starting: Checkout
2019-09-05T14:40:28.8630925Z ==============================================================================
2019-09-05T14:40:28.8630970Z Task         : Get sources
2019-09-05T14:40:28.8631029Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
