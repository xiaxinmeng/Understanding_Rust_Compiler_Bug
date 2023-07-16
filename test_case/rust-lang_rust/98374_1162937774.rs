python
import csv
import dataclasses
import os
import subprocess
import time
import typing

def _gen_struct_definition(name: str, num_fields: int)->str:
    fields = (
        f"\ta{i}: String,"
        for i in range(num_fields)
    )
    fields = f"\n".join(fields)
    return f"""
    #[derive(Debug)]
    struct {name} {{
        {fields}
    }}"""

def _gen_clone(num_fields: int)->str:
    fields = (
        f"\t\t\t\ta{i}: self.a{i}.clone(),"
        for i in range(num_fields)
    )
    fields = f"\n".join(fields)
    return f"""
        #[inline]
        fn clone(&self)->Self{{
            Self{{
                {fields}
            }}
        }}
    """

def _gen_clone_from(num_fields: int)->str:
    fields = (
        f"\t\t\tself.a{i}.clone_from(&other.a{i});"
        for i in range(num_fields)
    )
    fields = f"\n".join(fields)
    return f"""
        #[inline]
        fn clone_from(&mut self, other: &Self){{
            {fields}
        }}
    """

def _generate_trait_impl(name: str, num_fields: int, make_clone_from: bool)->str:
    bodies = _gen_clone(num_fields)
    if make_clone_from:
        bodies += f"\n {_gen_clone_from(num_fields)}"
    return f"""
    impl Clone for {name} {{
        {bodies}
    }}
    """

def _generate_file(fname: str, num_structs: int, num_fields: int, make_clone_from: bool):
    with open(fname, "w") as f:
        for i in range(num_structs):
            struct_name = f"Struct{i}"
            f.write(_gen_struct_definition(struct_name, num_fields))
            f.write("\n")
            f.write(_generate_trait_impl(struct_name, num_fields, make_clone_from))
            f.write("\n")

def _generate_and_measure(num_structs: int, num_fields: int, make_clone_from: bool, do_optimize: bool)->float:
    if do_optimize:
        c_opt_level = "-Copt-level=3"
    else:
        c_opt_level = "-Copt-level=0"

    rustc_command = f"rustc measure_clone.rs --crate-type=rlib -Ccodegen-units=1 {c_opt_level}"

    _generate_file("measure_clone.rs", num_structs, num_fields, make_clone_from)

    def clean_prev_compilation():
        for item in os.listdir():
            if item.endswith(".rlib"):
                os.remove(item)

    # Run 3 compilations and get median
    timings = []
    for _ in range(3):
        clean_prev_compilation()
        begin = time.time()
        subprocess.run(rustc_command)
        end = time.time()
        timings.append(end - begin)
    clean_prev_compilation()

    timings.sort()
    return timings[ len(timings)//2 ]


@dataclasses.dataclass(frozen=True)
class Measurement:
    num_structs: int
    secs_with_old_dbg: float
    secs_with_clone_from_dbg: float
    secs_with_old_release: float
    secs_with_clone_from_release: float


def _generate_data()->tuple[Measurement, ...]:
    min_structs = 100
    max_structs = 1000
    step = 100
    fields = 20
    rows = []
    for num_structs in range(min_structs, max_structs+step, step):
        row = Measurement(
            num_structs=num_structs,
            secs_with_old_dbg=_generate_and_measure(
                num_structs, fields,
                make_clone_from=False, do_optimize=False
            ),
            secs_with_clone_from_dbg=_generate_and_measure(
                num_structs, fields,
                make_clone_from=True, do_optimize=False
            ),
            secs_with_old_release=_generate_and_measure(
                num_structs, fields,
                make_clone_from=False, do_optimize=True
            ),
            secs_with_clone_from_release=_generate_and_measure(
                num_structs, fields,
                make_clone_from=True, do_optimize=True
            ),
        )
        rows.append(row)
    return tuple(rows)

def _save_to_csv(rows: typing.Iterable[Measurement]):
    fields = [x.name for x in dataclasses.fields(Measurement)]
    with open("results.csv", "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for row in rows:
            writer.writerow(dataclasses.asdict(row))

if __name__ == "__main__":
    data = _generate_data()
    _save_to_csv(data)
    print("Finished!")
