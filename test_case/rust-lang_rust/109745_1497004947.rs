rust
> error: Undefined Behavior: accessing memory with alignment 1, but alignment 2 is required
>   --> src/main.rs:13:14
>    |
> 13 |     unsafe { ptr.read() };
>    |              ^^^^^^^^^^ accessing memory with alignment 1, but alignment 2 is required
> 