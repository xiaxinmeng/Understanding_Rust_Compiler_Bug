
PS C:\Users\aaron\Documents\github\uefi-div-bug> python3 ./run.py
rustc +nightly --version
rustc 1.61.0-nightly (76d770ac2 2022-04-02)
cargo +nightly build
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
cp target\x86_64-unknown-uefi\debug\uefi-div-bug.efi efi_partition\EFI/BOOT/BOOTX64.EFI
Traceback (most recent call last):
  File "C:\Users\aaron\Documents\github\uefi-div-bug\run.py", line 40, in <module>
    main()
  File "C:\Users\aaron\Documents\github\uefi-div-bug\run.py", line 25, in main
    run('cp', efi_app, os.path.join(efi_partition, 'EFI/BOOT/BOOTX64.EFI'))
  File "C:\Users\aaron\Documents\github\uefi-div-bug\run.py", line 10, in run
    subprocess.run(cmd, check=True)
  File "C:\Program Files\WindowsApps\PythonSoftwareFoundation.Python.3.10_3.10.1264.0_x64__qbz5n2kfra8p0\lib\subprocess.py", line 501, in run
    with Popen(*popenargs, **kwargs) as process:
  File "C:\Program Files\WindowsApps\PythonSoftwareFoundation.Python.3.10_3.10.1264.0_x64__qbz5n2kfra8p0\lib\subprocess.py", line 966, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "C:\Program Files\WindowsApps\PythonSoftwareFoundation.Python.3.10_3.10.1264.0_x64__qbz5n2kfra8p0\lib\subprocess.py", line 1435, in _execute_child
    hp, ht, pid, tid = _winapi.CreateProcess(executable, args,
FileNotFoundError: [WinError 2] The system cannot find the file specified
