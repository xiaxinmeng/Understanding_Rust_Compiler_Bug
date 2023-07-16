
extern mod std;
use core::send_map::linear::{LinearMap};

enum StateMesg
{
    AddListener(~str, pipes::Chan<int>),    // str is used to identify the listener
    RemoveListener(~str),
    Shutdown,
}

type StatePort = pipes::Port<StateMesg>;
type StateChan = pipes::Chan<StateMesg>;

pub enum ControlEvent
{
    RefreshEvent,
    CloseEvent,
}
pub type PushChan = pipes::SharedChan<~str>;
pub type ControlPort = pipes::Port<ControlEvent>;
pub type ControlChan = pipes::Chan<ControlEvent>;
pub type OpenSse = fn~ (channel: PushChan) -> ControlChan;
pub type SseTasks = ~[(~str, ControlChan)];

fn main()
{
    let state_chan = pipes::SharedChan(manage_state());
    let up: OpenSse = |push| {uptime_sse(state_chan.clone(), push)};

    let (exit_port, exit_chan) = pipes::stream();
    handle_connection(exit_port, up);

    libc::funcs::posix88::unistd::sleep(3);
//    state_chan.send(Shutdown);
    exit_chan.send(());

    // Not quite sure how to shutdown the manage_state task so for now
    // we'll forcibly exit.
    libc::funcs::posix88::unistd::sleep(1);
    libc::exit(0);
}

fn manage_state() -> StateChan
{
    error!("starting manage_state");
    let (state_port, state_chan) : (StatePort, StateChan) = pipes::stream();
    do task::spawn_sched(task::ManualThreads(1)) |move state_port|
    {
        let mut time = 0;
        let mut listeners = LinearMap();
        loop
        {
            time += 1;
            libc::funcs::posix88::unistd::sleep(1);
            error!("sending new state");
            for listeners.each_value |ch: &pipes::Chan<int>| {ch.send(copy(time))};

            if state_port.peek()
            {
                match state_port.recv()
                {
                    AddListener(key, ch) =>
                    {
                        let added = listeners.insert(key, ch);
                        assert added;
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

fn uptime_sse(registrar: pipes::SharedChan<StateMesg>, push: PushChan) -> ControlChan
{
    error!("starting sse client");
    let (control_port, control_chan): (ControlPort, ControlChan) = pipes::stream();
    do task::spawn_sched(task::ThreadPerCore) |move control_port, move registrar, move push|
    {
        let (notify_port, notify_chan) = pipes::stream();

        let key = fmt!("uptime %?", ptr::addr_of(&notify_port));
        registrar.send(AddListener(copy key, notify_chan));

        loop
        {
            let mut time = 0;
            match pipes::select2i(&notify_port, &control_port)
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

pub fn handle_connection(exit: pipes::Port<()>, opener: OpenSse)
{
    error!("starting connection");
    do task::spawn_sched(task::ManualThreads(1))
    {
        let (sse_port, sse_chan) = pipes::stream();
        let sse_chan = pipes::SharedChan(sse_chan);

        let mut sse_tasks = ~[];
        let control_chan = opener(sse_chan);
        vec::push(&mut sse_tasks, (~"some path", control_chan));

        loop
        {
            match pipes::select2i(&exit, &sse_port)
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
//    libc::funcs::posix88::unistd::sleep(1);
}

