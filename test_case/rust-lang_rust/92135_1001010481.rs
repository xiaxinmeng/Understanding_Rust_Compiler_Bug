python
import sys
import subprocess


def parse_stdout_time(output)->float:
    output = output.strip()
    last = output.split()[-1]
    last = last.decode('ascii')
    assert last[-1] == 's', last
    last = last[:-1]
    parsed = float(last)
    return parsed

def do_check()->float:
    res = subprocess.run(["cargo", "+stage1", "check"], capture_output=True)
    return parse_stdout_time(res.stderr)

def do_build()->float:
    res = subprocess.run(["cargo", "+stage1", "bench", "--no-run"], capture_output=True)
    return parse_stdout_time(res.stderr)

def do_clean():
    subprocess.run(["cargo", "+stage1", "clean"], capture_output=True)

NUM_TRIES = 51

def print_agg(measures):
    mean = sum(measures) / len(measures)
    median = sorted(measures)[len(measures)//2]
    variance = sum((x-mean)*(x-mean) for x in measures)/len(measures)
    stddev = variance**(1/2)
    labels = ['mean', 'median', 'stddev', 'variance']
    values = [mean, median, stddev, variance]
    f = '\t'.join(f'{s}: {v}' for s, v in zip(labels, values))
    print(f"Got {f}")

def try_many_times(command):
    do_clean()
    measures = []
    for i in range(NUM_TRIES):
        print(f"Running test #{i}")
        measures.append(command())
        print(f'#{i} finished in\t{measures[-1]}')
        do_clean()
    print_agg(measures)


if '--check' in sys.argv:
    print("Would run check")
    command = do_check
elif '--build' in sys.argv:
    print("Would run bench --no-run")
    command = do_build
else:
    print("Please, pass either '--check' or '--build'")
    exit(1)

try_many_times(command)
