
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or the code will be wrapped inside a main function

error: unknown attribute `teSt_harness`. Did you mean `test_harness`?
   |
   |
LL | / /// b
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 