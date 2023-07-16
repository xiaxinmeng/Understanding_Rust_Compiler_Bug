
trait Selectable {...}

impl<T:Selectable> Selectable for ~T {...}

struct SelectorHandle<T> {...}

struct SelectorEvent<'a, T, D> {
port: &'a mut T,
custom_data: D,
events: EventMask
}

impl Selector<T: Selectable, D>
{
fn add(&mut self, port: T, event_mask: EventMask, custom_data: D) -> SelectorHandle<T> {...}
fn modify(&mut self, handle: SelectorHandle<T>, event_mask: EventMask) {...}
fn remove(&mut self, handle: SelectorHandle<T>) -> T {...}
fn get<'a>(&'a mut self, handle: &'a SelectorHandle<T>) -> &'a mut T {...}
fn signaled<'a>(&'a mut self, timeout: Option<TimeSpan>) -> Iterator<SelectorEvent<'a, T, D>> {...}
}

impl<T, D> Selectable for Selector<T, D>

// to select on multiple different T types, use Selector<~Selectable, D>
