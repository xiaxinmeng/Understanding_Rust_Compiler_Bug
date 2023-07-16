
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:134:28
[01:04:49]     |
[01:04:49] 134 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                            ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:134:43
[01:04:49]     |
[01:04:49] 134 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                                           ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:151:28
[01:04:49]     |
[01:04:49] 151 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                            ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:151:43
[01:04:49]     |
[01:04:49] 151 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                                           ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/udp.rs:183:28
[01:04:49]     |
[01:04:49] 183 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                            ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/udp.rs:183:43
[01:04:49]     |
[01:04:49] 183 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                                           ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/udp.rs:200:28
[01:04:49]     |
[01:04:49] 200 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                            ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0433]: failed to resolve. Use of undeclared type or module `io`
[01:04:49]    --> libstd/sys/redox/net/udp.rs:200:43
[01:04:49]     |
[01:04:49] 200 |                 return Err(io::Error::new(io::ErrorKind::InvalidInput,
[01:04:49]     |                                           ^^ Use of undeclared type or module `io`
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:133:16
[01:04:49]     |
[01:04:49] 133 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:133:38
[01:04:49]     |
[01:04:49] 133 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                                      ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:150:16
[01:04:49]     |
[01:04:49] 150 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/tcp.rs:150:38
[01:04:49]     |
[01:04:49] 150 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                                      ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/udp.rs:182:16
[01:04:49]     |
[01:04:49] 182 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/udp.rs:182:38
[01:04:49]     |
[01:04:49] 182 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                                      ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/udp.rs:199:16
[01:04:49]     |
[01:04:49] 199 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                ^^^ not found in this scope
[01:04:49] 
[01:04:49] error[E0425]: cannot find value `dur` in this scope
[01:04:49]    --> libstd/sys/redox/net/udp.rs:199:38
[01:04:49]     |
[01:04:49] 199 |             if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
[01:04:49]     |                                      ^^^ not found in this scope
[01:04:49] 
[01:04:51] error: aborting due to 16 previous errors
[01:04:51] 
[01:04:51] error: Could not compile `std`.
