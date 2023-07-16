patch
diff --git a/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.stderr b/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.stderr
index a6f8563a047..1df7fd59f57 100644
--- a/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.stderr
+++ b/src/test/ui/half-open-range-patterns/exclusive_range_pattern_syntax_collision.stderr
@@ -8,6 +8,10 @@ LL |         [_, 99.., _] => {},
    |
    = note: expected struct `std::ops::Range<{integer}>`
                 found type `{integer}`
+help: you might have meant to use field `start` whose type is `{integer}`
+   |
+LL |     match [5..4, 99..105, 43..44].start {
+   |           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
