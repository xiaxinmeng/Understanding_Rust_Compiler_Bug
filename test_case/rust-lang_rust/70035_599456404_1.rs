bash
run_test() {
    echo -n "Testing $1 ... "
    file="$(mktemp)"
    outfile="$(mktemp)"
    python -c "print($1, end='')" > "$file"
    echo -n "file=$(numfmt --to=iec-i --suffix=B $(stat --printf="%s" "$file")) ... "

    kb="$(/usr/bin/time -v rustc -A warnings -O -o "$outfile" /dev/stdin <<EOT 2>&1 | grep 'Maximum resident set size' | cut -d ':' -f 2
use std::collections::HashMap;

static TESTDATA: &str = include_str!("$file");

fn parse_testdata() -> $outtype {
    let mut map = Default::default();
    for line in TESTDATA.lines() {
    }
    map
}

pub fn do_stuff() {
    let (tx, _) = std::sync::mpsc::channel();
    let testdata = parse_testdata();
    tx.send(vec![0]).unwrap();
}

fn main() {
    do_stuff();
}
EOT
)"


    echo "ram=$(numfmt --to=iec-i --suffix=B --from-unit 1024 $kb)"
    rm "$file" "$outfile"
}

run_all() {
    echo "Running $outtype tests"
    run_test "'foo bar baz qux 1337 42\n' * 1_000_000"
    run_test "'foo bar baz qux 1337 42 ' * 1_000_000"
    run_test "'a' * 25*1024*1024"
    run_test "'a' * 10*1024*1024"
    run_test "'a' * 1*1024*1024"
    echo
}

outtype="HashMap<(), ()>" run_all
outtype="Vec<()>" run_all
rustc --version
