plain
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:15:29
+   --> $DIR/const-int-unchecked.rs:14:29
3    |
4 LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
5    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
6 
7 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:17:31
+   --> $DIR/const-int-unchecked.rs:16:31
+   --> $DIR/const-int-unchecked.rs:16:31
9    |
10 LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
11    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
12 
13 error[E0080]: evaluation of constant value failed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/const-int-unchecked.rs:19:31
-   --> $DIR/const-int-unchecked.rs:19:31
+   --> $DIR/const-int-unchecked.rs:18:31
15    |
16 LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
17    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
18 
19 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:21:31
+   --> $DIR/const-int-unchecked.rs:20:31
+   --> $DIR/const-int-unchecked.rs:20:31
21    |
22 LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
23    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
24 
25 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:23:33
+   --> $DIR/const-int-unchecked.rs:22:33
+   --> $DIR/const-int-unchecked.rs:22:33
27    |
28 LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
29    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
30 
31 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:28:29
+   --> $DIR/const-int-unchecked.rs:27:29
+   --> $DIR/const-int-unchecked.rs:27:29
33    |
34 LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
35    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
36 
37 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:30:31
+   --> $DIR/const-int-unchecked.rs:29:31
+   --> $DIR/const-int-unchecked.rs:29:31
39    |
40 LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
41    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
42 
43 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:32:31
+   --> $DIR/const-int-unchecked.rs:31:31
+   --> $DIR/const-int-unchecked.rs:31:31
45    |
46 LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
47    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
48 
49 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:34:31
+   --> $DIR/const-int-unchecked.rs:33:31
+   --> $DIR/const-int-unchecked.rs:33:31
51    |
52 LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
53    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
54 
55 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:36:33
+   --> $DIR/const-int-unchecked.rs:35:33
+   --> $DIR/const-int-unchecked.rs:35:33
57    |
58 LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
59    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
60 
61 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:41:33
+   --> $DIR/const-int-unchecked.rs:40:33
+   --> $DIR/const-int-unchecked.rs:40:33
63    |
64 LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
65    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shl`
66 
67 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:43:35
+   --> $DIR/const-int-unchecked.rs:42:35
+   --> $DIR/const-int-unchecked.rs:42:35
69    |
70 LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
71    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shl`
72 
73 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:45:35
+   --> $DIR/const-int-unchecked.rs:44:35
+   --> $DIR/const-int-unchecked.rs:44:35
75    |
76 LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
77    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shl`
78 
79 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:47:35
+   --> $DIR/const-int-unchecked.rs:46:35
+   --> $DIR/const-int-unchecked.rs:46:35
81    |
82 LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
83    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shl`
84 
85 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:49:37
+   --> $DIR/const-int-unchecked.rs:48:37
+   --> $DIR/const-int-unchecked.rs:48:37
87    |
88 LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
89    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
90 
91 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:55:40
+   --> $DIR/const-int-unchecked.rs:54:40
+   --> $DIR/const-int-unchecked.rs:54:40
93    |
94 LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
95    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shl`
96 
97 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:57:42
+   --> $DIR/const-int-unchecked.rs:56:42
+   --> $DIR/const-int-unchecked.rs:56:42
99    |
100 LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
101    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65523 in `unchecked_shl`
102 
103 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:59:42
+   --> $DIR/const-int-unchecked.rs:58:42
+   --> $DIR/const-int-unchecked.rs:58:42
105    |
106 LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
107    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967271 in `unchecked_shl`
108 
109 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:61:42
+   --> $DIR/const-int-unchecked.rs:60:42
+   --> $DIR/const-int-unchecked.rs:60:42
111    |
112 LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
113    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551586 in `unchecked_shl`
114 
115 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:63:44
+   --> $DIR/const-int-unchecked.rs:62:44
+   --> $DIR/const-int-unchecked.rs:62:44
117    |
118 LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
119    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
120 
121 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:70:29
+   --> $DIR/const-int-unchecked.rs:69:29
+   --> $DIR/const-int-unchecked.rs:69:29
123    |
124 LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
125    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
126 
127 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:72:31
+   --> $DIR/const-int-unchecked.rs:71:31
+   --> $DIR/const-int-unchecked.rs:71:31
129    |
130 LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
131    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
132 
133 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:74:31
+   --> $DIR/const-int-unchecked.rs:73:31
+   --> $DIR/const-int-unchecked.rs:73:31
135    |
136 LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
137    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
138 
139 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:76:31
+   --> $DIR/const-int-unchecked.rs:75:31
+   --> $DIR/const-int-unchecked.rs:75:31
141    |
142 LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
143    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
144 
145 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:78:33
+   --> $DIR/const-int-unchecked.rs:77:33
+   --> $DIR/const-int-unchecked.rs:77:33
147    |
148 LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
149    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
150 
151 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:83:29
+   --> $DIR/const-int-unchecked.rs:82:29
+   --> $DIR/const-int-unchecked.rs:82:29
153    |
154 LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
155    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
156 
157 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:85:31
+   --> $DIR/const-int-unchecked.rs:84:31
+   --> $DIR/const-int-unchecked.rs:84:31
159    |
160 LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
161    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
162 
163 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:87:31
+   --> $DIR/const-int-unchecked.rs:86:31
+   --> $DIR/const-int-unchecked.rs:86:31
165    |
166 LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
167    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
168 
169 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:89:31
+   --> $DIR/const-int-unchecked.rs:88:31
+   --> $DIR/const-int-unchecked.rs:88:31
171    |
172 LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
173    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
174 
175 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:91:33
+   --> $DIR/const-int-unchecked.rs:90:33
+   --> $DIR/const-int-unchecked.rs:90:33
177    |
178 LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
179    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
180 
181 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:96:33
+   --> $DIR/const-int-unchecked.rs:95:33
+   --> $DIR/const-int-unchecked.rs:95:33
183    |
184 LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
185    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shr`
186 
187 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:98:35
+   --> $DIR/const-int-unchecked.rs:97:35
+   --> $DIR/const-int-unchecked.rs:97:35
189    |
190 LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
191    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shr`
192 
193 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:100:35
+   --> $DIR/const-int-unchecked.rs:99:35
+   --> $DIR/const-int-unchecked.rs:99:35
195    |
196 LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
197    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shr`
198 
199 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:102:35
+   --> $DIR/const-int-unchecked.rs:101:35
+   --> $DIR/const-int-unchecked.rs:101:35
201    |
202 LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
203    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shr`
204 
205 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:104:37
+   --> $DIR/const-int-unchecked.rs:103:37
+   --> $DIR/const-int-unchecked.rs:103:37
207    |
208 LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
209    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
210 
211 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:110:40
+   --> $DIR/const-int-unchecked.rs:109:40
+   --> $DIR/const-int-unchecked.rs:109:40
213    |
214 LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
215    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shr`
216 
217 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:112:42
+   --> $DIR/const-int-unchecked.rs:111:42
+   --> $DIR/const-int-unchecked.rs:111:42
219    |
220 LL | const SHR_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shr(5_16, -13) };
221    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65523 in `unchecked_shr`
222 
223 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:114:42
+   --> $DIR/const-int-unchecked.rs:113:42
+   --> $DIR/const-int-unchecked.rs:113:42
225    |
226 LL | const SHR_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -25) };
227    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967271 in `unchecked_shr`
228 
229 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:116:42
+   --> $DIR/const-int-unchecked.rs:115:42
+   --> $DIR/const-int-unchecked.rs:115:42
231    |
232 LL | const SHR_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -30) };
233    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551586 in `unchecked_shr`
234 
235 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:118:44
+   --> $DIR/const-int-unchecked.rs:117:44
+   --> $DIR/const-int-unchecked.rs:117:44
237    |
238 LL | const SHR_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -93) };
239    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shr`
240 
241 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:123:25
+   --> $DIR/const-int-unchecked.rs:122:25
+   --> $DIR/const-int-unchecked.rs:122:25
243    |
244 LL | const _: u16 = unsafe { std::intrinsics::unchecked_add(40000u16, 30000) };
245    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_add`
246 
247 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:126:25
+   --> $DIR/const-int-unchecked.rs:125:25
+   --> $DIR/const-int-unchecked.rs:125:25
249    |
250 LL | const _: u32 = unsafe { std::intrinsics::unchecked_sub(14u32, 22) };
251    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_sub`
252 
253 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:129:25
+   --> $DIR/const-int-unchecked.rs:128:25
+   --> $DIR/const-int-unchecked.rs:128:25
255    |
256 LL | const _: u16 = unsafe { std::intrinsics::unchecked_mul(300u16, 250u16) };
257    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_mul`
258 
259 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:132:25
+   --> $DIR/const-int-unchecked.rs:131:25
+   --> $DIR/const-int-unchecked.rs:131:25
261    |
262 LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(1, 0) };
263    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dividing by zero
264 
265 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:134:25
+   --> $DIR/const-int-unchecked.rs:133:25
+   --> $DIR/const-int-unchecked.rs:133:25
267    |
268 LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
269    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow in signed division (dividing MIN by -1)
270 
271 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:137:25
+   --> $DIR/const-int-unchecked.rs:136:25
+   --> $DIR/const-int-unchecked.rs:136:25
273    |
274 LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(1, 0) };
275    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ calculating the remainder with a divisor of zero
276 
277 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:139:25
+   --> $DIR/const-int-unchecked.rs:138:25
+   --> $DIR/const-int-unchecked.rs:138:25
279    |
280 LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
281    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow in signed remainder (dividing MIN by -1)
282 
283 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:144:25
+   --> $DIR/const-int-unchecked.rs:143:25
+   --> $DIR/const-int-unchecked.rs:143:25
285    |
286 LL | const _: u32 = unsafe { std::intrinsics::ctlz_nonzero(0) };
287    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ctlz_nonzero` called on 0
288 
289 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-int-unchecked.rs:146:25
+   --> $DIR/const-int-unchecked.rs:145:25
+   --> $DIR/const-int-unchecked.rs:145:25
291    |
292 LL | const _: u32 = unsafe { std::intrinsics::cttz_nonzero(0) };
293    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `cttz_nonzero` called on 0

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/const-int-unchecked.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-unchecked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:16:31
   |
   |
LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:18:31
   |
   |
LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:20:31
   |
   |
LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:22:33
   |
   |
LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:27:29
   |
   |
LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:29:31
   |
   |
LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:31:31
   |
   |
LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:33:31
   |
   |
LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:35:33
   |
   |
LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:40:33
   |
   |
LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:42:35
   |
   |
LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:44:35
   |
   |
LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:46:35
   |
   |
LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:48:37
   |
   |
LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:54:40
   |
   |
LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:56:42
