rust
2 |     'a: loop { break };
3 |     loop { break 'a };
  |                  ^^ undeclared label `'a`
