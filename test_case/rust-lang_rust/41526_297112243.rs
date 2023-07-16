
[01:05:37] ---- net/tcp.rs - net::tcp::TcpListener (line 75) stdout ----
[01:05:37] 	error[E0434]: can't capture dynamic environment in a fn item; use the || { ... } closure form instead
[01:05:37]   --> <anon>:15:15
[01:05:37]    |
[01:05:37] 15 | for stream in listener.incoming() {
[01:05:37]    |               ^^^^^^^^
[01:05:37] 
[01:05:37] error: aborting due to previous error(s)
