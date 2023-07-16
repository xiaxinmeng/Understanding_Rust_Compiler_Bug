 rust
use std::io;
use ptrace::word;

mod posix {
    use std::libc;
    use std::os;
    use std::ptr;
    use std::str;
    use std::vec;

    mod c {
        use std::libc;

        extern {
            fn fork() -> libc::pid_t;
            fn exit(status: libc::c_int) -> !;
            fn waitpid(pid: libc::pid_t, status: *libc::c_int, flags: libc::c_int) -> libc::c_int;
            fn execvp(file: *libc::c_char, argv: **libc::c_char) -> !;
        }
    }

    pub enum ForkResult {
        ForkFailure(int),
        ForkChild,
        ForkParent(int),
    }

    pub enum WaitPidResult {
        WaitPidFailure(int),
        WaitPidSuccess(int, int),
    }

    pub fn fork() -> ForkResult {
        unsafe {
            let pid = c::fork();

            match pid {
                -1 => ForkFailure(os::errno()),
                0 => ForkChild,
                pid => ForkParent(pid as int),
            }
        }
    }

    pub fn waitpid(pid: int, flags: int) -> WaitPidResult {
        unsafe {
            let status : libc::c_int = 0;

            let pid = c::waitpid(pid as libc::pid_t, &status as *libc::c_int, flags as libc::c_int);

            if pid == -1 {
                WaitPidFailure(os::errno())
            } else {
                WaitPidSuccess(pid as int, status as int)
            }
        }
    }

    // this is probably pretty awful...
    fn str_array_to_char_pp(ary: &[~str], callback: &fn(**libc::c_char)) {
        fn helper_fn(ptrs: &mut ~[*libc::c_char], ary: &[~str], callback: &fn(**libc::c_char)) {
            match ary {
                [] => {
                    ptrs.push(ptr::null());
                    callback(vec::raw::to_ptr(*ptrs));
                },
                    [ref head, ..tail] => {
                        do head.to_c_str().with_ref() |raw_str| {
                            ptrs.push(raw_str);
                            helper_fn(ptrs, tail, |c| callback(c));
                        }
                    },
            }
        }

        let mut ptrs : ~[*libc::c_char] = vec::with_capacity(ary.len());

        helper_fn(&mut ptrs, ary, callback);
    }

    pub fn exec(command_and_args: &[~str]) {
        unsafe {
            do command_and_args[0].to_c_str().with_ref() |command| {
                do str_array_to_char_pp(command_and_args) |args| {
                    c::execvp(command, args);
                }
            }
        }
    }

    pub fn exit(status: int) -> ! {
        unsafe {
            c::exit(status as libc::c_int)
        }
    }

    pub static SIGTRAP : int = 5;
}

mod ptrace {
    use std::libc;
    use std::ptr;

    extern {
        fn ptrace(request: libc::c_int, pid: libc::pid_t, addr: *libc::c_void, data: *libc::c_void) -> libc::c_long;
    }

    pub type word = u64;

    static TRACEME : libc::c_int = 0;
    static SYSCALL : libc::c_int = 24;
    static SETOPTIONS : libc::c_int = 0x4200;

    pub fn trace_me() {
        unsafe {
            ptrace(TRACEME, 0, ptr::null(), ptr::null());
        }
    }

    pub fn setoptions(pid: int, options: int) {
        unsafe {
            ptrace(SETOPTIONS, pid as libc::pid_t, ptr::null(), options as *libc::c_void);
        }
    }

    pub fn syscall(pid: int) {
        unsafe {
            ptrace(SYSCALL, pid as libc::pid_t, ptr::null(), ptr::null());
        }
    }

    pub static TRACESYSGOOD : int = 0x00000001;
    pub static TRACEFORK : int = 0x00000002;
    pub static TRACEEXEC : int = 0x00000010;
}

enum TraceEvent {
    SystemCall {
        syscall_no : word,
        arguments : (word, word, word, word, word, word),
    },
    Other,
}

fn init_trace(child_pid: int) {
    match posix::waitpid(child_pid, 0) {
        posix::WaitPidFailure(_) => (),
        posix::WaitPidSuccess(pid, status) => {
            if status & posix::SIGTRAP != 0 {
                ptrace::setoptions(pid, ptrace::TRACEFORK | ptrace::TRACESYSGOOD | ptrace::TRACEEXEC);
                ptrace::syscall(pid);
            }
        },
    }
}

fn resume_trace(child_pid: int) {
    ptrace::syscall(child_pid);
}

fn next_trace(callback: &fn(int, TraceEvent)) -> bool {
    loop {
        let result = posix::waitpid(-1, 0);

        match result {
            posix::WaitPidFailure(_) => return false,
            posix::WaitPidSuccess(pid, status) => {
                if ((status >> 8) & (0x80 | posix::SIGTRAP)) != 0 {
                    callback(pid, SystemCall {
                             syscall_no: 0,
                             arguments: ( 0, 0, 0, 0, 0, 0 ),
                             });
                } else {
                    callback(pid, Other);
                }
                resume_trace(pid);
            },
        }
    }
}

fn run_parent(child_pid: int) {
    init_trace(child_pid);

    do next_trace() |_, event| {
        match event {
            // XXX changing _args to _ fixes the segfault (?!?!?!)
            SystemCall { syscall_no: _, arguments: _args } => {
            }
            _ => (),
        };
    };
}

fn main() {
    let result = posix::fork();

    match result {
        posix::ForkChild => {
            ptrace::trace_me();

            posix::exec([~"ls"]);
            posix::exit(255);
        }
        posix::ForkFailure(_) => {
            io::println("An error occurred");
        }
        posix::ForkParent(child_pid) => {
            run_parent(child_pid);
        }
    }
}
