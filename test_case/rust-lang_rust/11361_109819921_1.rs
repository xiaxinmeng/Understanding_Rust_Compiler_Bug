
<anon>:10:25: 10:35 error: cannot infer an appropriate lifetime for lifetime parameter 'a in function call due to conflicting requirements
<anon>:10             *buf = &mut buf[len..];
                                  ^~~~~~~~~~
<anon>:9:9: 11:10 help: consider using an explicit lifetime parameter as shown: fn reslice<'a>(buf: &'a mut &'a mut [u8], len: usize)
<anon>:9         fn reslice(buf: &mut &mut [u8], len: usize) {
<anon>:10             *buf = &mut buf[len..];
<anon>:11         }
error: aborting due to previous error
