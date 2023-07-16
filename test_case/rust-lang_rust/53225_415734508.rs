diff
    ---- [ui] ui/nll/user-annotations/dump-fn-method.rs stdout ----
    diff of stderr:

1	error: user substs: Canonical { variables: [], value: [u32] }
-	  --> $DIR/dump-fn-method.rs:35:13
+	  --> $DIR/dump-fn-method.rs:36:13
3	   |
4	LL |     let x = foo::<u32>; //~ ERROR [u32]
5	   |             ^^^^^^^^^^

6	
7	error: user substs: Canonical { variables: [CanonicalVarInfo { kind: Ty(General) }, CanonicalVarInfo { kind: Ty(General) }], value: [?0, u32, ?1] }
-	  --> $DIR/dump-fn-method.rs:39:13
+	  --> $DIR/dump-fn-method.rs:42:13
9	   |
10	LL |     let x = <_ as Bazoom<u32>>::method::<_>; //~ ERROR [?0, u32, ?1]
11	   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

12	
13	error: user substs: Canonical { variables: [], value: [u8, u16, u32] }
-	  --> $DIR/dump-fn-method.rs:43:13
+	  --> $DIR/dump-fn-method.rs:46:13
15	   |
16	LL |     let x = <u8 as Bazoom<u16>>::method::<u32>; //~ ERROR [u8, u16, u32]
17	   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

18	
19	error: user substs: Canonical { variables: [CanonicalVarInfo { kind: Ty(General) }, CanonicalVarInfo { kind: Ty(General) }], value: [?0, ?1, u32] }
-	  --> $DIR/dump-fn-method.rs:49:5
+	  --> $DIR/dump-fn-method.rs:54:5
21	   |
22	LL |     y.method::<u32>(44, 66); //~ ERROR [?0, ?1, u32]
23	   |     ^^^^^^^^^^^^^^^^^^^^^^^
