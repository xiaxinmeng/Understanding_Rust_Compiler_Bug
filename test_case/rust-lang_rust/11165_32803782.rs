 rust
pub struct Event
{
    event_type: EventKind, // The type of the event which occured and all necessary info
    is_valid: bool, // Internally used
    source: Rc<bool> // source of the event. Type is not relevant for the user
}

// All types of events that are known. This is what I used in the examples, but there would be much more
// Rusts enums are really a great way to describe different kinds of events and their associated data
pub enum EventKind
{
    StreamClosedEvent,
    IoErrorEvent(IoError),
    DataAvailableEvent(uint),
    TimerEvent,
    ChannelClosedEvent,
    ChannelMessageEvent,
    ...,
    SignalReceived(uint),
    DnsQueryResolved(),
    PacketReceived(uint)
}
