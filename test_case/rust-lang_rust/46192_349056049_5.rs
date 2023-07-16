
impl<K, V> CountableMeter<K, V> for Count {
impl<K, V, T: Meter<K, V, Measure=usize>> CountableMeter<K, V> for T {
