 rust
state = match (p.token, next_state(state), next_state(next_state(state))) {
    (token::COLON, StateNone, _)   |
    (token::MOD_SEP, _, StateNone) => {
        p.bump();
        break 'statement;
    }
    (token::COLON, st, _)   |
    (token::MOD_SEP, _, st) => {
        p.bump();
        st
    }
    (token::EOF, _) => break 'statement,
    _ => state
}
