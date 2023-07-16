rust
if can_begin_expr(token) && !can_begin_type(token) {
    if !is_whitelisted(token) {
        span_err("conservative error");
    }
    parse_expr(Restrictions::CLOSING_ANGLE_BRACKET)
} else {
    parse_type()
}
