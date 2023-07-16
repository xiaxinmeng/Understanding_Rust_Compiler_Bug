

---- [ui] ui/parser/issue-68730.rs stdout ----
diff of stderr:

33         |        ^
34
35      error: expected one of `>`, `const`, identifier, or lifetime, found `<`
-         --> $DIR/issue-68730.rs:5:11
+         --> $DIR/issue-68730.rs:5:10
37         |
38      LL | enumem˂˂
-          |         ^ expected one of `>`, `const`, identifier, or lifetime
+          |        ^ expected one of `>`, `const`, identifier, or lifetime
40
41      error: aborting due to 5 previous errors
42

