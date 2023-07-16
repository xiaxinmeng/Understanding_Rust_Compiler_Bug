
void main() {
    import std.stdio, std.meta, std.typecons;
    immutable tup = tuple(10, 20, 30);
    foreach (immutable i; AliasSeq!(0, 1, 2)) {
        writeln(tup[i]);
    }
}
