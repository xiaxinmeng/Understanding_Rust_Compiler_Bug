
fn print2<
    S1: Serializer,
    S2: Serializer,
    T: Serializable<@Serializer>
>(t: T, s1: @S1, s2: @S2) {
    t.serialize(s1 as @Serializer);
    io::println("");

    t.serialize(s2 as @Serializer);
    io::println("");
}
