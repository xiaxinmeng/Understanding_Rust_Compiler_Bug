
<anon>:5:32: 5:35 error: type mismatch: the type `fn(&std::fs::DirEntry) -> std::path::PathBuf {std::fs::DirEntry::path}` implements the trait `for<'r> std::ops::FnMut<(&'r std::fs::DirEntry,)>`, but the trait `std::ops::FnMut<(std::result::Result<std::fs::DirEntry, std::io::Error>,)>` is required (expected enum `std::result::Result`, found &-ptr) [E0281]
<anon>:5     fs::read_dir("/").unwrap().map(DirEntry::path).filter(|_| true);
                                        ^~~
<anon>:5:32: 5:35 help: see the detailed explanation for E0281
<anon>:5:32: 5:35 error: type mismatch: the type `fn(&std::fs::DirEntry) -> std::path::PathBuf {std::fs::DirEntry::path}` implements the trait `for<'r> std::ops::FnOnce<(&'r std::fs::DirEntry,)>`, but the trait `std::ops::FnOnce<(std::result::Result<std::fs::DirEntry, std::io::Error>,)>` is required (expected enum `std::result::Result`, found &-ptr) [E0281]
<anon>:5     fs::read_dir("/").unwrap().map(DirEntry::path).filter(|_| true);
                                        ^~~
<anon>:5:32: 5:35 help: see the detailed explanation for E0281
<anon>:5:52: 5:58 error: no method named `filter` found for type `std::iter::Map<std::fs::ReadDir, fn(&std::fs::DirEntry) -> std::path::PathBuf {std::fs::DirEntry::path}>` in the current scope
<anon>:5     fs::read_dir("/").unwrap().map(DirEntry::path).filter(|_| true);
                                                            ^~~~~~
<anon>:5:52: 5:58 note: the method `filter` exists but the following trait bounds were not satisfied: `fn(&std::fs::DirEntry) -> std::path::PathBuf {std::fs::DirEntry::path} : std::ops::FnMut<(std::result::Result<std::fs::DirEntry, std::io::Error>,)>`, `std::iter::Map<std::fs::ReadDir, fn(&std::fs::DirEntry) -> std::path::PathBuf {std::fs::DirEntry::path}> : std::iter::Iterator`
