
rustc 1.16.0 (30cf806ef 2017-03-10)
error: lifetime of `a` is too short to guarantee its contents can be safely reborrowed
 --> <anon>:5:3
  |
5 | 		|| {
  | 		^^
  |
note: `a` would have to be valid for the scope of call-site for function at 4:4...
 --> <anon>:4:5
  |
4 |   	|| {
  |  _____^ starting here...
5 | | 		|| {
6 | | 			a.push(true)
7 | | 		}
8 | | 	};
  | |__^ ...ending here
note: ...but `a` is only valid for the lifetime  as defined on the body at 4:4
 --> <anon>:4:5
  |
4 |   	|| {
  |  _____^ starting here...
5 | | 		|| {
6 | | 			a.push(true)
7 | | 		}
8 | | 	};
  | |__^ ...ending here

error: aborting due to previous error
