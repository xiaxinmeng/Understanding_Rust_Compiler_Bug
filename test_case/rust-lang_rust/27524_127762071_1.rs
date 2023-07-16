 rust
struct Sanitized;
struct Raw;

struct String<Sanitization> {
    data: String,
    _sanitization: PhantomData<Sanitization>,
}
