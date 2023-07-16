
error: attempted access of field `fied` on type `Ty`, but no field with that name was found
 --> <anon>:3:19
  |
3 | fn test1(t: Ty) { t.fied }
  |                   ^^^^^^
  |
help: did you mean `field`?
 --> <anon>:3:21
  |
3 | fn test1(t: Ty) { t.fied }
  |                     ^^^^

error: attempted access of field `fied` on type `&mut Ty`, but no field with that name was found
 --> <anon>:4:24
  |
4 | fn test2(t: &mut Ty) { t.fied }
  |                        ^^^^^^
