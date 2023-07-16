diff
use std::collections::BTreeMap;

fn my_retain(map: &mut BTreeMap<i32, i32>, pred: impl Fn(&i32) -> bool) {
-    let pred_better = |_, val: &mut i32| {
+    let pred_better = |_: &_, val: &mut i32| {
        pred(val)
    };
    map.retain(pred_better);
}

fn main() {}
