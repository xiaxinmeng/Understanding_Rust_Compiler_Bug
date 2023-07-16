
// rustc --test multiple_tasks.rs && ./multiple_tasks
use std;

enum control_event
{
    dup_event(str, comm::chan<str>),
    close_event,
}

// This works with single_threaded and with manual_threads(4|16).
// Also tried thread_per_core and thread_per_task but they are not
// implemented.
fn silly2(n: uint) -> comm::chan<control_event>
{
    let channel_port = comm::port();
    let channel_channel = comm::chan(channel_port);

    do task::spawn_sched(task::manual_threads(4))
    {
        let control_port: comm::port<control_event> = comm::port();    // this is bookkeeping that spawn_listener handles for us
        let control_channel = comm::chan(control_port);
        comm::send(channel_channel, control_channel);

        io::println(#fmt["started task %?", n]);
        loop
        {
            libc::funcs::posix88::unistd::sleep(1);

            if comm::peek(control_port)
            {
                alt comm::recv(control_port)
                {
                    dup_event(data, data_ch)
                    {
                        io::println(#fmt["task %? received %?", n, data]);
                        comm::send(data_ch, data + data);
                    }
                    close_event
                    {
                        io::println(#fmt["exiting task %?", n]);
                        break;
                    }
                }
            }
        }
    };

    comm::recv(channel_port)
}

#[test]
fn blah()
{
    let po = comm::port();
    let ch = comm::chan(po);

    let silly_channels = do vec::from_fn(10) |n| {silly2(n)};

    for vec::each(silly_channels)
    |sc|
    {
        comm::send(sc, dup_event("hey", ch));
        assert comm::recv(po) == "heyhey";
    }

    for vec::each(silly_channels)
    |sc|
    {
        comm::send(sc, close_event);
    }
}
