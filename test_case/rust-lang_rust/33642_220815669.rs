 diff
--- pretty.expected.rs  2016-05-22 13:39:14.080831211 +0800
+++ pretty.actual.rs    2016-05-22 13:38:56.912831929 +0800
@@ -178,7 +178,6 @@
        , b = "hijklmn" , c = 3 ) , "abcd hijk 4\nabc hij 3");
     t!(format ! ( "{a:.*} {0} {:.*}" , 4 , 3 , "efgh" , a = "abcdef" ) ,
        "abcd 4 efg");
-
     // Test that pointers don't get truncated.
     {
         let val = usize::MAX;
