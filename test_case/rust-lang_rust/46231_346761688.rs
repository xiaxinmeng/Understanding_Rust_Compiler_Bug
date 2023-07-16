rust
#[derive(Copy, Clone)]
enum DesiredAction {
    Update,
    Borrow,
    Use,
    Assignment,
}
impl DesiredAction {
    fn as_noun(self) -> &'static str {
        match self {
            DesiredAction::Update => "update",
            ...
        }
    }
    fn as_verb_in_past_tense(self) -> &'static str {
        match self {
            DesiredAction::Update => "updated",
            ...
        }
    }
}
