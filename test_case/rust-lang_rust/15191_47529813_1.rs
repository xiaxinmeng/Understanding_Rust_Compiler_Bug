
<anon>:39:18: 39:19 error: `d` does not live long enough
<anon>:39         let _d = d.trans();
                           ^
<anon>:35:11: 41:2 note: reference must be valid for the block at 35:10...
<anon>:35 fn main() {
<anon>:36     let c = Context { view: None };
<anon>:37     {
<anon>:38         let d = c.trans();
<anon>:39         let _d = d.trans();
<anon>:40     }
          ...
<anon>:37:5: 40:6 note: ...but borrowed value is only valid for the block at 37:4
<anon>:37     {
<anon>:38         let d = c.trans();
<anon>:39         let _d = d.trans();
<anon>:40     }
