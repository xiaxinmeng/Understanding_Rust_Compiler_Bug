
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or won't fail if it doesn't panic when running

error: unknown attribute `no-run`. Did you mean `no_run`?
   |
   |
LL | / /// foobar
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 