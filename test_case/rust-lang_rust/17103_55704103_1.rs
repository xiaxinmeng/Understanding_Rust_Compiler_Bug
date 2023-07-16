
<log macros>:3:21: 3:24 error: failed to resolve. Maybe a missing `extern crate log`?
<log macros>:3         static LOC: ::log::LogLocation = ::log::LogLocation {
                                   ^~~
<log macros>:1:1: 13:2 note: in expansion of log!
<log macros>:2:46: 2:76 note: expansion site
<log macros>:1:1: 3:2 note: in expansion of debug!
<anon>:8:9: 9:6 note: expansion site
<log macros>:3:21: 3:39 error: use of undeclared type name `log::LogLocation`
<log macros>:3         static LOC: ::log::LogLocation = ::log::LogLocation {
                                   ^~~~~~~~~~~~~~~~~~
[...]
