rust

 pub trait Owned<'a> {
    type Reader: FromPointerReader<'a>;
}
pub trait FromPointerReader<'a> : Sized {
    fn get_from_pointer(reader: &'a[u8]) -> Self;
}

pub mod messaging {
    pub mod reactor {
        use std::collections::HashMap;
        use super::types::{Handler, AnyPtrHandler};
        use crate::any_pointer;
        use crate::Owned;

        pub trait Ctx : 'static + for<'a> Context<'a> {}
        impl<C> Ctx for C where C: 'static + for<'a> Context<'a> {}

        pub trait Context<'a> : Sized {
            type Handle: 'a + CtxHandle<Self>;
        }

        pub trait CtxHandle<C> {
            fn spawn<S>(&mut self, params: CoreParams<S, C>)
                where S: 'static + Send,
                      C: Ctx;
        }

        pub struct Reactor<S, C: Ctx> {
            pub internal_state: S,
            pub internal_handlers: HashMap<u64, CoreHandler<S, C>>,
        }

        pub struct LinkReducer<S, C>
            where C: Ctx
        {
            pub state: S,
            pub internal_handlers: LinkHandlers<S, C>,
        }

        pub struct ReactorHandle<'a, 'c: 'a, C: Ctx> {
            _ctx: &'a mut <C as Context<'c>>::Handle,
        }

        impl<'a, 'c, C: Ctx> ReactorHandle<'a, 'c, C> {
            pub fn open_link<S>(&mut self, _params: LinkParams<S, C>)
                where S: 'static + Send
            {
                todo!()
            }
        }

        pub struct LinkHandle<'a, 'c: 'a, C: Ctx> {
            _ctx: &'a mut <C as Context<'c>>::Handle,
        }

        pub struct HandlerCtx<'a, S, H> {
            state: &'a mut S,
            handle: &'a mut H,
        }

        impl<'a, S, H> HandlerCtx<'a, S, H> {
            pub fn state<'b>(&'b mut self) -> &'b mut S {
                &mut self.state
            }

            pub fn handle<'b>(&'b mut self) -> &'b mut H {
                &mut self.handle
            }
        }

        use std::marker::PhantomData;

        pub struct CtxHandler<M, F> {
            message_type: PhantomData<M>,
            function: F,
        }

        impl<M, F> CtxHandler<M, F> {
            pub fn new(function: F) -> Self {
                CtxHandler {
                    message_type: PhantomData,
                    function,
                }
            }
        }

        impl<'a, S, H, M, F> Handler<'a, HandlerCtx<'a, S, H>, M> for CtxHandler<M, F>
            where F: Fn(&mut S, &mut H, <M as Owned<'a>>::Reader),
                  F: Send,
                  M: Owned<'a> + 'static + Send
        {
            fn handle(&self, ctx: &mut HandlerCtx<'a, S, H>, reader: <M as Owned<'a>>::Reader)
            {
                (self.function)(&mut ctx.state, &mut ctx.handle, reader);
            }
        }

        pub struct CtxHandlerWithoutState<M, F> {
            message_type: PhantomData<M>,
            _function: F,
        }

        impl<M, F> CtxHandlerWithoutState<M, F> {
            pub fn new(_function: F) -> Self { todo!() }
        }

        impl<'a, S, H, M, F> Handler<'a, HandlerCtx<'a, S, H>, M> for CtxHandlerWithoutState<M, F>
            where F: Fn(&mut H, <M as Owned<'a>>::Reader),
                  F: Send,
                  M: Owned<'a> + 'static + Send
        {
            fn handle(&self, _ctx: &mut HandlerCtx<'a, S, H>, _reader: <M as Owned<'a>>::Reader)
            {
                todo!()
            }
        }

        pub type ReactorCtx<'a, 'c, S, C> = HandlerCtx<'a, S, ReactorHandle<'a, 'c, C>>;
        pub type LinkCtx<'a, 'c, S, C> = HandlerCtx<'a, S, LinkHandle<'a, 'c, C>>;

        type CoreHandler<S, C> = Box<
                dyn for <'a, 'c>
                Handler<
                        'a,
                    HandlerCtx<'a, S, ReactorHandle<'a, 'c, C>>,
                    any_pointer::Owned,
                        >
                >;

        type LinkHandler<S, C> = Box<
                dyn for<'a, 'c>
                Handler<'a,
                        HandlerCtx<'a, S, LinkHandle<'a, 'c, C>>,
                        any_pointer::Owned,
                        >
                >;

        type LinkHandlers<S, C> = HashMap<u64, LinkHandler<S, C>>;

        pub struct CoreParams<S, C: Ctx> {
            pub state: S,
            pub handlers: HashMap<u64, CoreHandler<S, C>>,
        }

        impl<S, C: Ctx> CoreParams<S, C> {
            pub fn new(_state: S) -> Self {
                todo!()
            }

            pub fn handler<M, H>(&mut self, _m: M, h: H)
                where M: for<'a> Owned<'a> + Send + 'static,
                      H: 'static + for <'a, 'c> Handler<'a, HandlerCtx<'a, S, ReactorHandle<'a, 'c, C>>, M>,
            {
                let boxed = Box::new(AnyPtrHandler::new(h));
                self.handlers.insert(
                    0,
                    boxed,
                );
            }
        }

        pub struct LinkParams<S, C: Ctx> {
            pub state: S,
            pub external_handlers: LinkHandlers<S, C>,
        }

        impl<S, C: Ctx> LinkParams<S, C> {
            pub fn new(_state: S) -> Self {
                todo!()
            }

            pub fn external_handler<M, H>(&mut self, _m: M, h: H)
                where M: for<'a> Owned<'a> + Send + 'static,
                      H: 'static + for <'a, 'c> Handler<'a, HandlerCtx<'a, S, LinkHandle<'a, 'c, C>>, M>
            {
                let boxed = Box::new(AnyPtrHandler::new(h));
                self.external_handlers.insert(
                    0,
                    boxed,
                );
            }

            pub fn send_external_to_internal<M>(&mut self, m: M,)
                where M: for<'a> Owned<'a> + Send + 'static,
            {
                self.external_handler(m, CtxHandlerWithoutState::new(e_to_i::<C, M>));
            }
        }

        fn e_to_i<C:Ctx, M>(_handle: &mut LinkHandle<C>, _msg: <M as Owned<'_>>::Reader)
            where
            M: for<'a> Owned<'a> + Send + 'static,
        {
            todo!()
        }
    }

    pub mod types {
        use std::marker::PhantomData;
        use crate::any_pointer;
        use crate::Owned;

        pub trait Handler<'a, S, M>: Send
            where M: Owned<'a>
        {
            fn handle(&self, state: &mut S, reader: <M as Owned<'a>>::Reader);
        }

        pub struct FnHandler<M, F> {
            message_type: PhantomData<M>,
            _function: F,
        }

        impl<M, F> FnHandler<M, F> {
            pub fn new(function: F) -> Self {
                FnHandler {
                    _function: function,
                    message_type: PhantomData,
                }
            }
        }

        impl<'a, S, M, F, T, E> Handler<'a, S, M> for FnHandler<M, F>
            where F: Fn(&mut S, <M as Owned<'a>>::Reader) -> Result<T, E>,
                  F: Send,
                  M: Owned<'a> + 'static + Send
        {
            fn handle(&self, _state: &mut S, _reader: <M as Owned<'a>>::Reader)
            {}
        }

        pub struct AnyPtrHandler<H, M> {
            message_type: PhantomData<M>,
            handler: H,
        }

        impl<H, M> AnyPtrHandler<H, M> {
            pub fn new(handler: H) -> Self {
                AnyPtrHandler {
                    handler,
                    message_type: PhantomData,
                }
            }
        }

        impl<'a, S, M, H> Handler<'a, S, any_pointer::Owned> for AnyPtrHandler<H, M>
            where H: Handler<'a, S, M>,
                  M: Send + Owned<'a>
        {
            fn handle(&self, state: &mut S, reader: any_pointer::Reader<'a>)
            {
                let m = reader.get_as();
                return self.handler.handle(state, m);
            }
        }
    }
}

pub mod runtime {
    use crate::messaging::reactor::{CtxHandle, CoreParams, Context};
    pub struct Runtime;

    pub fn spawn<S>(_core_params: CoreParams<S, Runtime>)
        where S: 'static + Send
    {
        todo!()
    }

    impl<'a> Context<'a> for Runtime {
        type Handle = DriverHandle<'a>;
    }

    pub struct DriverHandle<'a> {
        _broker: &'a (),
    }

    impl<'a> CtxHandle<Runtime> for DriverHandle<'a> {
        fn spawn<T>(&mut self, _params: CoreParams<T, Runtime>)
            where T: 'static + Send
        {
            todo!()
        }
    }
}

pub mod aggregator {
    use crate::messaging::reactor::{LinkParams, Ctx, ReactorHandle, CoreParams, CtxHandler};
    use crate::any_pointer;
    pub struct Aggregator;

    impl Aggregator {
        pub fn params<C: Ctx>() -> CoreParams<Self, C> {
            let me = Self;
            let mut params = CoreParams::new(me);
            params.handler(any_pointer::Owned, CtxHandler::new(Self::handle_initialize));
            return params;
        }

        fn handle_initialize<C: Ctx>(
            &mut self,
            handle: &mut ReactorHandle<C>,
            _: any_pointer::Reader,
        )
        {
            handle.open_link(HostLink::params());
        }
    }

    struct HostLink;
    impl HostLink {
        fn params<C: Ctx>() -> LinkParams<Self, C> {
            let mut params = LinkParams::new(HostLink);
            params.send_external_to_internal(any_pointer::Owned);
            return params;
        }
    }
}

pub mod any_pointer {
    pub struct Owned;

    impl <'a> crate::Owned<'a> for Owned {
        type Reader = Reader<'a>;
    }

    pub struct Reader<'a> {
        _reader: &'a[u8],
    }

    impl <'a> Reader<'a> {
        pub fn get_as<T: crate::FromPointerReader<'a>>(&self) -> T {
            todo!()
        }
    }

    impl <'a,> crate::FromPointerReader<'a> for Reader<'a,>  {
        fn get_from_pointer(_reader: &'a[u8]) -> Reader<'a,> {
            todo!()
        }
    }
}

pub fn foo() {
    crate::runtime::spawn(crate::aggregator::Aggregator::params());
}
