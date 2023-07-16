rust
ConcreteReceiver = (typeof self) where Self: Sized;
DynReceiver = (typeof self)[dyn Trait/Self];
ConcreteReceiver: CoerceUnsizedDispatchable<DynReceiver>
