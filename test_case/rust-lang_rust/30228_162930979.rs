
  %0 = $b
  %1 = $a
  drop $a uwto uw0
  *%1 = %0
  %ret = ()
  return
uw0:
  *%1 = %0
  unwind
