plain
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:726:1
    |
726 | / iter_n! {
727 | |     #[stable(feature = "rust1", since = "1.0.0")]
728 | |     #[fused(stable(feature = "fused", since = "1.26.0"))]
729 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
747 | |     fn max_items;
748 | | }
    | |_- in this macro invocation
    |
    |
    = note: `-D missing-docs` implied by `-D warnings`
error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | /  macro_rules! iter_n {
751 | /  macro_rules! iter_n {
752 | |      (
753 | |          #[$stability:meta]
754 | |          #[fused($fused_stability:meta)]
...   |
823 | |              pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |      };
828 | |  }
    | |__- in this expansion of `iter_n!`
    | |__- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:750:1
    |
750 |  / iter_n! {
751 |  |     #[stable(feature = "rust1", since = "1.0.0")]
752 |  |     #[fused(stable(feature = "fused", since = "1.26.0"))]
753 |  |     #[must_use = "iterators are lazy and do nothing unless consumed"]
772 |  |     fn max_items;
773 |  | }
    |  |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:775:1
    |
775 | /  iter_n! {
776 | |      #[stable(feature = "rust1", since = "1.0.0")]
777 | |      #[fused(stable(feature = "fused", since = "1.26.0"))]
778 | |      #[must_use = "iterators are lazy and do nothing unless consumed"]
796 | |      fn max_items;
797 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:799:1
    |
799 | /  iter_n! {
800 | |      #[stable(feature = "rust1", since = "1.0.0")]
801 | |      #[fused(stable(feature = "fused", since = "1.26.0"))]
802 | |      #[must_use = "iterators are lazy and do nothing unless consumed"]
821 | |      fn max_items;
822 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 |  / macro_rules! iter_n {
752 |  |     (
753 |  |         #[$stability:meta]
754 |  |         #[fused($fused_stability:meta)]
...    |
823 |  |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827 |  |     };
828 |  | }
    |  |_- in this expansion of `iter_n!`
    |  |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:824:1
    |
824 | /  iter_n! {
825 | |      #[unstable(feature = "split_inclusive_variants", issue = "none")]
826 | |      #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
827 | |      #[must_use = "iterators are lazy and do nothing unless consumed"]
848 | |      fn max_items;
849 | |  }
    | |__- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:851:1
    |
851 | / iter_n! {
852 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
853 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
854 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
875 | |     fn max_items;
876 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:878:1
    |
878 | / iter_n! {
879 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
880 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
881 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
902 | |     fn max_items;
903 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:905:1
    |
905 | / iter_n! {
906 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
907 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
908 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
929 | |     fn max_items;
930 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:932:1
    |
932 | / iter_n! {
933 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
934 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
935 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
956 | |     fn max_items;
957 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
   --> library/core/src/slice/iter/macros.rs:823:13
    |
751 | / macro_rules! iter_n {
752 | |     (
753 | |         #[$stability:meta]
754 | |         #[fused($fused_stability:meta)]
...   |
823 | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...   |
827 | |     };
828 | | }
    | |_- in this expansion of `iter_n!`
    | |_- in this expansion of `iter_n!`
    |
   ::: library/core/src/slice/iter.rs:959:1
    |
959 | / iter_n! {
960 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
961 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
962 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
983 | |     fn max_items;
984 | | }
    | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/slice/iter/macros.rs:823:13
     |
751  | / macro_rules! iter_n {
752  | |     (
753  | |         #[$stability:meta]
754  | |         #[fused($fused_stability:meta)]
...    |
823  | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827  | |     };
828  | | }
     | |_- in this expansion of `iter_n!`
     | |_- in this expansion of `iter_n!`
     |
    ::: library/core/src/slice/iter.rs:986:1
     |
986  | / iter_n! {
987  | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
988  | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
989  | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
1010 | |     fn max_items;
1011 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/slice/iter/macros.rs:823:13
     |
751  | / macro_rules! iter_n {
752  | |     (
753  | |         #[$stability:meta]
754  | |         #[fused($fused_stability:meta)]
...    |
823  | |             pub fn max_items(self, n: usize) -> $iter_n<'a, T, P> {
...    |
827  | |     };
828  | | }
     | |_- in this expansion of `iter_n!`
     | |_- in this expansion of `iter_n!`
     |
    ::: library/core/src/slice/iter.rs:1013:1
     |
1013 | / iter_n! {
1014 | |     #[unstable(feature = "split_inclusive_variants", issue = "none")]
1015 | |     #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1016 | |     #[must_use = "iterators are lazy and do nothing unless consumed"]
1037 | |     fn max_items;
1038 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:849:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
849  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1013 | / generate_n_iterators! {
1014 | |     forward:
1015 | |         #[stable(feature = "rust1", since = "1.0.0")]
1016 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1040 | |         fn as_str;
1041 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1013 | / generate_n_iterators! {
1014 | |     forward:
1015 | |         #[stable(feature = "rust1", since = "1.0.0")]
1016 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1040 | |         fn as_str;
1041 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:927:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
927  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1013 | / generate_n_iterators! {
1014 | |     forward:
1015 | |         #[stable(feature = "rust1", since = "1.0.0")]
1016 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1040 | |         fn as_str;
1041 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1013 | / generate_n_iterators! {
1014 | |     forward:
1015 | |         #[stable(feature = "rust1", since = "1.0.0")]
1016 | |         #[fused(stable(feature = "fused", since = "1.26.0"))]
1040 | |         fn as_str;
1041 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
533  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1051 | / generate_pattern_iterators! {
1052 | |     forward:
1053 | |         #[stable(feature = "split_inclusive", since = "1.51.0")]
1054 | |         #[fused(stable(feature = "split_inclusive", since = "1.51.0"))]
1089 | |     delegate double ended;
1090 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:849:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
849  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1092 | / generate_n_iterators! {
1093 | |     forward:
1094 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1095 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1119 | |         fn as_str;
1120 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1092 | / generate_n_iterators! {
1093 | |     forward:
1094 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1095 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1119 | |         fn as_str;
1120 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:927:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
927  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1092 | / generate_n_iterators! {
1093 | |     forward:
1094 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1095 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1119 | |         fn as_str;
1120 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1092 | / generate_n_iterators! {
1093 | |     forward:
1094 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1095 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1119 | |         fn as_str;
1120 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
533  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1130 | / generate_pattern_iterators! {
1131 | |     forward:
1132 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1133 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1169 | |     delegate double ended;
1170 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:849:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
849  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1172 | / generate_n_iterators! {
1173 | |     forward:
1174 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1175 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1199 | |         fn as_str;
1200 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1172 | / generate_n_iterators! {
1173 | |     forward:
1174 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1175 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1199 | |         fn as_str;
1200 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:927:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
927  | |             pub fn as_str(&self) -> &'a str {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1172 | / generate_n_iterators! {
1173 | |     forward:
1174 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1175 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1199 | |         fn as_str;
1200 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1172 | / generate_n_iterators! {
1173 | |     forward:
1174 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1175 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1199 | |         fn as_str;
1200 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
484  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1210 | / generate_pattern_iterators! {
1211 | |     forward:
1212 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1213 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1235 | |     delegate double ended;
1236 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
533  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1210 | / generate_pattern_iterators! {
1211 | |     forward:
1212 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1213 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1235 | |     delegate double ended;
1236 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1238 | / generate_n_iterators! {
1239 | |     forward:
1240 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1241 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1259 | |         fn as_str;
1260 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1238 | / generate_n_iterators! {
1239 | |     forward:
1240 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1241 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1259 | |         fn as_str;
1260 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1324 | / generate_n_iterators! {
1325 | |     forward:
1326 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1327 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1345 | |         fn as_str;
1346 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1324 | / generate_n_iterators! {
1325 | |     forward:
1326 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1327 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1345 | |         fn as_str;
1346 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
484  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1357 | / generate_pattern_iterators! {
1358 | |     forward:
1359 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1360 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1382 | |     delegate double ended;
1383 | | }
     | |_- in this macro invocation

---
423  | |     {
424  | |         // Forward iterator
425  | |         forward:
...    |
533  | |             pub fn as_str(&self) -> &'a str {
...    |
589  | |     } => {}
590  | | }
     | |_- in this expansion of `generate_pattern_iterators!`
     | |_- in this expansion of `generate_pattern_iterators!`
...
1357 | / generate_pattern_iterators! {
1358 | |     forward:
1359 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1360 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1382 | |     delegate double ended;
1383 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:857:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
857  | |             pub fn max_items(self, n: usize) -> $forward_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1385 | / generate_n_iterators! {
1386 | |     forward:
1387 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1388 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1406 | |         fn as_str;
1407 | | }
     | |_- in this macro invocation


error: missing documentation for an associated function
    --> library/core/src/str/iter.rs:939:13
     |
771  | / macro_rules! generate_n_iterators {
772  | |     (
773  | |         forward:
774  | |             #[$forward_stability_attribute:meta]
...    |
939  | |             pub fn max_items(self, n: usize) -> $reverse_n_iterator<'a, P> {
...    |
949  | |     }
950  | | }
950  | | }
     | |_- in this expansion of `generate_n_iterators!`
...
1385 | / generate_n_iterators! {
1386 | |     forward:
1387 | |         #[unstable(feature = "split_inclusive_variants", issue = "none")]
1388 | |         #[fused(unstable(feature = "split_inclusive_variants", issue = "none"))]
1406 | |         fn as_str;
1407 | | }
     | |_- in this macro invocation

