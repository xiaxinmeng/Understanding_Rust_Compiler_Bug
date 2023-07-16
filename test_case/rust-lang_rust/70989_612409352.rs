
2020-04-11T12:21:59.9749761Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
2020-04-11T12:21:59.9749920Z 23     }
2020-04-11T12:21:59.9750017Z 24 }
2020-04-11T12:21:59.9750122Z 25 
2020-04-11T12:21:59.9750468Z - // HACK(eddyb) this is for testing if PR CI catches a 32-bit mir-opt failure.
2020-04-11T12:21:59.9750745Z - 
2020-04-11T12:21:59.9750920Z 28 alloc0 (static: FOO, size: 8, align: 4) {
2020-04-11T12:21:59.9751299Z 29     ╾alloc24+0╼ 03 00 00 00                         │ ╾──╼....
2020-04-11T12:21:59.9751552Z 
