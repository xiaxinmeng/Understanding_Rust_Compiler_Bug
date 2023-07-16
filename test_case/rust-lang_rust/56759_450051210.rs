rust
struct S; // Candidate 1
fn main() {
    use self::S; // Candidate 2
    {
        use self::S; // Candidate 3
        use S as _; // Not an error
    }
}
