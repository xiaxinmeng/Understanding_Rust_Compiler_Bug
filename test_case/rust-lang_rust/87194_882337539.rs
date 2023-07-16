
<const> = <type> <const-data>
        | "p" // placeholder, shown as _
        | <backref>

// The encoding of a constant depends on its type. Integers use their value,
// in base 16 (0-9a-f), not their memory representation. Negative integer
// values are preceded with "n". The bool value false is encoded as `0_`, true
// value as `1_`. The char constants are encoded using their Unicode scalar
// value.
<const-data> = ["n"] {<hex-digit>} "_"
