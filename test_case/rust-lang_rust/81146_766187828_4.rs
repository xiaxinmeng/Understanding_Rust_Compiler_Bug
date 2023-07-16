ll
define [4 x i8] @sum([4 x i8] %0, [4 x i8] %1) unnamed_addr #0 {
start:
  %.fca.0.extract = extractvalue [4 x i8] %0, 0
  %.fca.1.extract = extractvalue [4 x i8] %0, 1
  %.fca.2.extract = extractvalue [4 x i8] %0, 2
  %.fca.3.extract = extractvalue [4 x i8] %0, 3
  %.fca.0.extract1 = extractvalue [4 x i8] %1, 0
  %.fca.1.extract2 = extractvalue [4 x i8] %1, 1
  %.fca.2.extract3 = extractvalue [4 x i8] %1, 2
  %.fca.3.extract4 = extractvalue [4 x i8] %1, 3
  %_3 = add i8 %.fca.0.extract1, %.fca.0.extract
  %_8 = add i8 %.fca.1.extract2, %.fca.1.extract
  %_13 = add i8 %.fca.2.extract3, %.fca.2.extract
  %_18 = add i8 %.fca.3.extract4, %.fca.3.extract
  %.fca.0.insert = insertvalue [4 x i8] undef, i8 %_3, 0
  %.fca.1.insert = insertvalue [4 x i8] %.fca.0.insert, i8 %_8, 1
  %.fca.2.insert = insertvalue [4 x i8] %.fca.1.insert, i8 %_13, 2
  %.fca.3.insert = insertvalue [4 x i8] %.fca.2.insert, i8 %_18, 3
  ret [4 x i8] %.fca.3.insert
}

attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
