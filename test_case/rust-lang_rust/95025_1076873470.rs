plain
........................................................iii......................................... 12700/12761
.............................................................
failures:

---- [ui] ui/deprecated-safe/deprecated-safe.rs stdout ----


1 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:27:34
3    |
3    |
4 LL |     let fn_impl: Box<dyn Fn()> = Box::new(foo0);

12    = note: reason
13 
13 
14 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/deprecated-safe.rs:28:37
16    |
16    |
17 LL |     let fn_impl: Box<dyn FnMut()> = Box::new(foo0);

20    = note: reason
21 
21 
22 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:29:38
24    |
24    |
25 LL |     let fn_impl: Box<dyn FnOnce()> = Box::new(foo0);

28    = note: reason
29 
29 
30 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:31:34
32    |
32    |
33 LL |     let fn_impl: Box<dyn Fn()> = Box::new(foo0);

36    = note: reason
37 
37 
38 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:32:37
40    |
40    |
41 LL |     let fn_impl: Box<dyn FnMut()> = Box::new(foo0);

44    = note: reason
45 
45 
46 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:33:38
48    |
48    |
49 LL |     let fn_impl: Box<dyn FnOnce()> = Box::new(foo0);

52    = note: reason
53 
53 
54 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:36:34
56    |
56    |
57 LL |     let fn_impl: Box<dyn Fn()> = Box::new(foo1);

60    = note: reason
61 
61 
62 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:37:37
64    |
64    |
65 LL |     let fn_impl: Box<dyn FnMut()> = Box::new(foo1);

68    = note: reason
69 
69 
70 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:38:38
72    |
72    |
73 LL |     let fn_impl: Box<dyn FnOnce()> = Box::new(foo1);

76    = note: reason
77 
77 
78 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:39:34
80    |
80    |
81 LL |     let fn_impl: Box<dyn Fn()> = Box::new(foo1);

84    = note: reason
85 
85 
86 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:40:37
88    |
88    |
89 LL |     let fn_impl: Box<dyn FnMut()> = Box::new(foo1);

92    = note: reason
93 
93 
94 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:41:38
96    |
96    |
97 LL |     let fn_impl: Box<dyn FnOnce()> = Box::new(foo1);

100    = note: reason
101 
101 
102 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:43:37
104    |
104    |
105 LL |     let fn_impl: Box<dyn Fn(u64)> = Box::new(foo2);

108    = note: reason
109 
109 
110 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:44:40
112    |
112    |
113 LL |     let fn_impl: Box<dyn FnMut(u64)> = Box::new(foo2);

116    = note: reason
117 
117 
118 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:45:41
120    |
120    |
121 LL |     let fn_impl: Box<dyn FnOnce(u64)> = Box::new(foo2);

124    = note: reason
125 
125 
126 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:46:37
128    |
128    |
129 LL |     let fn_impl: Box<dyn Fn(u64)> = Box::new(foo2);

132    = note: reason
133 
133 
134 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:47:40
136    |
136    |
137 LL |     let fn_impl: Box<dyn FnMut(u64)> = Box::new(foo2);

140    = note: reason
141 
141 
142 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:48:41
144    |
144    |
145 LL |     let fn_impl: Box<dyn FnOnce(u64)> = Box::new(foo2);

148    = note: reason
149 
149 
150 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:50:52
152    |
152    |
153 LL |     let fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

156    = note: reason
157 
157 
158 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:50:52
160    |
160    |
161 LL |     let fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

164    = note: reason
165 
165 
166 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:52:55
168    |
168    |
169 LL |     let fn_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

172    = note: reason
173 
173 
174 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:52:55
176    |
176    |
177 LL |     let fn_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

180    = note: reason
181 
181 
182 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:54:56
184    |
184    |
185 LL |     let fn_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

188    = note: reason
189 
189 
190 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:54:56
192    |
192    |
193 LL |     let fn_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

196    = note: reason
197 
197 
198 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:56:52
200    |
200    |
201 LL |     let fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

204    = note: reason
205 
205 
206 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:56:52
208    |
208    |
209 LL |     let fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

212    = note: reason
213 
213 
214 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:58:55
216    |
216    |
217 LL |     let fn_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

220    = note: reason
221 
221 
222 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:58:55
224    |
224    |
225 LL |     let fn_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

