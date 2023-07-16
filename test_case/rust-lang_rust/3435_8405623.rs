
// Like spawn_listener except the new task (and whatever tasks it spawns) are distributed
// among a fixed number of OS threads.
fn spawn_threaded_listener<A:send>(num_threads: uint, +block: fn~ (comm::Port<A>)) -> comm::Chan<A>
{
    let channel_port: comm::Port<comm::Chan<A>> = comm::Port();
    let channel_channel = comm::Chan(channel_port);

    do task::spawn_sched(task::ManualThreads(num_threads))
    {
        let task_port: comm::Port<A> = comm::Port();
        let task_channel = comm::Chan(task_port);
        comm::send(channel_channel, task_channel);

        block(task_port);
    };

    comm::recv(channel_port)
}
