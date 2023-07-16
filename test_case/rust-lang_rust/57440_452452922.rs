
pub fn cast_s2<'a, 'b: 'a>(s2: S2<I<'b>>) -> S2<I<'a>> {
    S2 { t: s2.t, a: s2.a, }
}
