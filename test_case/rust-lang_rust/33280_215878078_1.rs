
<std macros>:5:8: 6:42 error: mismatched types:
 expected `()`,
    found `core::result::Result<_, _>`
(expected (),
    found enum `core::result::Result`) [E0308]
<std macros>:5 return $ crate:: result:: Result:: Err (
<std macros>:6 $ crate:: convert:: From:: from ( err ) ) } } )
<anon>:2:5: 2:18 note: in this expansion of try! (defined in <std macros>)
<std macros>:5:8: 6:42 help: see the detailed explanation for E0308
error: aborting due to previous error
playpen: application terminated with error code 101
