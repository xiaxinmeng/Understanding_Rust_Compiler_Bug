py
import string

variant_names = [f"V{i}" for i in range(1024)]

def make_name(pat):
    return ''.join(["F" if x else "E" for x in pat])

def make_enum(pat):
    name = make_name(pat)
    buf = "#[cfg_attr(derived, derive(PartialOrd))]\n"
    buf += "#[derive(PartialEq)]\n"
    buf += f"pub enum {name}" + " {\n"
    for var, x in zip(variant_names, pat):
        buf += '    ' + var
        if x:
            buf += "(usize)"
        buf += ",\n"
    buf += "}\n"
    buf += "#[cfg(not(derived))]\n"
    buf += f"impl PartialOrd for {name} {{\n"
    buf += "    #[inline]\n"
    buf += "    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {\n"
    buf += "        let l_tag = ::core::intrinsics::discriminant_value(self);\n"
    buf += "        let r_tag = ::core::intrinsics::discriminant_value(other);\n"
    buf += "        match (self, other) {\n"
    for var, x in zip(variant_names, pat):
        if x:
            buf += f"           (Self::{var}(l), Self::{var}(r)) => PartialOrd::partial_cmp(l, r),\n"
    buf += "            _ => PartialOrd::partial_cmp(&l_tag, &r_tag),\n"
    buf += "        }\n"
    buf += "    }\n"
    buf += "}\n"
    return buf

out = """#![feature(core_intrinsics, bench_black_box)]
#![allow(non_snake_case, unreachable_patterns)]
use std::hint::black_box;
extern crate core;
"""

# Test cases: Two equal dataless
# Two different dataless
# Two equal dataful with same data
# Two equal dataful with different data

def make_tests(pat):
    name = make_name(pat)
    buf = f"fn test_{name}() -> (u128, u128, u128, u128)" + " {\n"
    buf += "    let one = {"
    dataless = [name for name, x in zip(variant_names, pat) if not x]
    dataful = [name for name, x in zip(variant_names, pat) if x]
    if not dataless:
        buf += "0\n"
    else:
        variant = dataless[0]
        buf += f"let (l, r) = ({name}::{variant}, {name}::{variant});" + '\n'
        buf += "let now = std::time::Instant::now();\n"
        buf += "for _ in 0..500000 {black_box(PartialOrd::partial_cmp(black_box(&l), black_box(&r)));}\n"
        buf += "now.elapsed().as_nanos()\n"
    buf += "    };\n"

    buf += "    let two = {"
    if len(dataless) < 2:
        buf += "0\n"
    else:
        var_l, var_r = dataless[:2]
        buf += f"let (l, r) = ({name}::{var_l}, {name}::{var_r});" + '\n'
        buf += "let now = std::time::Instant::now();\n"
        buf += "for _ in 0..500000 {black_box(PartialOrd::partial_cmp(black_box(&l), black_box(&r)));}\n"
        buf += "now.elapsed().as_nanos()\n"
    buf += "    };\n"

    buf += "    let three = {"
    if not dataful:
        buf += "0\n"
    else:
        variant = dataful[0]
        buf += f"let (l, r) = ({name}::{variant}(10), {name}::{variant}(10));" + '\n'
        buf += "let now = std::time::Instant::now();\n"
        buf += "for _ in 0..500000 {black_box(PartialOrd::partial_cmp(black_box(&l), black_box(&r)));}\n"
        buf += "now.elapsed().as_nanos()\n"
    buf += "    };\n"

    buf += "    let four = {"
    if len(dataful) < 2:
        buf += "0\n"
    else:
        var_l, var_r = dataful[:2]
        buf += f"let (l, r) = ({name}::{var_l}(10), {name}::{var_r}(10));" + '\n'
        buf += "let now = std::time::Instant::now();\n"
        buf += "for _ in 0..500000 {black_box(PartialOrd::partial_cmp(black_box(&l), black_box(&r)));}\n"
        buf += "now.elapsed().as_nanos()\n"
    buf += "    };\n"

    buf += "    (one, two, three, four)\n"
    buf += "}\n"
    return buf

names = []
for i in range(50):
    pat = [int(x) for x in f"{i:b}"]
    names.append(make_name(pat))
    out += make_enum(pat) + '\n'
    out += make_tests(pat)

for pat in [0b10101010101010101010101, 1<<250 | 1 << 100, (1<<500) - 1, ((1<<500)-1) ^ (1<<100) ^ (1<<200)]:
    pat = [int(x) for x in f"{pat:b}"]
    name = make_name(pat)
    names.append(name)
    out += make_enum(pat) + '\n'
    out += make_tests(pat)

out += "fn main() {\n"
out += 'println!("name,equal_dataless,inequal_dataless,equal_dataful,inequal_dataful,total_ns");\n'
for name in names:
    out += f"let (a, b, c, d) = test_{name}();\n"
    out += f'println!("{name},{{a}},{{b}},{{c}},{{d}},{{}}", a+b+c+d);\n'
out += "}\n"
print(out)
