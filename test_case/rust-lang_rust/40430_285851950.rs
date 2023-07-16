
fn handle(request: &Request, response: Response) -> impl Future<…> {
   match … {
        Case::Case => sub_handle(request, response)
   }
}
fn sub_handle(request: &Request, response: Response) -> impl Future<…> {
    future.and_then(|x| {
         look_at_(request)
   })
}
