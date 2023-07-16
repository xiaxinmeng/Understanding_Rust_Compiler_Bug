
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully

error: unknown attribute `comPile_fail`. Did you mean `compile_fail`?
   |
   |
LL | / /// foo
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 