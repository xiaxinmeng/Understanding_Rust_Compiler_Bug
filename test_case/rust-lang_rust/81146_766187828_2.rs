ll
define i32 @sum(i32 %0, i32 %1) unnamed_addr #0 {
start:
  %.sroa.4.0.extract.shift = and i32 %0, 65280
  %.sroa.5.0.extract.shift = and i32 %0, 16711680
  %.sroa.6.0.extract.shift = and i32 %0, -16777216
  %_3 = add i32 %1, %0
  %.sroa.64.0.extract.shift8 = add i32 %.sroa.6.0.extract.shift, %1
  %.sroa.46.0.insert.shift = and i32 %.sroa.64.0.extract.shift8, -16777216
  %.sroa.53.0.extract.shift10 = add i32 %.sroa.5.0.extract.shift, %1
  %.sroa.3.0.insert.shift = and i32 %.sroa.53.0.extract.shift10, 16711680
  %.sroa.3.0.insert.insert = or i32 %.sroa.46.0.insert.shift, %.sroa.3.0.insert.shift
  %.sroa.42.0.extract.shift12 = add i32 %.sroa.4.0.extract.shift, %1
  %.sroa.2.0.insert.shift = and i32 %.sroa.42.0.extract.shift12, 65280
  %.sroa.2.0.insert.insert = or i32 %.sroa.3.0.insert.insert, %.sroa.2.0.insert.shift
  %.sroa.05.0.insert.ext = and i32 %_3, 255
  %.sroa.05.0.insert.insert = or i32 %.sroa.2.0.insert.insert, %.sroa.05.0.insert.ext
  ret i32 %.sroa.05.0.insert.insert
}

attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
