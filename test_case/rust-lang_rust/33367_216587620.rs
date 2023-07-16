 rust
// this would replace line 561
if check_old_skool() {
    err.note(&format("{}", terr));
} else {
    err.span_label(trace.origin.span(), &terr);
}
