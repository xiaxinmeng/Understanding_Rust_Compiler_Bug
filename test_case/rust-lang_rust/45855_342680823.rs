
index 7e15671..90c3622 100644
--- a/src/body_parser.rs
+++ b/src/body_parser.rs
@@ -76,7 +76,7 @@ pub trait FormBody {
 }
 
 #[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
-impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> FormBody for Request<'mw, B, D> {
+impl<'mw, D> FormBody for Request<'mw, Body, D> {
     fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)> {
         self.get_ref::<FormBodyParser>().map_err(|e| (StatusCode::BadRequest, e))
     }
@@ -88,7 +88,7 @@ pub trait JsonBody {
 }
 
 #[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
-impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> JsonBody for Request<'mw, B, D> {
+impl<'mw, D> JsonBody for Request<'mw, Body, D> {
     // FIXME: Update the error type.
     // Would be good to capture parsing error rather than a generic io::Error.
     // FIXME: Do the content-type check
