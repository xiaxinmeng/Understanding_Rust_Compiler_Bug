
error[E0220]: associated type `Res` not found for `Self`
  --> $DIR/issue-59029-1.rs:5:46
   |
LL | trait MkSvc<Target, Req> = Svc<Target> where Self::Res: Svc<Req>;
   |                                              ^^^^^^^^^
   |                                              |
   |                                              associated type `Res` not found
   |                                              help: you might have meant to use the fully qualified path: `<Self as Svc<Target>>::Res`

