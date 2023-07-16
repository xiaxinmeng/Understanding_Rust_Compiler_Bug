
warning: unused variable: `cacher`
  --> src/main.rs:31:37
   |
31 | fn symmetric_uncertainty_cached<'a>(cacher: &mut MyCacher, x: &'a Col<'a>, y: &'a Col<'a>) -> ValueType {
   |                                     ^^^^^^ help: consider using `_cacher` instead
   |
   = note: #[warn(unused_variables)] on by default

error[E0597]: `x` does not live long enough
   --> src/main.rs:106:26
    |
106 |     let cols: Vec<Col> = x.gencolumns()
    |                          ^ borrowed value does not live long enough
...
172 | }
    | - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0597]: `s_list` does not live long enough
   --> src/main.rs:130:27
    |
130 |     while let Some(f_p) = s_list.get(cur_feature_idx) {
    |                           ^^^^^^ borrowed value does not live long enough
...
172 | }
    | - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0597]: `s_list` does not live long enough
   --> src/main.rs:132:26
    |
132 |         let unexplored = s_list.iter().skip(cur_feature_idx + 1).filter(|f_q| {
    |                          ^^^^^^ borrowed value does not live long enough
...
172 | }
    | - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0597]: `f_y` does not live long enough
   --> src/main.rs:132:73
    |
132 |           let unexplored = s_list.iter().skip(cur_feature_idx + 1).filter(|f_q| {
    |  _________________________________________________________________________^
133 | |             assert_ne!(f_p.col.idx, f_q.col.idx);
134 | |             let su_p_q = symmetric_uncertainty_cached(&mut cacher,
135 | |                     &f_q.col,
...   |
141 | |             f_q_su_target > su_p_q
142 | |         });
    | |_________^ borrowed value does not live long enough
...
172 |   }
    |   - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0597]: `c` does not live long enough
   --> src/main.rs:118:64
    |
118 |         let su_i_c = symmetric_uncertainty_cached(&mut cacher, &c, &f_y);
    |                                                                ^^ borrowed value does not live long enough
...
172 | }
    | - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0597]: `f_y` does not live long enough
   --> src/main.rs:118:68
    |
118 |         let su_i_c = symmetric_uncertainty_cached(&mut cacher, &c, &f_y);
    |                                                                    ^^^^ borrowed value does not live long enough
...
172 | }
    | - borrowed value only lives until here
    |
note: borrowed value must be valid for the lifetime 'x as defined on the function body at 91:1...
   --> src/main.rs:91:1
    |
91  | / pub fn fcbf<'x>(
92  | |     x: ArrayView<'x, ValueType, Ix2>,
93  | |     y: ArrayView<'x, ValueType, Ix1>,
94  | |     threshold: f64,
...   |
171 | |     x.select(Axis(1), &feature_views)
172 | | }
    | |_^

error[E0505]: cannot move out of `c` because it is borrowed
   --> src/main.rs:120:38
    |
118 |         let su_i_c = symmetric_uncertainty_cached(&mut cacher, &c, &f_y);
    |                                                                -- borrow of `c` occurs here
119 |         if su_i_c >= threshold {
120 |             s_list.push(Feature{col: c, su_target: su_i_c});
    |                                      ^
    |                                      |
    |                                      move out of `c` occurs here
    |                                      borrow later used here

error[E0502]: cannot borrow `s_list` as mutable because it is also borrowed as immutable
   --> src/main.rs:159:5
    |
130 |     while let Some(f_p) = s_list.get(cur_feature_idx) {
    |                           ------ immutable borrow occurs here
...
159 |     s_list.sort_by(|a, b| {
    |     ^^^^^^
    |     |
    |     mutable borrow occurs here
    |     borrow later used here

error[E0506]: cannot assign to `s_list` because it is borrowed
   --> src/main.rs:153:9
    |
130 |     while let Some(f_p) = s_list.get(cur_feature_idx) {
    |                           ------
    |                           |
    |                           borrow of `s_list` occurs here
    |                           borrow later used here
...
153 |         s_list = new_s_list.clone();
    |         ^^^^^^ assignment to borrowed `s_list` occurs here
