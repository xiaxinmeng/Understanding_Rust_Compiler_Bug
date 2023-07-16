 cmd
<anon>:4:5: 4:55 error: method `next` has an incompatible type for trait: expected enum `core::option::Option`,
                                                                             found enum `core::result::Result` [E0053]
<anon>:4     fn next(&mut self) -> Result<uint, uint> { Ok(7) }
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
playpen: application terminated with error code 101
