
---- [ui] src/test/ui/check-cfg/mix.rs stdout ----
diff of stderr:

28	LL | #[cfg_attr(uu, test)]
29	   |            ^^
30	
-	warning: unexpected condition value `bar` for condition name `feature`
-	   |
-	   = help: was set with `--cfg` but isn't in the `--check-cfg` expected values
-	
35	warning: unexpected `unknown_name` as condition name
36	   |
37	   = help: was set with `--cfg` but isn't in the `--check-cfg` expected names

+	
+	warning: unexpected condition value `bar` for condition name `feature`
+	   |
+	   = help: was set with `--cfg` but isn't in the `--check-cfg` expected values
38	
39	warning: unexpected `cfg` condition name
40	  --> $DIR/mix.rs:35:10

