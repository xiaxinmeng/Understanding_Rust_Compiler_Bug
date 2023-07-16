rust
        impl Handler<$name> for $handler {
            type Result = Addr<$result_inner>;

            #[allow(unused_variables)]
            fn handle(&mut self, msg: $name, ctx: &mut Context<Self>) -> Self::Result {
                let $name { $($arg,)* } = msg;
                let fun: fn (&mut Self, $($arg_ty,)* &mut Context<Self>) -> Self::Result = $code;
                fun(self, $($arg,)* ctx)
            }
        }
        get_message!(CreateLight, Light: LightImpl, index: u8, name: String,
             -> Lighting, create_light,
             -> LightingImpl => {
                 |this, index, name, ctx|{
                    let addr = this
                        .lights
                        .entry(index)
                        .or_insert(LightImpl::new(ctx.address(), index));
                    (*addr).clone()
                 }
             });

