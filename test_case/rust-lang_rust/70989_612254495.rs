diff
23	    }
24	}
25	
+	// HACK(eddyb) this is for testing if PR CI catches a 32-bit mir-opt failure.
+	
26	alloc0 (static: FOO, size: 8, align: 4) {
27	    ╾alloc24+0╼ 03 00 00 00                         │ ╾──╼....
28	}
