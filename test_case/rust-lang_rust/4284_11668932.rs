
extern mod std;

fn handle_connection(notify: pipes::Chan<()>)
{
    do task::spawn_sched(task::ManualThreads(2)) |move notify|
    {
        let (push_port, push_chan) = pipes::stream();
        let push_chan = pipes::SharedChan(push_chan);

        let mut tasks = ~[];
        loop
        {
            let exit_chan = pusher(push_chan.clone());
            vec::push(&mut tasks, exit_chan);

            // In the real code this happens in response to a message sent
            // as a result of a socket closing.
            if tasks.len() == 1
            {
                notify.send(());
                close_tasks(tasks);
                break;
            }

            push_port.recv();
            error!("received");
        }
    }
}

fn close_tasks(tasks: &[pipes::Chan<()>])
{
    for tasks.each |task|
    {
        task.send(());
    }
    // In the real code I get the failure unless I sleep for a bit here.
}

fn pusher(push_chan: pipes::SharedChan<~str>) -> pipes::Chan<()>
{
    let (exit_port, exit_chan) = pipes::stream();
    let push_chan = push_chan.clone();
    do task::spawn_sched(task::ThreadPerCore) |move push_chan, move exit_port|
    {
        while !exit_port.peek()
        {
            error!("sending");
            push_chan.send(~"hmm");
            libc::funcs::posix88::unistd::sleep(1);
        }
    }
    exit_chan
}

fn main()
{
    let (port, chan) = pipes::stream();
    handle_connection(chan);
    port.recv();
}
