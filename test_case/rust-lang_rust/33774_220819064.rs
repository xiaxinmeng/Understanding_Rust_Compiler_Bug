
<std macros>:5:8: 6:45 error: mismatched types [E0308]
<std macros>:5 return $ crate :: result :: Result :: Err (
                      ^
src/main.rs:24:5: 24:25 note: in this expansion of try! (defined in <std macros>)
<std macros>:5:8: 6:45 help: run `rustc --explain E0308` to see a detailed explanation
<std macros>:5:8: 6:45 note: expected type `std::path::PathBuf`
<std macros>:5:8: 6:45 note:    found type `std::result::Result<_, _>`
