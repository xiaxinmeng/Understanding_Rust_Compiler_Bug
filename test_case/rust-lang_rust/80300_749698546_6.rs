
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)

error: unknown attribute `test-harness`. Did you mean `test_harness`?
   |
   |
LL | / /// b
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 