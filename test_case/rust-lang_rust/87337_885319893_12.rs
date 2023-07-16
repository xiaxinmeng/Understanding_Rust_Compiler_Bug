
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)

error: unknown attribute `no_Run`. Did you mean `no_run`?
   |
   |
LL | / /// foobar
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 