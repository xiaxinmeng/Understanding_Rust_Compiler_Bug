 rust
extern mod extra;
use std::hashmap::HashMap;
use std::*;

enum StateMesg
{
    AddListener(~str, comm::Chan<int>),    // str is used to identify the listener
    RemoveListener(~str),
    Shutdown,
}

type StatePort = comm::Port<StateMesg>;
type StateChan = comm::Chan<StateMesg>;

pub enum ControlEvent
{
    RefreshEvent,
    CloseEvent,
}
pub type PushChan = comm::SharedChan<~str>;
pub type ControlPort = comm::Port<ControlEvent>;
pub type ControlChan = comm::Chan<ControlEvent>;
pub type OpenSse = ~fn (channel: PushChan) -> ControlChan;
pub type SseTasks = ~[(~str, ControlChan)];

fn main()
{
    let state_chan = comm::SharedChan::new(manage_state());
    let up: OpenSse = |push| {uptime_sse(state_chan.clone(), push)};

    let (exit_port, exit_chan) = comm::stream();
    handle_connection(exit_port, up);

    unsafe {
        libc::sleep(3);
        exit_chan.send(());

        libc::sleep(1);
        libc::exit(0);
    }
}

fn manage_state() -> StateChan
{
    error!("starting manage_state");
    let (state_port, state_chan) : (StatePort, StateChan) = comm::stream();
    do task::spawn_sched(task::ManualThreads(1))
    {
        let mut time = 0;
        let mut listeners = HashMap::new();
        loop
        {
            time += 1;
            unsafe {
            libc::sleep(1);
            }
            error!("sending new state");
            for listeners.each_value |ch: &comm::Chan<int>| {ch.send(time)};

            if state_port.peek()
            {
                match state_port.recv()
                {
                    AddListener(key, ch) =>
                    {
                        let added = listeners.insert(key, ch);
                        assert!(added);
                    }
                    RemoveListener(key) =>
                    {
                        listeners.remove(&key);
                    }
                    Shutdown =>
                    {
                        error!("exiting manage_state");
                        break;
                    }
                }
            }
        }
    }
    state_chan
}

fn uptime_sse(registrar: comm::SharedChan<StateMesg>, push: PushChan) -> ControlChan
{
    error!("starting sse client");
    let (control_port, control_chan): (ControlPort, ControlChan) = comm::stream();
    let control_cell = cell::Cell(control_port);
    do task::spawn_sched(task::ThreadPerCore)
    {
        let mut (notify_port, notify_chan) = comm::stream();

        let mut control_port = control_cell.take();

        let key = fmt!("uptime %?", ptr::to_unsafe_ptr(&notify_port));
        registrar.send(AddListener(copy key, notify_chan));

        loop
        {
            let mut time = 0;
            match comm::select2i(&mut notify_port, &mut control_port)
            {
                either::Left(_) =>
                {
                    error!("pushing new state");
                    let new_time = notify_port.recv();
                    time = new_time;
                    push.send(fmt!("retry: 5000\ndata: %?\n\n", time));
                }
                either::Right(_) =>
                {
                    match control_port.recv()
                    {
                        RefreshEvent =>
                        {
                            push.send(fmt!("retry: 5000\ndata: %?\n\n", time));
                        }
                        CloseEvent =>
                        {
                            error!("exiting sse client");
                            registrar.send(RemoveListener(key));
                            break;
                        }
                    }
                }
            }
        }
    }
    control_chan
}

pub fn handle_connection(exit: comm::Port<()>, opener: OpenSse)
{
    let exit_cell = cell::Cell(exit);
    error!("starting connection");
    do task::spawn_sched(task::ManualThreads(1))
    {
        let mut (sse_port, sse_chan) = comm::stream();
        let sse_chan = comm::SharedChan::new(sse_chan);
        let mut exit = exit_cell.take();

        let mut sse_tasks = ~[];
        let control_chan = opener(sse_chan);
        vec::push(&mut sse_tasks, (~"some path", control_chan));

        loop
        {
            match pipes::select2i(&mut exit, &mut sse_port)
            {
                either::Left(_) =>
                {
                    error!("exiting connection");
                    exit.recv();
                    close_sses(&sse_tasks);
                    break;
                }
                either::Right(_) =>
                {
                    let body = sse_port.recv();
                    error!("received %s state", body);
                }
            }
        }
    }
}

pub fn close_sses(tasks: &SseTasks)
{
    error!("closing all sse");
    for tasks.each |&(_path, control_ch)|
    {
        control_ch.send(CloseEvent);
    };
    // With this commented out we get the "you dun goofed" failure.
//    libc::funcs::posix88::uniextra::sleep(1);
}
