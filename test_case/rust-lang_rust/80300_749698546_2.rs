
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or won't fail if it doesn't panic when running

error: unknown attribute `shouldpanic`. Did you mean `should_panic`?
   |
   |
LL | / /// bar
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 