rust
fn ingress_step<'net>(
    network: &mut Network<'net>,
    queues: &mut …,
    …
) {
    match network {
        Setup { target, with_foo_service, … } => {
            // some setup io..
            if let Some(foo) = &with_foo_service {
                foo.ingress(queues);
            }

            // Done with setup?
            if !with_foo_service.as_ref().map_or(true, FindFoo::is_done) {
                return;
            }
            
            // Switch to active service mode.
            network = Network::Running {
                net: RefMut::leak(target.borrow_mut()), …
            };
        },
        Running { net } => route_packets(net, queues),
    }
}