228    = note: reason
229 
229 
230 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:60:56
232    |
232    |
233 LL |     let fn_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

236    = note: reason
237 
237 
238 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:60:56
240    |
240    |
241 LL |     let fn_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

244    = note: reason
245 
245 
246 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:63:27
248    |
248    |
249 LL |     fn_taking_dyn_fn_impl(Box::new(set_var));

252    = note: reason
253 
253 
254 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:63:27
256    |
256    |
257 LL |     fn_taking_dyn_fn_impl(Box::new(set_var));

260    = note: reason
261 
261 
262 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:65:30
264    |
264    |
265 LL |     fn_taking_dyn_fnmut_impl(Box::new(set_var));

268    = note: reason
269 
269 
270 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:65:30
272    |
272    |
273 LL |     fn_taking_dyn_fnmut_impl(Box::new(set_var));

276    = note: reason
277 
277 
278 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:67:31
280    |
280    |
281 LL |     fn_taking_dyn_fnonce_impl(Box::new(set_var));

284    = note: reason
285 
285 
286 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:67:31
288    |
288    |
289 LL |     fn_taking_dyn_fnonce_impl(Box::new(set_var));

292    = note: reason
293 
293 
294 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:69:27
296    |
296    |
297 LL |     fn_taking_dyn_fn_impl(Box::new(set_var));

300    = note: reason
301 
301 
302 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:69:27
304    |
304    |
305 LL |     fn_taking_dyn_fn_impl(Box::new(set_var));

308    = note: reason
309 
309 
310 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:71:30
312    |
312    |
313 LL |     fn_taking_dyn_fnmut_impl(Box::new(set_var));

316    = note: reason
317 
317 
318 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:71:30
320    |
320    |
321 LL |     fn_taking_dyn_fnmut_impl(Box::new(set_var));

324    = note: reason
325 
325 
326 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:73:31
328    |
328    |
329 LL |     fn_taking_dyn_fnonce_impl(Box::new(set_var));

332    = note: reason
333 
333 
334 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:73:31
336    |
336    |
337 LL |     fn_taking_dyn_fnonce_impl(Box::new(set_var));

340    = note: reason
341 
341 
342 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:76:5
344    |
344    |
345 LL |     fn_taking_fn_impl(set_var);

348    = note: reason
349 
349 
350 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:77:5
352    |
352    |
353 LL |     fn_taking_fnmut_impl(set_var);

356    = note: reason
357 
357 
358 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:78:5
360    |
360    |
361 LL |     fn_taking_fnonce_impl(set_var);

364    = note: reason
365 
365 
366 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:79:5
368    |
368    |
369 LL |     fn_taking_fn_impl(set_var);

372    = note: reason
373 
373 
374 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:80:5
376    |
376    |
377 LL |     fn_taking_fnmut_impl(set_var);

380    = note: reason
381 
381 
382 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:81:5
384    |
384    |
385 LL |     fn_taking_fnonce_impl(set_var);

388    = note: reason
389 
389 
390 error: unsafe fn() that has been deprecated as being safe and... must not be coerced to safe fn() pointer?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:91:50
392    |
392    |
393 LL |     let set_var_fn_ptr: fn(OsString, OsString) = set_var;

396    = note: reason
397 
397 
398 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:92:60
400    |
400    |
401 LL |     let set_var_fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

404    = note: reason
405 
405 
406 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:92:60
408    |
408    |
409 LL |     let set_var_fn_impl: Box<dyn Fn(OsString, OsString)> = Box::new(set_var);

412    = note: reason
413 
413 
414 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:94:70
416    |
416    |
417 LL |     let mut set_var_fnmut_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

420    = note: reason
421 
421 
422 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:94:70
424    |
424    |
425 LL |     let mut set_var_fnmut_impl: Box<dyn FnMut(OsString, OsString)> = Box::new(set_var);

428    = note: reason
429 
429 
430 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:96:68
432    |
432    |
433 LL |     let set_var_fnonce_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

436    = note: reason
437 
437 
438 error: Fn() impl of unsafe fn that has been deprecated as being safe and... must not be cast to the Fn() family?? (error E0FIXME)
-   --> $DIR/deprecated-safe.rs:96:68
440    |
440    |
441 LL |     let set_var_fnonce_impl: Box<dyn FnOnce(OsString, OsString)> = Box::new(set_var);

