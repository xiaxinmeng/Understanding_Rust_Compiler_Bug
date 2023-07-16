
// rustc --test sleep.rs && ./sleep
use std;

enum StateMesg
{
    AddListener(~str, comm::Chan<int>),    // str is used to identify the listener
    RemoveListener(~str),
    Shutdown,
}

type StateChan = comm::Chan<StateMesg>;

// Task used to maintain an uptime (in seconds) variable.
fn manage_state() -> StateChan
{
    do task::spawn_listener
    |state_port: comm::Port<StateMesg>|
    {
        let mut time = 0;
        let listeners = std::map::box_str_hash();
        loop
        {
            time += 1;
            info!("time = %?", time);
            libc::funcs::posix88::unistd::sleep(1);
            for listeners.each_value |ch| {comm::send(ch, copy(time))};

            if state_port.peek()
            {
                match state_port.recv()
                {
                    AddListener(key, ch) =>
                    {
                        let added = listeners.insert(@(copy key), ch);
                        assert added;
                    }
                    RemoveListener(key) =>
                    {
                        listeners.remove(@(copy key));
                    }
                    Shutdown =>
                    {
                        break;
                    }
                }
            }
        }
    }
}

enum ControlEvent
{
    CloseEvent,
}

type ControlPort = comm::Port<ControlEvent>;
type ControlChan = comm::Chan<ControlEvent>;

type PushChan = comm::Chan<int>;

// Registers with the manage task and gets notified when uptime changes.
// This is where things go south: we try to start three of these tasks, but
// only two start up (although after running multiple times it seems that
// there are some failues where all of these tasks start).
fn uptime_sse(registrar: StateChan, push: PushChan) -> ControlChan
{
    do task::spawn_listener
    |control_port: ControlPort|
    {
        io::println(fmt!("starting uptime sse stream"));
        let notify_port = comm::Port();
        let notify_chan = comm::Chan(notify_port);

        let key = fmt!("uptime %?", ptr::addr_of(notify_port));
        comm::send(registrar, AddListener(copy key, notify_chan));

        loop
        {
            match comm::select2(notify_port, control_port)
            {
                either::Left(new_time) =>
                {
                    info!("   new_time = %?", new_time);
                    comm::send(push, new_time);
                }
                either::Right(CloseEvent) =>
                {
                    info!("shutting down uptime sse stream");
                    comm::send(registrar, RemoveListener(key));
                    break;
                }
            }
        }
    }
}

// These are tasks that just sit around doing nothing. (I was not able to get a
// failure without these).
fn blocked_task() -> StateChan
{
    do task::spawn_listener
    |state_port: comm::Port<StateMesg>|
    {
        info!("starting blocked");
        loop
        {
            match state_port.recv()
            {
                Shutdown =>
                {
                    break;
                }
                _ =>
                {
                }
            }
        }
    }
}

// Number of  listener and blocked tasks to start up. May be able to reduce these values
// and still get a failure.
const num_listeners: uint = 3;
const num_blocked: uint = 12;

fn run_test() -> bool
{
    // Startup the task to manage the uptime variable.
    let registrar = manage_state();

    // Startup some tasks that block on recv until the very end of the test.
    // This corresponds to worker tasks in my app.
    info!("starting %? blocked tasks", num_blocked);
    let mut blocked = ~[];
    for num_blocked.times
    {
        let control = blocked_task();
        vec::push(blocked, control);
    }

    // Tasks that listen for changes in uptime. In my real (test) app it failed with
    //  one of these and it uses a server sent event to push the new uptime to a web
    // page.
    io::println(fmt!("starting %? uptime tasks", num_listeners));
    let mut listeners = ~[];
    for num_listeners.times
    {
        let push_port = comm::Port();
        let push_chan = comm::Chan(push_port);

        let control = uptime_sse(registrar, push_chan);
        vec::push(listeners, (push_port, control));
    }

    // Give the listeners plenty of time to get at least one update.
    // Note that this fails in the same way even if we sleep a lot longer.
    libc::funcs::posix88::unistd::sleep((5*num_listeners + 3) as libc::types::os::arch::c95::c_uint);

    // Print the uptimes received by each listener task.
    let mut times = ~[];
    for listeners.eachi
    |i, comms|
    {
        let (push_port, _control_chan) = comms;

        let mut last_time = 0;
        while (push_port.peek())
        {
            let time = push_port.recv();
            info!("   %? received = %?", i, time);
            if time > last_time
            {
                last_time = time;
            }
        }
        vec::push(times, last_time);
    }

    // Close down the tasks.
    for blocked.each
    |state_chan|
    {
        comm::send(state_chan, Shutdown)
    }

    for listeners.eachi
    |i, comms|
    {
        let (_push_port, control_chan) = comms;
        io::println(fmt!("%? time = %?", i, times[i]));
        comm::send(control_chan, CloseEvent)
    }

    comm::send(registrar, Shutdown);

    // Each listener should have received at least one update.
    do times.all |t| {t > 0}
}

// Only use a single unit test function to avoid interactions with running multiple unit tests
// in parallel (which could be an issue if this is dropped into the rust test suite).
#[test]
fn tester()
{
    // Ran this three times with the following results:
    // 5 out of 10 failed
    // 4 out of 10 failed
    // 7 out of 10 failed
    let num_tests = 10;
    let mut num_failures = 0;

    for num_tests.timesi
    |i|
    {
        io::println(fmt!("\nrunning %? of %?", i+1, num_tests));
        if !run_test()
        {
            num_failures += 1;
        }
    }

    if num_failures > 0
    {
        io::println(fmt!("%? tests out of %? failed", num_failures, num_tests));
        assert false;
    }
}
