 rust
struct Test;

impl KeyComparator<()> for Test {
    type K = IndexableValue<(), Test>;
}
