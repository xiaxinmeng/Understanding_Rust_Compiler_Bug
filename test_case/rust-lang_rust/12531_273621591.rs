
error[E0425]: cannot find function `foo` in module `submod`
 --> <anon>:8:3
  |
8 |   submod::foo();
  |   ^^^^^^^^^^^ not found in `submod`
  |
  = help: possible candidate is found in another module, you can import it into scope:
  = help:   `use submod::submod::foo;`
