
#[allow(non_implicitly_copyable_typarams)];
use core::send_map::linear::{LinearMap};

extern mod std;

// ---- application code ----------------------------------
fn main()
{
    let state_chan = manage_state();
    let state_chan = pipes::SharedChan(state_chan);

    let mut openers = LinearMap();
    let opener: Opener = |_config, push| {manage_connection(state_chan.clone(), push)};
    openers.insert(~"/uptime", opener);

    run_server(Config {openers: openers});

    libc::funcs::posix88::unistd::sleep(3);
    libc::exit(0);
}

enum StateMesg
{
    AddListener(~str, pipes::Chan<int>),
}

type StatePort = pipes::Port<StateMesg>;
type StateChan = pipes::Chan<StateMesg>;

// This task might be used by a server to manage global state. Incoming
// connections register themselves using AddListener so that they can be 
// notified of state changes. In this case our state is very simple: the number 
// of seconds the server has been up.
fn manage_state() -> StateChan
{
    let (state_port, state_chan): (StatePort, StateChan) = pipes::stream();
    do task::spawn_sched(task::ManualThreads(1)) |move state_port|
    {
        let mut time = 0;
        let mut listeners = LinearMap();
        loop
        {
            time += 1;
            libc::funcs::posix88::unistd::sleep(1);
            for listeners.each_value |ch: &pipes::Chan<int>| {ch.send(time)};

            match state_port.recv()
            {
                AddListener(key, ch) =>
                {
                    listeners.insert(key, ch);
                }
            }
        }
    }
    state_chan
}

// Called for each connection.
fn manage_connection(state_chan: pipes::SharedChan<StateMesg>, push: PushChan)
{
    do task::spawn_sched(task::ManualThreads(1)) |move state_chan, move push|
    {
        let (notify_port, notify_chan) = pipes::stream();
        state_chan.send(AddListener(~"something unique", notify_chan));

        loop
        {
            let secs = notify_port.recv();
            error!("pushing %?", secs);
            push.send(secs.to_str());
        }
    }
}

// ---- server code ---------------------------------------

pub type PushChan = pipes::SharedChan<~str>;
pub type Opener = fn~ (config: Config, push: PushChan);

pub struct Config
{
    // Real code would have a lot more here, including stuff
    // which might be useful to application connections.
    pub openers: LinearMap<~str, Opener>,
}

// This would correspond to some sort of server library entry point.
// Because it is a library it needs to deal with abstract types like
// closures. To keep things simple we don't handle connections closing 
// or cleanly shutting down tasks.
fn run_server(config: Config)
{
    do task::spawn_sched(task::ManualThreads(1)) |move config|
    {
        let (push_port, push_chan) = pipes::stream();
        let push_chan = pipes::SharedChan(push_chan);

        create_connection(&config, &~"/uptime", push_chan);

        loop
        {
            let data = push_port.recv();
            error!("received %s", data);
        }
    }
}

// Called for each connection. Spawns an application task to manage
// the connection.
fn create_connection(config: &Config, name: &~str, push: PushChan)
{
    match config.openers.find_ref(name)
    {
        Some(opener) =>
        {
            // There can be multiple connection tasks so we need to
            // either copy config or share it:
            //
            // 1) Copying the config sounds reasonable and is easy,
            // but copying unique closures requires copying what 
            // they capture which is very dangerous here (and not
            // caught by the compiler atm) because SharedChans
            // are clonable but not copyable.
            //
            // 2) ARC is the next obvious solution. But Config is not
            // Const so that is out.
            //
            // 3) MutexARC relaxes the Const requirement but it's
            // a bit cumbersome to use and only provides mutable
            // access which means we can't get a reference to values
            // in the map without using unsafe code. It does expose
            // the MutexARC state, but it doesn't expose the types
            // and functions used by access so we can't write a const
            // version.
            (*opener)(copy *config, push);
        }
        None =>
        {
            fail fmt!("Couldn't find %s", *name);
        }
    }
}
