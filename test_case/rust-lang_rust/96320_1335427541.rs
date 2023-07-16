
#[async_trait]
trait RequestHandler {
    async fn start(&mut self, ...);
    ...
}

trait Request: Sized {
    type ReplyType: Reply
    type Handler: From<Message<Self> + RequestHandler;
    fn foo(&mut self);
}

trait Reply  { ... }

struct Message<T: Request> {
    pub request: Arc<T>,
    pub replier: tokio::sync::mpsc::Sender<Result<T::ReplyType, Status>>,
}

struct MyHandler<T: Request> {
    message: Message<T>,
}

impl<T: Request + Send + Sync> From<Message<T>> for MyHandler<T> {
    fn from(msg: Message<T>) -> Self { Self { message: msg } }
}

#[async_trait]
impl <T> RequestHandler for MyHandler<T>
    where T: Request + Send + Sync,
          <T as Request>::ReplyType: Send
{
    async fn start(&mut self, ...) {...}
}

struct Inner {
    current_task: Option<Box<dyn RequestHandler + Send>>
}

impl Inner {
    async fn handle_message<T: Request>(&mut self, msg: &Message<T>) {
       if let Some(_) = &self.current_task {
            msg.replier.send(Err(Status::Busy)).await; return;
       }

        let task = Box::new(T::Handler::from(msg.clone()));
        task.start(...).await;
        self.current_task = Some(task);
    }
}
