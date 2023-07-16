plain
2019-07-27T03:15:32.4971397Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T03:15:32.4971469Z 
2019-07-27T03:15:32.4971719Z   git checkout -b <new-branch-name>
2019-07-27T03:15:32.4971765Z 
2019-07-27T03:15:32.4972098Z HEAD is now at cca247b94 Auto merge of #63030 - Mark-Simulacrum:rollup-3yw3i09, r=Mark-Simulacrum
2019-07-27T03:15:32.5112904Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T03:15:32.5116168Z ==============================================================================
2019-07-27T03:15:32.5116277Z Task         : Bash
2019-07-27T03:15:32.5116351Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T04:58:15.4600217Z 
2019-07-27T04:58:15.4602019Z ---- [ui (nll)] ui/borrowck/borrowck-describe-lvalue.rs stdout ----
2019-07-27T04:58:15.4602588Z diff of stderr:
2019-07-27T04:58:15.4602780Z 
2019-07-27T04:58:15.4602940Z 192 LL |         let x = &mut v;
2019-07-27T04:58:15.4603528Z 193    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4603635Z 194 LL |         match v {
2019-07-27T04:58:15.4603923Z - LL |             &[x..] => println!("{:?}", x),
2019-07-27T04:58:15.4604206Z -    |               ^ use of borrowed `v`
2019-07-27T04:58:15.4604408Z + LL |             &[x @ ..] => println!("{:?}", x),
2019-07-27T04:58:15.4604497Z +    |               ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4604561Z 197 ...
2019-07-27T04:58:15.4604645Z 198 LL |         drop(x);
2019-07-27T04:58:15.4604902Z 199    |              - borrow later used here
2019-07-27T04:58:15.4605024Z 204 LL |         let x = &mut v;
2019-07-27T04:58:15.4605024Z 204 LL |         let x = &mut v;
2019-07-27T04:58:15.4605304Z 205    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4605376Z 206 ...
2019-07-27T04:58:15.4606002Z - LL |             &[_, x..] => println!("{:?}", x),
2019-07-27T04:58:15.4606415Z -    |                  ^ use of borrowed `v`
2019-07-27T04:58:15.4606506Z + LL |             &[_, x @ ..] => println!("{:?}", x),
2019-07-27T04:58:15.4608353Z +    |                  ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4608809Z 209 ...
2019-07-27T04:58:15.4608870Z 210 LL |         drop(x);
2019-07-27T04:58:15.4609271Z 211    |              - borrow later used here
2019-07-27T04:58:15.4609382Z 216 LL |         let x = &mut v;
2019-07-27T04:58:15.4609382Z 216 LL |         let x = &mut v;
2019-07-27T04:58:15.4609677Z 217    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4609765Z 218 ...
2019-07-27T04:58:15.4610046Z - LL |             &[x.., _] => println!("{:?}", x),
2019-07-27T04:58:15.4610304Z -    |               ^ use of borrowed `v`
2019-07-27T04:58:15.4610399Z + LL |             &[x @ .., _] => println!("{:?}", x),
2019-07-27T04:58:15.4610487Z +    |               ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4610553Z 221 ...
2019-07-27T04:58:15.4610626Z 222 LL |         drop(x);
2019-07-27T04:58:15.4611440Z 223    |              - borrow later used here
2019-07-27T04:58:15.4611589Z 228 LL |         let x = &mut v;
2019-07-27T04:58:15.4611589Z 228 LL |         let x = &mut v;
2019-07-27T04:58:15.4611871Z 229    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4611961Z 230 ...
2019-07-27T04:58:15.4612222Z - LL |             &[_, x.., _] => println!("{:?}", x),
2019-07-27T04:58:15.4612501Z -    |                  ^ use of borrowed `v`
2019-07-27T04:58:15.4612579Z + LL |             &[_, x @ .., _] => println!("{:?}", x),
2019-07-27T04:58:15.4612670Z +    |                  ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4612919Z 233 ...
2019-07-27T04:58:15.4612995Z 234 LL |         drop(x);
2019-07-27T04:58:15.4613293Z 235    |              - borrow later used here
2019-07-27T04:58:15.4613392Z 
2019-07-27T04:58:15.4613457Z The actual stderr differed from the expected stderr.
2019-07-27T04:58:15.4613457Z The actual stderr differed from the expected stderr.
2019-07-27T04:58:15.4613862Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.nll/borrowck-describe-lvalue.nll.stderr
2019-07-27T04:58:15.4614210Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T04:58:15.4614544Z To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
2019-07-27T04:58:15.4614682Z error: 1 errors occurred comparing output.
2019-07-27T04:58:15.4614762Z status: exit code: 1
2019-07-27T04:58:15.4614762Z status: exit code: 1
2019-07-27T04:58:15.4615684Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.nll/auxiliary" "-A" "unused"
2019-07-27T04:58:15.4616231Z ------------------------------------------
2019-07-27T04:58:15.4616282Z 
2019-07-27T04:58:15.4616531Z ------------------------------------------
2019-07-27T04:58:15.4616616Z stderr:
2019-07-27T04:58:15.4616616Z stderr:
2019-07-27T04:58:15.4616859Z ------------------------------------------
2019-07-27T04:58:15.4616955Z error[E0499]: cannot borrow `x` as mutable more than once at a time
2019-07-27T04:58:15.4617359Z    |
2019-07-27T04:58:15.4617359Z    |
2019-07-27T04:58:15.4617434Z LL |             let y = &mut x;
2019-07-27T04:58:15.4617717Z    |                     ------ first mutable borrow occurs here
2019-07-27T04:58:15.4617824Z LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-07-27T04:58:15.4617911Z    |             ^^^^^^ second mutable borrow occurs here
2019-07-27T04:58:15.4618000Z LL |             *y = 1;
2019-07-27T04:58:15.4618263Z    |             ------ first borrow later used here
2019-07-27T04:58:15.4618327Z 
2019-07-27T04:58:15.4618485Z error[E0499]: cannot borrow `x` as mutable more than once at a time
2019-07-27T04:58:15.4618908Z    |
2019-07-27T04:58:15.4618908Z    |
2019-07-27T04:58:15.4618984Z LL |                    let y = &mut x;
2019-07-27T04:58:15.4619267Z    |                            ------ first mutable borrow occurs here
2019-07-27T04:58:15.4619377Z LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-07-27T04:58:15.4619475Z    |                    ^^^^^^ second mutable borrow occurs here
2019-07-27T04:58:15.4619561Z LL |                    *y = 1;
2019-07-27T04:58:15.4619836Z    |                    ------ first borrow later used here
2019-07-27T04:58:15.4619906Z 
2019-07-27T04:58:15.4619972Z error: captured variable cannot escape `FnMut` closure body
2019-07-27T04:58:15.4620353Z    |
2019-07-27T04:58:15.4620435Z LL |              || {
2019-07-27T04:58:15.4620435Z LL |              || {
2019-07-27T04:58:15.4621085Z    |               - inferred to be a `FnMut` closure
2019-07-27T04:58:15.4621296Z LL | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
2019-07-27T04:58:15.4621377Z LL | |                    let y = &mut x;
2019-07-27T04:58:15.4623780Z LL | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
2019-07-27T04:58:15.4624056Z LL | |                    *y = 1;
2019-07-27T04:58:15.4624145Z LL | |                    drop(y);
2019-07-27T04:58:15.4624210Z LL | |                 }
2019-07-27T04:58:15.4624497Z    | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
2019-07-27T04:58:15.4624718Z    |
2019-07-27T04:58:15.4624809Z    = note: `FnMut` closures only have access to their captured variables while they are executing...
2019-07-27T04:58:15.4624906Z    = note: ...therefore, they cannot allow references to captured variables to escape
2019-07-27T04:58:15.4624975Z 
2019-07-27T04:58:15.4625036Z error[E0503]: cannot use `f.x` because it was mutably borrowed
2019-07-27T04:58:15.4625524Z    |
2019-07-27T04:58:15.4625524Z    |
2019-07-27T04:58:15.4625597Z LL |         let x = f.x();
2019-07-27T04:58:15.4627254Z    |                 - borrow of `f` occurs here
2019-07-27T04:58:15.4627394Z LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
2019-07-27T04:58:15.4627777Z    |         ^^^ use of borrowed `f`
2019-07-27T04:58:15.4627879Z LL |         drop(x);
2019-07-27T04:58:15.4628247Z    |              - borrow later used here
2019-07-27T04:58:15.4628314Z 
2019-07-27T04:58:15.4628382Z error[E0503]: cannot use `g.0` because it was mutably borrowed
2019-07-27T04:58:15.4628893Z    |
2019-07-27T04:58:15.4628966Z LL |         let x = g.x();
2019-07-27T04:58:15.4628966Z LL |         let x = g.x();
2019-07-27T04:58:15.4629242Z    |                 - borrow of `g` occurs here
2019-07-27T04:58:15.4629344Z LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
2019-07-27T04:58:15.4629552Z    |         ^^^ use of borrowed `g`
2019-07-27T04:58:15.4629633Z LL |         drop(x);
2019-07-27T04:58:15.4630114Z    |              - borrow later used here
2019-07-27T04:58:15.4630176Z 
2019-07-27T04:58:15.4630241Z error[E0503]: cannot use `h.0` because it was mutably borrowed
2019-07-27T04:58:15.4630621Z    |
2019-07-27T04:58:15.4631310Z LL |         let x = &mut h.0;
2019-07-27T04:58:15.4631310Z LL |         let x = &mut h.0;
2019-07-27T04:58:15.4631673Z    |                 -------- borrow of `h.0` occurs here
2019-07-27T04:58:15.4631777Z LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
2019-07-27T04:58:15.4631853Z    |         ^^^ use of borrowed `h.0`
2019-07-27T04:58:15.4631934Z LL |         drop(x);
2019-07-27T04:58:15.4632318Z    |              - borrow later used here
2019-07-27T04:58:15.4632395Z 
2019-07-27T04:58:15.4632463Z error[E0503]: cannot use `e.0` because it was mutably borrowed
2019-07-27T04:58:15.4632884Z    |
2019-07-27T04:58:15.4632884Z    |
2019-07-27T04:58:15.4632957Z LL |         let x = e.x();
2019-07-27T04:58:15.4633220Z    |                 - borrow of `e` occurs here
2019-07-27T04:58:15.4633319Z LL |         match e {
2019-07-27T04:58:15.4633403Z LL |             Baz::X(value) => value //~ ERROR cannot use `e.0` because it was mutably borrowed
2019-07-27T04:58:15.4633506Z    |                    ^^^^^ use of borrowed `e`
2019-07-27T04:58:15.4633574Z LL |         };
2019-07-27T04:58:15.4633651Z LL |         drop(x);
2019-07-27T04:58:15.4633907Z    |              - borrow later used here
2019-07-27T04:58:15.4633955Z 
2019-07-27T04:58:15.4634037Z error[E0503]: cannot use `u.a` because it was mutably borrowed
2019-07-27T04:58:15.4634544Z    |
2019-07-27T04:58:15.4634544Z    |
2019-07-27T04:58:15.4634600Z LL |         let x = &mut u.a;
2019-07-27T04:58:15.4634877Z    |                 -------- borrow of `u.a` occurs here
2019-07-27T04:58:15.4634975Z LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
2019-07-27T04:58:15.4635050Z    |         ^^^ use of borrowed `u.a`
2019-07-27T04:58:15.4635217Z LL |         drop(x);
2019-07-27T04:58:15.4635788Z    |              - borrow later used here
2019-07-27T04:58:15.4635839Z 
2019-07-27T04:58:15.4635924Z error[E0503]: cannot use `f.x` because it was mutably borrowed
2019-07-27T04:58:15.4636352Z    |
2019-07-27T04:58:15.4636352Z    |
2019-07-27T04:58:15.4636409Z LL |         let x = f.x();
2019-07-27T04:58:15.4636677Z    |                 - borrow of `*f` occurs here
2019-07-27T04:58:15.4636768Z LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
2019-07-27T04:58:15.4636858Z    |         ^^^ use of borrowed `*f`
2019-07-27T04:58:15.4636922Z LL |         drop(x);
2019-07-27T04:58:15.4637185Z    |              - borrow later used here
2019-07-27T04:58:15.4637232Z 
2019-07-27T04:58:15.4637311Z error[E0503]: cannot use `g.0` because it was mutably borrowed
2019-07-27T04:58:15.4638900Z    |
2019-07-27T04:58:15.4638986Z LL |         let x = g.x();
2019-07-27T04:58:15.4638986Z LL |         let x = g.x();
2019-07-27T04:58:15.4639371Z    |                 - borrow of `*g` occurs here
2019-07-27T04:58:15.4639453Z LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
2019-07-27T04:58:15.4639543Z    |         ^^^ use of borrowed `*g`
2019-07-27T04:58:15.4639606Z LL |         drop(x);
2019-07-27T04:58:15.4639923Z    |              - borrow later used here
2019-07-27T04:58:15.4639972Z 
2019-07-27T04:58:15.4640051Z error[E0503]: cannot use `h.0` because it was mutably borrowed
2019-07-27T04:58:15.4640438Z    |
2019-07-27T04:58:15.4640496Z LL |         let x = &mut h.0;
2019-07-27T04:58:15.4640496Z LL |         let x = &mut h.0;
2019-07-27T04:58:15.4641302Z    |                 -------- borrow of `h.0` occurs here
2019-07-27T04:58:15.4641399Z LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
2019-07-27T04:58:15.4641488Z    |         ^^^ use of borrowed `h.0`
2019-07-27T04:58:15.4641565Z LL |         drop(x);
2019-07-27T04:58:15.4641866Z    |              - borrow later used here
2019-07-27T04:58:15.4641915Z 
2019-07-27T04:58:15.4641996Z error[E0503]: cannot use `e.0` because it was mutably borrowed
2019-07-27T04:58:15.4642386Z    |
2019-07-27T04:58:15.4642386Z    |
2019-07-27T04:58:15.4642446Z LL |         let x = e.x();
2019-07-27T04:58:15.4642723Z    |                 - borrow of `*e` occurs here
2019-07-27T04:58:15.4642796Z LL |         match *e {
2019-07-27T04:58:15.4643006Z LL |             Baz::X(value) => value
2019-07-27T04:58:15.4643088Z    |                    ^^^^^ use of borrowed `*e`
2019-07-27T04:58:15.4643170Z ...
2019-07-27T04:58:15.4643232Z LL |         drop(x);
2019-07-27T04:58:15.4643532Z    |              - borrow later used here
2019-07-27T04:58:15.4643581Z 
2019-07-27T04:58:15.4643648Z error[E0503]: cannot use `u.a` because it was mutably borrowed
2019-07-27T04:58:15.4644041Z    |
2019-07-27T04:58:15.4644041Z    |
2019-07-27T04:58:15.4644118Z LL |         let x = &mut u.a;
2019-07-27T04:58:15.4644389Z    |                 -------- borrow of `u.a` occurs here
2019-07-27T04:58:15.4644491Z LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
2019-07-27T04:58:15.4644582Z    |         ^^^ use of borrowed `u.a`
2019-07-27T04:58:15.4644646Z LL |         drop(x);
2019-07-27T04:58:15.4644914Z    |              - borrow later used here
2019-07-27T04:58:15.4644961Z 
2019-07-27T04:58:15.4645036Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4645422Z    |
2019-07-27T04:58:15.4645498Z LL |         let x = &mut v;
2019-07-27T04:58:15.4645498Z LL |         let x = &mut v;
2019-07-27T04:58:15.4645762Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4645850Z LL |         match v {
2019-07-27T04:58:15.4646013Z LL |             &[x, _, .., _, _] => println!("{}", x),
2019-07-27T04:58:15.4646100Z    |               ^ use of borrowed `v`
2019-07-27T04:58:15.4646164Z ...
2019-07-27T04:58:15.4646236Z LL |         drop(x);
2019-07-27T04:58:15.4646510Z    |              - borrow later used here
2019-07-27T04:58:15.4646574Z 
2019-07-27T04:58:15.4646641Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4647024Z    |
2019-07-27T04:58:15.4647107Z LL |         let x = &mut v;
2019-07-27T04:58:15.4647107Z LL |         let x = &mut v;
2019-07-27T04:58:15.4647377Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4647464Z ...
2019-07-27T04:58:15.4647528Z LL |             &[_, x, .., _, _] => println!("{}", x),
2019-07-27T04:58:15.4647616Z    |                  ^ use of borrowed `v`
2019-07-27T04:58:15.4647680Z ...
2019-07-27T04:58:15.4647751Z LL |         drop(x);
2019-07-27T04:58:15.4648003Z    |              - borrow later used here
2019-07-27T04:58:15.4648075Z 
2019-07-27T04:58:15.4648142Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4648530Z    |
2019-07-27T04:58:15.4648604Z LL |         let x = &mut v;
2019-07-27T04:58:15.4648604Z LL |         let x = &mut v;
2019-07-27T04:58:15.4648868Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4648955Z ...
2019-07-27T04:58:15.4649020Z LL |             &[_, _, .., x, _] => println!("{}", x),
2019-07-27T04:58:15.4649119Z    |                         ^ use of borrowed `v`
2019-07-27T04:58:15.4649186Z ...
2019-07-27T04:58:15.4649257Z LL |         drop(x);
2019-07-27T04:58:15.4649509Z    |              - borrow later used here
2019-07-27T04:58:15.4649556Z 
2019-07-27T04:58:15.4649639Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4650035Z    |
2019-07-27T04:58:15.4650108Z LL |         let x = &mut v;
2019-07-27T04:58:15.4650108Z LL |         let x = &mut v;
2019-07-27T04:58:15.4650377Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4650462Z ...
2019-07-27T04:58:15.4650527Z LL |             &[_, _, .., _, x] => println!("{}", x),
2019-07-27T04:58:15.4650617Z    |                            ^ use of borrowed `v`
2019-07-27T04:58:15.4651158Z ...
2019-07-27T04:58:15.4651250Z LL |         drop(x);
2019-07-27T04:58:15.4651563Z    |              - borrow later used here
2019-07-27T04:58:15.4651611Z 
2019-07-27T04:58:15.4651806Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4652232Z    |
2019-07-27T04:58:15.4652291Z LL |         let x = &mut v;
2019-07-27T04:58:15.4652291Z LL |         let x = &mut v;
2019-07-27T04:58:15.4652568Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4652640Z LL |         match v {
2019-07-27T04:58:15.4652725Z LL |             &[x @ ..] => println!("{:?}", x),
2019-07-27T04:58:15.4652807Z    |               ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4652889Z ...
2019-07-27T04:58:15.4652946Z LL |         drop(x);
2019-07-27T04:58:15.4653217Z    |              - borrow later used here
2019-07-27T04:58:15.4653264Z 
2019-07-27T04:58:15.4653348Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4653732Z    |
2019-07-27T04:58:15.4653792Z LL |         let x = &mut v;
2019-07-27T04:58:15.4653792Z LL |         let x = &mut v;
2019-07-27T04:58:15.4654080Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4654151Z ...
2019-07-27T04:58:15.4654230Z LL |             &[_, x @ ..] => println!("{:?}", x),
2019-07-27T04:58:15.4654303Z    |                  ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4654383Z ...
2019-07-27T04:58:15.4654440Z LL |         drop(x);
2019-07-27T04:58:15.4654707Z    |              - borrow later used here
2019-07-27T04:58:15.4654755Z 
2019-07-27T04:58:15.4654925Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4655335Z    |
2019-07-27T04:58:15.4655395Z LL |         let x = &mut v;
2019-07-27T04:58:15.4655395Z LL |         let x = &mut v;
2019-07-27T04:58:15.4655674Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4655744Z ...
2019-07-27T04:58:15.4655823Z LL |             &[x @ .., _] => println!("{:?}", x),
2019-07-27T04:58:15.4655897Z    |               ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4655984Z ...
2019-07-27T04:58:15.4656041Z LL |         drop(x);
2019-07-27T04:58:15.4656309Z    |              - borrow later used here
2019-07-27T04:58:15.4656358Z 
2019-07-27T04:58:15.4656425Z error[E0503]: cannot use `v[..]` because it was mutably borrowed
2019-07-27T04:58:15.4656824Z    |
2019-07-27T04:58:15.4656883Z LL |         let x = &mut v;
2019-07-27T04:58:15.4656883Z LL |         let x = &mut v;
2019-07-27T04:58:15.4657174Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4657246Z ...
2019-07-27T04:58:15.4657325Z LL |             &[_, x @ .., _] => println!("{:?}", x),
2019-07-27T04:58:15.4657401Z    |                  ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4657479Z ...
2019-07-27T04:58:15.4657536Z LL |         drop(x);
2019-07-27T04:58:15.4657802Z    |              - borrow later used here
2019-07-27T04:58:15.4657850Z 
2019-07-27T04:58:15.4657916Z error[E0503]: cannot use `e` because it was mutably borrowed
2019-07-27T04:58:15.4658307Z    |
2019-07-27T04:58:15.4658382Z LL |         let x = &mut e;
2019-07-27T04:58:15.4658648Z    |                 ------ borrow of `e` occurs here
2019-07-27T04:58:15.4658736Z LL |         match e {
2019-07-27T04:58:15.4658736Z LL |         match e {
2019-07-27T04:58:15.4658798Z LL |             E::A(ref ax) =>
2019-07-27T04:58:15.4658883Z    |             ^^^^^^^^^^^^ use of borrowed `e`
2019-07-27T04:58:15.4658956Z ...
2019-07-27T04:58:15.4659028Z LL |         drop(x);
2019-07-27T04:58:15.4659386Z    |              - borrow later used here
2019-07-27T04:58:15.4659446Z 
2019-07-27T04:58:15.4659516Z error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4660008Z    |
2019-07-27T04:58:15.4660078Z LL |         let x = &mut e;
2019-07-27T04:58:15.4660402Z    |                 ------ mutable borrow occurs here
2019-07-27T04:58:15.4660497Z LL |         match e {
2019-07-27T04:58:15.4660497Z LL |         match e {
2019-07-27T04:58:15.4660556Z LL |             E::A(ref ax) =>
2019-07-27T04:58:15.4661234Z    |                  ^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4661320Z ...
2019-07-27T04:58:15.4661393Z LL |         drop(x);
2019-07-27T04:58:15.4661735Z    |              - mutable borrow later used here
2019-07-27T04:58:15.4661801Z 
2019-07-27T04:58:15.4661873Z error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4662281Z    |
2019-07-27T04:58:15.4662355Z LL |         let x = &mut e;
2019-07-27T04:58:15.4662620Z    |                 ------ mutable borrow occurs here
2019-07-27T04:58:15.4662705Z ...
2019-07-27T04:58:15.4662705Z ...
2019-07-27T04:58:15.4662764Z LL |             E::B { x: ref bx } =>
2019-07-27T04:58:15.4662853Z    |                       ^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4662922Z ...
2019-07-27T04:58:15.4663001Z LL |         drop(x);
2019-07-27T04:58:15.4663264Z    |              - mutable borrow later used here
2019-07-27T04:58:15.4663329Z 
2019-07-27T04:58:15.4663402Z error[E0502]: cannot borrow `s.y.0` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4663793Z    |
2019-07-27T04:58:15.4663867Z LL |         let x = &mut s;
2019-07-27T04:58:15.4664270Z    |                 ------ mutable borrow occurs here
2019-07-27T04:58:15.4664361Z LL |         match s {
2019-07-27T04:58:15.4664361Z LL |         match s {
2019-07-27T04:58:15.4664428Z LL |             S  { y: (ref y0, _), .. } =>
2019-07-27T04:58:15.4664517Z    |                      ^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4664587Z ...
2019-07-27T04:58:15.4664658Z LL |         drop(x);
2019-07-27T04:58:15.4664919Z    |              - mutable borrow later used here
2019-07-27T04:58:15.4664967Z 
2019-07-27T04:58:15.4665065Z error[E0502]: cannot borrow `s.x.y` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4665462Z    |
2019-07-27T04:58:15.4665521Z LL |         let x = &mut s;
2019-07-27T04:58:15.4665804Z    |                 ------ mutable borrow occurs here
2019-07-27T04:58:15.4665890Z ...
2019-07-27T04:58:15.4665890Z ...
2019-07-27T04:58:15.4665954Z LL |             S  { x: F { y: ref x0, .. }, .. } =>
2019-07-27T04:58:15.4666054Z    |                            ^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4666124Z ...
2019-07-27T04:58:15.4666195Z LL |         drop(x);
2019-07-27T04:58:15.4666460Z    |              - mutable borrow later used here
2019-07-27T04:58:15.4666509Z 
2019-07-27T04:58:15.4666592Z error[E0503]: cannot use `*v` because it was mutably borrowed
2019-07-27T04:58:15.4666973Z    |
2019-07-27T04:58:15.4667032Z LL |         let x = &mut v;
2019-07-27T04:58:15.4667032Z LL |         let x = &mut v;
2019-07-27T04:58:15.4667319Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4667393Z LL |         v[0].y;
2019-07-27T04:58:15.4667471Z    |         ^^^^ use of borrowed `v`
2019-07-27T04:58:15.4667536Z ...
2019-07-27T04:58:15.4667606Z LL |         drop(x);
2019-07-27T04:58:15.4667859Z    |              - borrow later used here
2019-07-27T04:58:15.4667922Z 
2019-07-27T04:58:15.4667990Z error[E0503]: cannot use `v[_].y` because it was mutably borrowed
2019-07-27T04:58:15.4668385Z    |
2019-07-27T04:58:15.4668457Z LL |         let x = &mut v;
2019-07-27T04:58:15.4668457Z LL |         let x = &mut v;
2019-07-27T04:58:15.4668721Z    |                 ------ borrow of `v` occurs here
2019-07-27T04:58:15.4668809Z LL |         v[0].y;
2019-07-27T04:58:15.4668872Z    |         ^^^^^^ use of borrowed `v`
2019-07-27T04:58:15.4668952Z ...
2019-07-27T04:58:15.4669009Z LL |         drop(x);
2019-07-27T04:58:15.4669592Z    |              - borrow later used here
2019-07-27T04:58:15.4669646Z 
2019-07-27T04:58:15.4669732Z error[E0502]: cannot borrow `v[..].x` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4670126Z    |
2019-07-27T04:58:15.4670180Z LL |         let x = &mut v;
2019-07-27T04:58:15.4670442Z    |                 ------ mutable borrow occurs here
2019-07-27T04:58:15.4670520Z LL |         match v {
2019-07-27T04:58:15.4670520Z LL |         match v {
2019-07-27T04:58:15.4670599Z LL |             &[_, F {x: ref xf, ..}] => println!("{}", xf),
2019-07-27T04:58:15.4671142Z    |                        ^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4671233Z ...
2019-07-27T04:58:15.4671291Z LL |         drop(x);
2019-07-27T04:58:15.4671624Z    |              - mutable borrow later used here
2019-07-27T04:58:15.4671674Z 
2019-07-27T04:58:15.4671750Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4672178Z    |
2019-07-27T04:58:15.4672237Z LL |             let x = &mut block;
2019-07-27T04:58:15.4672533Z    |                     ---------- mutable borrow occurs here
2019-07-27T04:58:15.4672533Z    |                     ---------- mutable borrow occurs here
2019-07-27T04:58:15.4672806Z LL |             let p: &'a u8 = &*block.current;
2019-07-27T04:58:15.4672905Z    |                             ^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4673090Z ...
2019-07-27T04:58:15.4673163Z LL |             drop(x);
2019-07-27T04:58:15.4673456Z    |                  - mutable borrow later used here
2019-07-27T04:58:15.4673507Z 
2019-07-27T04:58:15.4673597Z error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
2019-07-27T04:58:15.4673993Z    |
2019-07-27T04:58:15.4674052Z LL |             let x = &mut block;
2019-07-27T04:58:15.4674476Z    |                     ---------- mutable borrow occurs here
2019-07-27T04:58:15.4674476Z    |                     ---------- mutable borrow occurs here
2019-07-27T04:58:15.4674567Z LL |             let p : *const u8 = &*(*block).current;
2019-07-27T04:58:15.4674645Z    |                                 ^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
2019-07-27T04:58:15.4674728Z ...
2019-07-27T04:58:15.4674782Z LL |             drop(x);
2019-07-27T04:58:15.4675046Z    |                  - mutable borrow later used here
2019-07-27T04:58:15.4675100Z 
2019-07-27T04:58:15.4675155Z error[E0382]: use of moved value: `x`
2019-07-27T04:58:15.4675718Z    |
2019-07-27T04:58:15.4675718Z    |
2019-07-27T04:58:15.4675791Z LL |                 drop(x);
2019-07-27T04:58:15.4676081Z    |                      - value moved here
2019-07-27T04:58:15.4676173Z LL |                 drop(x); //~ ERROR use of moved value: `x`
2019-07-27T04:58:15.4676246Z    |                      ^ value used here after move
2019-07-27T04:58:15.4676324Z    |
2019-07-27T04:58:15.4676410Z    = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
2019-07-27T04:58:15.4676540Z error: aborting due to 32 previous errors
2019-07-27T04:58:15.4676593Z 
2019-07-27T04:58:15.4676658Z Some errors have detailed explanations: E0382, E0499, E0502, E0503.
2019-07-27T04:58:15.4676953Z For more information about an error, try `rustc --explain E0382`.
---
2019-07-27T04:58:15.4678538Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T04:58:15.4678657Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T04:58:15.4678707Z 
2019-07-27T04:58:15.4678739Z 
2019-07-27T04:58:15.4680513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-07-27T04:58:15.4681937Z 
2019-07-27T04:58:15.4681975Z 
2019-07-27T04:58:15.4682042Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T04:58:15.4682132Z Build completed unsuccessfully in 1:36:57
2019-07-27T04:58:15.4682132Z Build completed unsuccessfully in 1:36:57
2019-07-27T04:58:16.6081824Z ##[error]Bash exited with code '1'.
2019-07-27T04:58:16.6118001Z ##[section]Starting: Upload CPU usage statistics
2019-07-27T04:58:16.6127359Z ==============================================================================
2019-07-27T04:58:16.6127474Z Task         : Bash
2019-07-27T04:58:16.6127545Z Description  : Run a Bash script on macOS, Linux, or Windows
