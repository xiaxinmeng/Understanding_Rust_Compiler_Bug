rust
trait CloneColor where Self: Color, <Self as Color>::Channel: Clone {}
