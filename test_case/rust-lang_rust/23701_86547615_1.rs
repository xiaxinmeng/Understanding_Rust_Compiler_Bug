
macro_rules! getarg {
    (x => $e:expr) => (println! ("x = {}", $e));
    ($b:block) => $b;
}
