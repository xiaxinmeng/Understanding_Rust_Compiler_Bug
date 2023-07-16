plain
travis_time:end:0a608de0:start=1549292754314724033,finish=1549292757462884458,duration=3148160425
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:00:33] iiiii............................................................................................... 1100/5364
[01:00:35] .................................................................................................... 1200/5364
[01:00:38] .................................................................................................... 1300/5364
[01:00:41] .................................................................................................... 1400/5364
[01:00:43] ...................................F................................................................ 1500/5364
[01:00:49] ..................................................................i................................. 1700/5364
[01:00:53] .................................................................................................... 1800/5364
[01:00:57] .................................................................................................... 1900/5364
[01:01:00] .................................................................................................... 2000/5364
---
[01:03:09] 37 warning: unknown lint: `x5400`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:104:25
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:105:25
[01:03:09] 39    |
[01:03:09] 40 LL |     mod inner { #![warn(x5400)] }
[01:03:09] 
[01:03:09] 42 
[01:03:09] 43 warning: unknown lint: `x5400`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:107:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:107:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:108:12
[01:03:09] 45    |
[01:03:09] 46 LL |     #[warn(x5400)] fn f() { }
[01:03:09] 
[01:03:09] 48 
[01:03:09] 49 warning: unknown lint: `x5400`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:110:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:110:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:111:12
[01:03:09] 51    |
[01:03:09] 52 LL |     #[warn(x5400)] struct S;
[01:03:09] 
[01:03:09] 54 
[01:03:09] 55 warning: unknown lint: `x5400`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:114:12
[01:03:09] 57    |
[01:03:09] 58 LL |     #[warn(x5400)] type T = S;
[01:03:09] 
[01:03:09] 60 
[01:03:09] 61 warning: unknown lint: `x5400`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:116:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:116:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:117:12
[01:03:09] 63    |
[01:03:09] 64 LL |     #[warn(x5400)] impl S { }
[01:03:09] 
[01:03:09] 66 
[01:03:09] 67 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:120:9
---
[01:03:09] 73 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:123:26
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:124:26
[01:03:09] 75    |
[01:03:09] 76 LL |     mod inner { #![allow(x5300)] }
[01:03:09] 
[01:03:09] 78 
[01:03:09] 79 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:126:13
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:126:13
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:127:13
[01:03:09] 81    |
[01:03:09] 82 LL |     #[allow(x5300)] fn f() { }
[01:03:09] 
[01:03:09] 84 
[01:03:09] 85 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:129:13
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:129:13
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:130:13
[01:03:09] 87    |
[01:03:09] 88 LL |     #[allow(x5300)] struct S;
[01:03:09] 
[01:03:09] 90 
[01:03:09] 91 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:132:13
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:132:13
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:133:13
[01:03:09] 93    |
[01:03:09] 94 LL |     #[allow(x5300)] type T = S;
[01:03:09] 
[01:03:09] 96 
[01:03:09] 97 warning: unknown lint: `x5300`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:135:13
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:135:13
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:136:13
[01:03:09] 99    |
[01:03:09] 100 LL |     #[allow(x5300)] impl S { }
[01:03:09] 
[01:03:09] 102 
[01:03:09] 103 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:139:10
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:139:10
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:140:10
[01:03:09] 105    |
[01:03:09] 106 LL | #[forbid(x5200)]
[01:03:09] 
[01:03:09] 108 
[01:03:09] 109 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:142:27
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:142:27
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:143:27
[01:03:09] 111    |
[01:03:09] 112 LL |     mod inner { #![forbid(x5200)] }
[01:03:09] 
[01:03:09] 114 
[01:03:09] 115 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:145:14
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:145:14
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:146:14
[01:03:09] 117    |
[01:03:09] 118 LL |     #[forbid(x5200)] fn f() { }
[01:03:09] 
[01:03:09] 120 
[01:03:09] 121 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:148:14
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:148:14
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:149:14
[01:03:09] 123    |
[01:03:09] 124 LL |     #[forbid(x5200)] struct S;
[01:03:09] 
[01:03:09] 126 
[01:03:09] 127 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:151:14
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:151:14
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:152:14
[01:03:09] 129    |
[01:03:09] 130 LL |     #[forbid(x5200)] type T = S;
[01:03:09] 
[01:03:09] 132 
[01:03:09] 133 warning: unknown lint: `x5200`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:154:14
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:154:14
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:155:14
[01:03:09] 135    |
[01:03:09] 136 LL |     #[forbid(x5200)] impl S { }
[01:03:09] 
[01:03:09] 138 
[01:03:09] 139 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:158:8
---
[01:03:09] 145 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:161:25
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:162:25
[01:03:09] 147    |
[01:03:09] 148 LL |     mod inner { #![deny(x5100)] }
[01:03:09] 
[01:03:09] 150 
[01:03:09] 151 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:164:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:164:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:165:12
[01:03:09] 153    |
[01:03:09] 154 LL |     #[deny(x5100)] fn f() { }
[01:03:09] 
[01:03:09] 156 
[01:03:09] 157 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:167:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:167:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:168:12
[01:03:09] 159    |
[01:03:09] 160 LL |     #[deny(x5100)] struct S;
[01:03:09] 
[01:03:09] 162 
[01:03:09] 163 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:170:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:170:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:171:12
[01:03:09] 165    |
[01:03:09] 166 LL |     #[deny(x5100)] type T = S;
[01:03:09] 
[01:03:09] 168 
[01:03:09] 169 warning: unknown lint: `x5100`
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:173:12
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:173:12
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:174:12
[01:03:09] 171    |
[01:03:09] 172 LL |     #[deny(x5100)] impl S { }
[01:03:09] 
[01:03:09] 174 
[01:03:09] 174 
[01:03:09] 175 warning: `macro_escape` is a deprecated synonym for `macro_use`
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:458:1
[01:03:09] 177    |
[01:03:09] 178 LL | #[macro_escape]
[01:03:09] 179    | ^^^^^^^^^^^^^^^
[01:03:09] 179    | ^^^^^^^^^^^^^^^
[01:03:09] 
[01:03:09] 180 
[01:03:09] 181 warning: `macro_escape` is a deprecated synonym for `macro_use`
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:461:17
[01:03:09] 183    |
[01:03:09] 183    |
[01:03:09] 184 LL |     mod inner { #![macro_escape] }
[01:03:09] 
[01:03:09] 
[01:03:09] 187    = help: consider an outer attribute, `#[macro_use] mod ...`
[01:03:09] 189 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:181:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:182:5
[01:03:09] 191    |
[01:03:09] 191    |
[01:03:09] 192 LL |     #[macro_use] fn f() { }
[01:03:09] 
[01:03:09] 199    |         ^^^^^^^^^^^^^^^^^
[01:03:09] 200 
[01:03:09] 201 warning: unused attribute
---
[01:03:09] 207 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:187:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:188:5
[01:03:09] 209    |
[01:03:09] 210 LL |     #[macro_use] type T = S;
[01:03:09] 
[01:03:09] 212 
[01:03:09] 213 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:190:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:190:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:191:5
[01:03:09] 215    |
[01:03:09] 216 LL |     #[macro_use] impl S { }
[01:03:09] 
[01:03:09] 218 
[01:03:09] 219 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:197:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:197:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:198:17
[01:03:09] 221    |
[01:03:09] 222 LL |     mod inner { #![macro_export] }
[01:03:09] 
[01:03:09] 224 
[01:03:09] 225 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:200:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:200:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:201:5
[01:03:09] 227    |
[01:03:09] 228 LL |     #[macro_export] fn f() { }
[01:03:09] 
[01:03:09] 230 
[01:03:09] 231 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:203:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:203:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:204:5
[01:03:09] 233    |
[01:03:09] 234 LL |     #[macro_export] struct S;
[01:03:09] 
[01:03:09] 236 
[01:03:09] 237 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:206:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:206:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:207:5
[01:03:09] 239    |
[01:03:09] 240 LL |     #[macro_export] type T = S;
[01:03:09] 
[01:03:09] 242 
[01:03:09] 243 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:209:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:209:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:210:5
[01:03:09] 245    |
[01:03:09] 246 LL |     #[macro_export] impl S { }
[01:03:09] 
[01:03:09] 248 
[01:03:09] 249 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:194:1
---
[01:03:09] 255 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:216:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:217:17
[01:03:09] 257    |
[01:03:09] 258 LL |     mod inner { #![plugin_registrar] }
[01:03:09] 
[01:03:09] 260 
[01:03:09] 261 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:221:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:221:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:222:5
[01:03:09] 263    |
[01:03:09] 264 LL |     #[plugin_registrar] struct S;
[01:03:09] 
[01:03:09] 266 
[01:03:09] 267 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:224:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:224:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:225:5
[01:03:09] 269    |
[01:03:09] 270 LL |     #[plugin_registrar] type T = S;
[01:03:09] 
[01:03:09] 272 
[01:03:09] 273 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:227:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:227:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:228:5
[01:03:09] 275    |
[01:03:09] 276 LL |     #[plugin_registrar] impl S { }
[01:03:09] 
[01:03:09] 278 
[01:03:09] 279 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:213:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:213:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:214:1
[01:03:09] 281    |
[01:03:09] 282 LL | #[plugin_registrar]
[01:03:09] 
[01:03:09] 284 
[01:03:09] 285 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:234:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:234:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:235:17
[01:03:09] 287    |
[01:03:09] 288 LL |     mod inner { #![main] }
[01:03:09] 
[01:03:09] 290 
[01:03:09] 291 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:239:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:239:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:240:5
[01:03:09] 293    |
[01:03:09] 294 LL |     #[main] struct S;
[01:03:09] 
[01:03:09] 296 
[01:03:09] 297 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:242:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:242:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:243:5
[01:03:09] 299    |
[01:03:09] 300 LL |     #[main] type T = S;
[01:03:09] 
[01:03:09] 302 
[01:03:09] 303 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:245:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:245:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:246:5
[01:03:09] 305    |
[01:03:09] 306 LL |     #[main] impl S { }
[01:03:09] 
[01:03:09] 308 
[01:03:09] 309 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:231:1
---
[01:03:09] 315 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:252:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:253:17
[01:03:09] 317    |
[01:03:09] 318 LL |     mod inner { #![start] }
[01:03:09] 
[01:03:09] 320 
[01:03:09] 321 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:257:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:257:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:258:5
[01:03:09] 323    |
[01:03:09] 324 LL |     #[start] struct S;
[01:03:09] 
[01:03:09] 326 
[01:03:09] 327 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:260:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:260:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:261:5
[01:03:09] 329    |
[01:03:09] 330 LL |     #[start] type T = S;
[01:03:09] 
[01:03:09] 332 
[01:03:09] 333 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:263:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:263:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:264:5
[01:03:09] 335    |
[01:03:09] 336 LL |     #[start] impl S { }
[01:03:09] 
[01:03:09] 338 
[01:03:09] 339 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:249:1
---
[01:03:09] 345 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:316:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:317:5
[01:03:09] 347    |
[01:03:09] 348 LL |     #[path = "3800"] fn f() { }
[01:03:09] 
[01:03:09] 350 
[01:03:09] 351 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:319:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:319:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:320:5
[01:03:09] 353    |
[01:03:09] 354 LL |     #[path = "3800"]  struct S;
[01:03:09] 
[01:03:09] 356 
[01:03:09] 357 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:322:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:322:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:323:5
[01:03:09] 359    |
[01:03:09] 360 LL |     #[path = "3800"] type T = S;
[01:03:09] 
[01:03:09] 362 
[01:03:09] 363 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:325:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:325:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:326:5
[01:03:09] 365    |
[01:03:09] 366 LL |     #[path = "3800"] impl S { }
[01:03:09] 
[01:03:09] 368 
[01:03:09] 369 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:332:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:332:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:333:17
[01:03:09] 371    |
[01:03:09] 372 LL |     mod inner { #![automatically_derived] }
[01:03:09] 
[01:03:09] 374 
[01:03:09] 375 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:335:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:335:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:336:5
[01:03:09] 377    |
[01:03:09] 378 LL |     #[automatically_derived] fn f() { }
[01:03:09] 
[01:03:09] 380 
[01:03:09] 381 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:338:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:338:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:339:5
[01:03:09] 383    |
[01:03:09] 384 LL |     #[automatically_derived] struct S;
[01:03:09] 
[01:03:09] 386 
[01:03:09] 387 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:341:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:341:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:342:5
[01:03:09] 389    |
[01:03:09] 390 LL |     #[automatically_derived] type T = S;
[01:03:09] 
[01:03:09] 392 
[01:03:09] 393 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:344:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:344:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:345:5
[01:03:09] 395    |
[01:03:09] 396 LL |     #[automatically_derived] impl S { }
[01:03:09] 
[01:03:09] 398 
[01:03:09] 399 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:329:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:329:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:330:1
[01:03:09] 401    |
[01:03:09] 402 LL | #[automatically_derived]
[01:03:09] 
[01:03:09] 404 
[01:03:09] 405 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:364:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:364:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:365:17
[01:03:09] 407    |
[01:03:09] 408 LL |     mod inner { #![no_link] }
[01:03:09] 
[01:03:09] 410 
[01:03:09] 411 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:367:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:367:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:368:5
[01:03:09] 413    |
[01:03:09] 414 LL |     #[no_link] fn f() { }
[01:03:09] 
[01:03:09] 416 
[01:03:09] 417 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:370:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:370:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:371:5
[01:03:09] 419    |
[01:03:09] 420 LL |     #[no_link] struct S;
[01:03:09] 
[01:03:09] 422 
[01:03:09] 423 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:373:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:373:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:374:5
[01:03:09] 425    |
[01:03:09] 426 LL |     #[no_link]type T = S;
[01:03:09] 
[01:03:09] 428 
[01:03:09] 429 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:376:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:376:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:377:5
[01:03:09] 431    |
[01:03:09] 432 LL |     #[no_link] impl S { }
[01:03:09] 
[01:03:09] 434 
[01:03:09] 435 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:361:1
---
[01:03:09] 441 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:383:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:384:17
[01:03:09] 443    |
[01:03:09] 444 LL |     mod inner { #![should_panic] }
[01:03:09] 
[01:03:09] 446 
[01:03:09] 447 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:386:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:386:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:387:5
[01:03:09] 449    |
[01:03:09] 450 LL |     #[should_panic] fn f() { }
[01:03:09] 
[01:03:09] 452 
[01:03:09] 453 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:389:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:389:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:390:5
[01:03:09] 455    |
[01:03:09] 456 LL |     #[should_panic] struct S;
[01:03:09] 
[01:03:09] 458 
[01:03:09] 459 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:392:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:392:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:393:5
[01:03:09] 461    |
[01:03:09] 462 LL |     #[should_panic] type T = S;
[01:03:09] 
[01:03:09] 464 
[01:03:09] 465 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:395:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:395:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:396:5
[01:03:09] 467    |
[01:03:09] 468 LL |     #[should_panic] impl S { }
[01:03:09] 
[01:03:09] 470 
[01:03:09] 471 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:380:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:380:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:381:1
[01:03:09] 473    |
[01:03:09] 474 LL | #[should_panic]
[01:03:09] 
[01:03:09] 476 
[01:03:09] 477 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:402:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:402:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:403:17
[01:03:09] 479    |
[01:03:09] 480 LL |     mod inner { #![ignore] }
[01:03:09] 
[01:03:09] 482 
[01:03:09] 483 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:405:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:405:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:406:5
[01:03:09] 485    |
[01:03:09] 486 LL |     #[ignore] fn f() { }
[01:03:09] 
[01:03:09] 488 
[01:03:09] 489 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:408:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:409:5
[01:03:09] 491    |
[01:03:09] 492 LL |     #[ignore] struct S;
[01:03:09] 
[01:03:09] 494 
[01:03:09] 495 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:411:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:412:5
[01:03:09] 497    |
[01:03:09] 498 LL |     #[ignore] type T = S;
[01:03:09] 
[01:03:09] 500 
[01:03:09] 501 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:414:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:414:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:415:5
[01:03:09] 503    |
[01:03:09] 504 LL |     #[ignore] impl S { }
[01:03:09] 
[01:03:09] 506 
[01:03:09] 507 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:399:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:399:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:400:1
[01:03:09] 509    |
[01:03:09] 510 LL | #[ignore]
[01:03:09] 
[01:03:09] 512 
[01:03:09] 513 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:421:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:422:17
[01:03:09] 515    |
[01:03:09] 516 LL |     mod inner { #![no_implicit_prelude] }
[01:03:09] 
[01:03:09] 518 
[01:03:09] 519 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:424:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:424:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:425:5
[01:03:09] 521    |
[01:03:09] 522 LL |     #[no_implicit_prelude] fn f() { }
[01:03:09] 
[01:03:09] 524 
[01:03:09] 525 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:427:5
---
[01:03:09] 531 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:430:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:431:5
[01:03:09] 533    |
[01:03:09] 534 LL |     #[no_implicit_prelude] type T = S;
[01:03:09] 
[01:03:09] 536 
[01:03:09] 537 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:433:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:434:5
[01:03:09] 539    |
[01:03:09] 540 LL |     #[no_implicit_prelude] impl S { }
[01:03:09] 
[01:03:09] 542 
[01:03:09] 543 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:418:1
---
[01:03:09] 549 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:440:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:441:17
[01:03:09] 551    |
[01:03:09] 552 LL |     mod inner { #![reexport_test_harness_main="2900"] }
[01:03:09] 
[01:03:09] 554 
[01:03:09] 555 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:443:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:443:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:444:5
[01:03:09] 557    |
[01:03:09] 558 LL |     #[reexport_test_harness_main = "2900"] fn f() { }
[01:03:09] 
[01:03:09] 560 
[01:03:09] 561 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:446:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:446:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:447:5
[01:03:09] 563    |
[01:03:09] 564 LL |     #[reexport_test_harness_main = "2900"] struct S;
[01:03:09] 
[01:03:09] 566 
[01:03:09] 567 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:449:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:449:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:450:5
[01:03:09] 569    |
[01:03:09] 570 LL |     #[reexport_test_harness_main = "2900"] type T = S;
[01:03:09] 
[01:03:09] 572 
[01:03:09] 573 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:452:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:452:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:453:5
[01:03:09] 575    |
[01:03:09] 576 LL |     #[reexport_test_harness_main = "2900"] impl S { }
[01:03:09] 
[01:03:09] 578 
[01:03:09] 579 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:437:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:438:1
[01:03:09] 581    |
[01:03:09] 582 LL | #[reexport_test_harness_main = "2900"]
[01:03:09] 
[01:03:09] 584 
[01:03:09] 585 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:463:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:463:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:464:5
[01:03:09] 587    |
[01:03:09] 588 LL |     #[macro_escape] fn f() { }
[01:03:09] 
[01:03:09] 590 
[01:03:09] 591 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:466:5
---
[01:03:09] 597 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:469:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:470:5
[01:03:09] 599    |
[01:03:09] 600 LL |     #[macro_escape] type T = S;
[01:03:09] 
[01:03:09] 602 
[01:03:09] 603 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:472:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:472:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:473:5
[01:03:09] 605    |
[01:03:09] 606 LL |     #[macro_escape] impl S { }
[01:03:09] 
[01:03:09] 608 
[01:03:09] 609 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:480:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:480:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:481:17
[01:03:09] 611    |
[01:03:09] 612 LL |     mod inner { #![no_std] }
[01:03:09] 
[01:03:09] 614 
[01:03:09] 615 warning: crate-level attribute should be in the root module
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:480:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:480:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:481:17
[01:03:09] 617    |
[01:03:09] 618 LL |     mod inner { #![no_std] }
[01:03:09] 
[01:03:09] 620 
[01:03:09] 621 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:484:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:484:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:485:5
[01:03:09] 623    |
[01:03:09] 624 LL |     #[no_std] fn f() { }
[01:03:09] 
[01:03:09] 626 
[01:03:09] 626 
[01:03:09] 627 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:485:5
[01:03:09] 629    |
[01:03:09] 629    |
[01:03:09] 630 LL |     #[no_std] fn f() { }
[01:03:09] 
[01:03:09] 632 
[01:03:09] 633 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:488:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:488:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:489:5
[01:03:09] 635    |
[01:03:09] 636 LL |     #[no_std] struct S;
[01:03:09] 
[01:03:09] 638 
[01:03:09] 638 
[01:03:09] 639 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:489:5
[01:03:09] 641    |
[01:03:09] 641    |
[01:03:09] 642 LL |     #[no_std] struct S;
[01:03:09] 
[01:03:09] 644 
[01:03:09] 645 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:492:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:492:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:493:5
[01:03:09] 647    |
[01:03:09] 648 LL |     #[no_std] type T = S;
[01:03:09] 
[01:03:09] 650 
[01:03:09] 650 
[01:03:09] 651 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:493:5
[01:03:09] 653    |
[01:03:09] 653    |
[01:03:09] 654 LL |     #[no_std] type T = S;
[01:03:09] 
[01:03:09] 656 
[01:03:09] 657 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:496:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:496:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:497:5
[01:03:09] 659    |
[01:03:09] 660 LL |     #[no_std] impl S { }
[01:03:09] 
[01:03:09] 662 
[01:03:09] 662 
[01:03:09] 663 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:497:5
[01:03:09] 665    |
[01:03:09] 665    |
[01:03:09] 666 LL |     #[no_std] impl S { }
[01:03:09] 
[01:03:09] 668 
[01:03:09] 669 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:476:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:477:1
[01:03:09] 671    |
[01:03:09] 672 LL | #[no_std]
[01:03:09] 673    | ^^^^^^^^^
[01:03:09] 
[01:03:09] 674 
[01:03:09] 675 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:477:1
[01:03:09] 677    |
[01:03:09] 678 LL | #[no_std]
[01:03:09] 679    | ^^^^^^^^^
[01:03:09] 679    | ^^^^^^^^^
[01:03:09] 
[01:03:09] 680 
[01:03:09] 681 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:636:17
[01:03:09] 683    |
[01:03:09] 684 LL |     mod inner { #![crate_name="0900"] }
[01:03:09] 
[01:03:09] 686 
[01:03:09] 687 warning: crate-level attribute should be in the root module
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:635:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:636:17
[01:03:09] 689    |
[01:03:09] 690 LL |     mod inner { #![crate_name="0900"] }
[01:03:09] 
[01:03:09] 692 
[01:03:09] 693 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:639:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:640:5
[01:03:09] 695    |
[01:03:09] 696 LL |     #[crate_name = "0900"] fn f() { }
[01:03:09] 
[01:03:09] 698 
[01:03:09] 698 
[01:03:09] 699 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:640:5
[01:03:09] 701    |
[01:03:09] 701    |
[01:03:09] 702 LL |     #[crate_name = "0900"] fn f() { }
[01:03:09] 
[01:03:09] 704 
[01:03:09] 705 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:643:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:643:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:644:5
[01:03:09] 707    |
[01:03:09] 708 LL |     #[crate_name = "0900"] struct S;
[01:03:09] 
[01:03:09] 710 
[01:03:09] 710 
[01:03:09] 711 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:644:5
[01:03:09] 713    |
[01:03:09] 713    |
[01:03:09] 714 LL |     #[crate_name = "0900"] struct S;
[01:03:09] 
[01:03:09] 716 
[01:03:09] 717 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:647:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:647:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:648:5
[01:03:09] 719    |
[01:03:09] 720 LL |     #[crate_name = "0900"] type T = S;
[01:03:09] 
[01:03:09] 722 
[01:03:09] 722 
[01:03:09] 723 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:648:5
[01:03:09] 725    |
[01:03:09] 725    |
[01:03:09] 726 LL |     #[crate_name = "0900"] type T = S;
[01:03:09] 
[01:03:09] 728 
[01:03:09] 729 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:651:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:651:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:652:5
[01:03:09] 731    |
[01:03:09] 732 LL |     #[crate_name = "0900"] impl S { }
[01:03:09] 
[01:03:09] 734 
[01:03:09] 734 
[01:03:09] 735 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:652:5
[01:03:09] 737    |
[01:03:09] 737    |
[01:03:09] 738 LL |     #[crate_name = "0900"] impl S { }
[01:03:09] 
[01:03:09] 740 
[01:03:09] 741 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:631:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:632:1
[01:03:09] 743    |
[01:03:09] 744 LL | #[crate_name = "0900"]
[01:03:09] 
[01:03:09] 746 
[01:03:09] 746 
[01:03:09] 747 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:632:1
[01:03:09] 749    |
[01:03:09] 749    |
[01:03:09] 750 LL | #[crate_name = "0900"]
[01:03:09] 
[01:03:09] 752 
[01:03:09] 753 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:660:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:660:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:661:17
[01:03:09] 755    |
[01:03:09] 756 LL |     mod inner { #![crate_type="0800"] }
[01:03:09] 
[01:03:09] 758 
[01:03:09] 759 warning: crate-level attribute should be in the root module
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:660:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:660:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:661:17
[01:03:09] 761    |
[01:03:09] 762 LL |     mod inner { #![crate_type="0800"] }
[01:03:09] 
[01:03:09] 764 
[01:03:09] 765 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:664:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:664:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:665:5
[01:03:09] 767    |
[01:03:09] 768 LL |     #[crate_type = "0800"] fn f() { }
[01:03:09] 
[01:03:09] 770 
[01:03:09] 770 
[01:03:09] 771 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:665:5
[01:03:09] 773    |
[01:03:09] 773    |
[01:03:09] 774 LL |     #[crate_type = "0800"] fn f() { }
[01:03:09] 
[01:03:09] 776 
[01:03:09] 777 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:668:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:668:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:669:5
[01:03:09] 779    |
[01:03:09] 780 LL |     #[crate_type = "0800"] struct S;
[01:03:09] 
[01:03:09] 782 
[01:03:09] 782 
[01:03:09] 783 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:669:5
[01:03:09] 785    |
[01:03:09] 785    |
[01:03:09] 786 LL |     #[crate_type = "0800"] struct S;
[01:03:09] 
[01:03:09] 788 
[01:03:09] 789 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:672:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:672:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:673:5
[01:03:09] 791    |
[01:03:09] 792 LL |     #[crate_type = "0800"] type T = S;
[01:03:09] 
[01:03:09] 794 
[01:03:09] 794 
[01:03:09] 795 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:673:5
[01:03:09] 797    |
[01:03:09] 797    |
[01:03:09] 798 LL |     #[crate_type = "0800"] type T = S;
[01:03:09] 
[01:03:09] 800 
[01:03:09] 801 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:676:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:676:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:677:5
[01:03:09] 803    |
[01:03:09] 804 LL |     #[crate_type = "0800"] impl S { }
[01:03:09] 
[01:03:09] 806 
[01:03:09] 806 
[01:03:09] 807 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:677:5
[01:03:09] 809    |
[01:03:09] 809    |
[01:03:09] 810 LL |     #[crate_type = "0800"] impl S { }
[01:03:09] 
[01:03:09] 812 
[01:03:09] 813 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:656:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:656:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:657:1
[01:03:09] 815    |
[01:03:09] 816 LL | #[crate_type = "0800"]
[01:03:09] 
[01:03:09] 818 
[01:03:09] 818 
[01:03:09] 819 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:657:1
[01:03:09] 821    |
[01:03:09] 821    |
[01:03:09] 822 LL | #[crate_type = "0800"]
[01:03:09] 
[01:03:09] 824 
[01:03:09] 825 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:685:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:685:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:686:17
[01:03:09] 827    |
[01:03:09] 828 LL |     mod inner { #![feature(x0600)] }
[01:03:09] 
[01:03:09] 830 
[01:03:09] 831 warning: crate-level attribute should be in the root module
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:685:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:685:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:686:17
[01:03:09] 833    |
[01:03:09] 834 LL |     mod inner { #![feature(x0600)] }
[01:03:09] 
[01:03:09] 836 
[01:03:09] 837 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:689:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:689:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:690:5
[01:03:09] 839    |
[01:03:09] 840 LL |     #[feature(x0600)] fn f() { }
[01:03:09] 
[01:03:09] 842 
[01:03:09] 842 
[01:03:09] 843 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:690:5
[01:03:09] 845    |
[01:03:09] 845    |
[01:03:09] 846 LL |     #[feature(x0600)] fn f() { }
[01:03:09] 
[01:03:09] 848 
[01:03:09] 849 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:693:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:693:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:694:5
[01:03:09] 851    |
[01:03:09] 852 LL |     #[feature(x0600)] struct S;
[01:03:09] 
[01:03:09] 854 
[01:03:09] 854 
[01:03:09] 855 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:694:5
[01:03:09] 857    |
[01:03:09] 857    |
[01:03:09] 858 LL |     #[feature(x0600)] struct S;
[01:03:09] 
[01:03:09] 860 
[01:03:09] 861 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:697:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:697:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:698:5
[01:03:09] 863    |
[01:03:09] 864 LL |     #[feature(x0600)] type T = S;
[01:03:09] 
[01:03:09] 866 
[01:03:09] 866 
[01:03:09] 867 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:698:5
[01:03:09] 869    |
[01:03:09] 869    |
[01:03:09] 870 LL |     #[feature(x0600)] type T = S;
[01:03:09] 
[01:03:09] 872 
[01:03:09] 873 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:701:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:701:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:702:5
[01:03:09] 875    |
[01:03:09] 876 LL |     #[feature(x0600)] impl S { }
[01:03:09] 
[01:03:09] 878 
[01:03:09] 878 
[01:03:09] 879 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:702:5
[01:03:09] 881    |
[01:03:09] 881    |
[01:03:09] 882 LL |     #[feature(x0600)] impl S { }
[01:03:09] 
[01:03:09] 884 
[01:03:09] 885 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:681:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:681:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:682:1
[01:03:09] 887    |
[01:03:09] 888 LL | #[feature(x0600)]
[01:03:09] 
[01:03:09] 890 
[01:03:09] 890 
[01:03:09] 891 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:682:1
[01:03:09] 893    |
[01:03:09] 893    |
[01:03:09] 894 LL | #[feature(x0600)]
[01:03:09] 
[01:03:09] 896 
[01:03:09] 897 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:711:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:711:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:712:17
[01:03:09] 899    |
[01:03:09] 900 LL |     mod inner { #![no_main] }
[01:03:09] 
[01:03:09] 902 
[01:03:09] 903 warning: crate-level attribute should be in the root module
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:711:17
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:711:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:712:17
[01:03:09] 905    |
[01:03:09] 906 LL |     mod inner { #![no_main] }
[01:03:09] 
[01:03:09] 908 
[01:03:09] 909 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:715:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:715:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:716:5
[01:03:09] 911    |
[01:03:09] 912 LL |     #[no_main] fn f() { }
[01:03:09] 
[01:03:09] 914 
[01:03:09] 914 
[01:03:09] 915 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:716:5
[01:03:09] 917    |
[01:03:09] 917    |
[01:03:09] 918 LL |     #[no_main] fn f() { }
[01:03:09] 
[01:03:09] 920 
[01:03:09] 921 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:719:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:719:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:720:5
[01:03:09] 923    |
[01:03:09] 924 LL |     #[no_main] struct S;
[01:03:09] 
[01:03:09] 926 
[01:03:09] 926 
[01:03:09] 927 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:720:5
[01:03:09] 929    |
[01:03:09] 929    |
[01:03:09] 930 LL |     #[no_main] struct S;
[01:03:09] 
[01:03:09] 932 
[01:03:09] 933 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:723:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:723:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:724:5
[01:03:09] 935    |
[01:03:09] 936 LL |     #[no_main] type T = S;
[01:03:09] 
[01:03:09] 938 
[01:03:09] 938 
[01:03:09] 939 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:724:5
[01:03:09] 941    |
[01:03:09] 941    |
[01:03:09] 942 LL |     #[no_main] type T = S;
[01:03:09] 
[01:03:09] 944 
[01:03:09] 945 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:727:5
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:727:5
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:728:5
[01:03:09] 947    |
[01:03:09] 948 LL |     #[no_main] impl S { }
[01:03:09] 
[01:03:09] 950 
[01:03:09] 950 
[01:03:09] 951 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:728:5
[01:03:09] 953    |
[01:03:09] 953    |
[01:03:09] 954 LL |     #[no_main] impl S { }
[01:03:09] 
[01:03:09] 956 
[01:03:09] 957 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:707:1
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:707:1
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:708:1
[01:03:09] 959    |
[01:03:09] 960 LL | #[no_main]
[01:03:09] 961    | ^^^^^^^^^^
[01:03:09] 
[01:03:09] 962 
[01:03:09] 963 warning: crate-level attribute should be an inner attribute: add an exclamation mark: #![foo]
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:708:1
[01:03:09] 965    |
[01:03:09] 966 LL | #[no_main]
[01:03:09] 967    | ^^^^^^^^^^
[01:03:09] 967    | ^^^^^^^^^^
[01:03:09] 
[01:03:09] 968 
[01:03:09] 969 warning: unused attribute
[01:03:09] -   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:749:17
[01:03:09] +   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:750:17
[01:03:09] 971    |
---
[01:03:09] test result: FAILED. 5340 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out
[01:03:09] 
[01:03:09] 
[01:03:09] 
[01:03:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:09] 
[01:03:09] 
[01:03:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:09] Build completed unsuccessfully in 0:04:16
[01:03:09] Build completed unsuccessfully in 0:04:16
[01:03:09] Makefile:48: recipe for target 'check' failed
[01:03:09] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0674579c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 16:09:17 UTC 2019
---
travis_time:end:0423f960:start=1549296558490315421,finish=1549296558495907909,duration=5592488
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:120910d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a190d69
travis_time:start:0a190d69
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00313e08
$ dmesg | grep -i kill
