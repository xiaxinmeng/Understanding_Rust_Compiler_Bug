patch
75,76c75,76
<     F: Fn(S, Request) -> MyResult,
<     S: Service + Clone,
---
>     F: Fn(Box<dyn Service>, Request) -> MyResult,2
>     S: Service + Clone + 'static,
87,88c87,88
<     F: Fn(S, Request) -> MyResult,
<     S: Service + Clone,
---
>     F: Fn(Box<dyn Service>, Request) -> MyResult,
>     S: Service + Clone + 'static,
91c91
<         (self.f)((*self.inner).clone(), request)
---
>         (self.f)(Box::new((*self.inner).clone()), request)
96c96
< pub fn stateless_log<O: Service + Clone>(mut next: O, request: Request) -> MyResult {
---
> pub fn stateless_log(mut next: Box<dyn Service>, request: Request) -> MyResult {
103c103
< pub fn stateless_double<O: Service + Clone>(mut next: O, request: Request) -> MyResult {
---
> pub fn stateless_double(mut next: Box<dyn Service>, request: Request) -> MyResult {
