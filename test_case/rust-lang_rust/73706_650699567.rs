
failures:

---- [ui] ui/proc-macro/meta-macro-hygiene.rs stdout ----
diff of stdout:
#1: parent: #0, outer_mark: (ExpnId(1), Opaque)
#2: parent: #0, outer_mark: (ExpnId(1), Transparent)
#3: parent: #0, outer_mark: (ExpnId(2), Opaque)
#4: parent: #0, outer_mark: (ExpnId(2), Transparent)
#5: parent: #0, outer_mark: (ExpnId(2), SemiTransparent)
*/

------------------------------------------
stderr:
------------------------------------------

------------------------------------------



failures:
    [ui] ui/proc-macro/meta-macro-hygiene.rs

test result: FAILED. 10350 passed; 1 failed; 62 ignored; 0 measured; 0 filtered out
