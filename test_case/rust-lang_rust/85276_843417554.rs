diff
diff --git a/src/test/assembly/static-relocation-model.rs b/src/test/assembly/static-relocation-model.rs
index bd456817199..ce2b3b1cfa4 100644
--- a/src/test/assembly/static-relocation-model.rs
+++ b/src/test/assembly/static-relocation-model.rs
@@ -34,7 +34,7 @@ impl Sync for u8 {}
 }
 
 // CHECK-LABEL: banana:
-// x64: movb   chaenomeles(%{{[a-z0-9]+}}), %{{[a-z]+}}
+// x64: movb   chaenomeles{{(\(%[a-z0-9]+\))?}}, %{{[a-z0-9]+}}
 // A64:      adrp    [[REG:[a-z0-9]+]], chaenomeles
 // A64-NEXT: ldrb    {{[a-z0-9]+}}, {{\[}}[[REG]], :lo12:chaenomeles]
 #[no_mangle]
@@ -45,7 +45,7 @@ pub fn banana() -> u8 {
 }
 
 // CHECK-LABEL: peach:
-// x64: movb    banana(%{{[a-z0-9]+}}), %{{[a-z]+}}
+// x64: movb    banana{{(\(%[a-z0-9]+\))?}}, %{{[a-z0-9]+}}
 // A64:      adrp    [[REG2:[a-z0-9]+]], banana
 // A64-NEXT: ldrb    {{[a-z0-9]+}}, {{\[}}[[REG2]], :lo12:banana]
 #[no_mangle]
@@ -56,8 +56,8 @@ pub fn peach() -> u8 {
 }
 
 // CHECK-LABEL: mango:
-// x64:      movq    EXOCHORDA(%{{[a-z0-9]+}}), %{{[a-z]+}}
-// x64-NEXT: movb (%rax), %al
+// x64:      movq    EXOCHORDA{{(\(%[a-z0-9]+\))?}}, %[[REG:[a-z0-9]+]]
+// x64-NEXT: movb    (%[[REG]]), %{{[a-z0-9]+}}
 // A64:      adrp    [[REG2:[a-z0-9]+]], EXOCHORDA
 // A64-NEXT: ldr     {{[a-z0-9]+}}, {{\[}}[[REG2]], :lo12:EXOCHORDA]
 #[no_mangle]
@@ -68,7 +68,7 @@ pub fn mango() -> u8 {
 }
 
 // CHECK-LABEL: orange:
-// x64:  movl    $PIERIS, %{{[a-z]+}}
+// x64: mov{{l|absq}}    $PIERIS, %{{[a-z0-9]+}}
 // A64:      adrp    [[REG2:[a-z0-9]+]], PIERIS
 // A64-NEXT: add     {{[a-z0-9]+}}, [[REG2]], :lo12:PIERIS
 #[no_mangle]
