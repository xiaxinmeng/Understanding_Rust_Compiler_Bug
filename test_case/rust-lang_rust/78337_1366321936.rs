
error[E0277]: `<S as RaftStorage<D, R>>::Snapshot` cannot be shared between threads safely
   --> async-raft/src/core/mod.rs:136:22
    |
136 |         tokio::spawn(this.main())
    |         ------------ ^^^^^^^^^^^ `<S as RaftStorage<D, R>>::Snapshot` cannot be shared between threads safely
    |         |
    |         required by a bound introduced by this call
    |
    = help: the trait `Sync` is not implemented for `<S as RaftStorage<D, R>>::Snapshot`
    = note: required for `Unique<<S as RaftStorage<D, R>>::Snapshot>` to implement `Sync`
    = note: required because it appears within the type `Box<<S as RaftStorage<D, R>>::Snapshot>`
note: required because it appears within the type `SnapshotState<<S as RaftStorage<D, R>>::Snapshot>`
   --> async-raft/src/core/mod.rs:413:16
    |
413 | pub(self) enum SnapshotState<S> {
    |                ^^^^^^^^^^^^^
    = note: required because it appears within the type `std::option::Option<SnapshotState<<S as RaftStorage<D, R>>::Snapshot>>`
note: required because it appears within the type `RaftCore<D, R, N, S>`
   --> async-raft/src/core/mod.rs:32:12
    |
32  | pub struct RaftCore<D: AppData, R: AppDataResponse, N: RaftNetwork<D>, S: RaftStorage<D, R>> {
    |            ^^^^^^^^
    = note: required for `&RaftCore<D, R, N, S>` to implement `std::marker::Send`
    = note: required because it captures the following types: `ResumeTy`, `bool`, `u64`, `RaftCore<D, R, N, S>`, `&mut RaftCore<D, R, N, S>`, `&S`, `Arc<S>`, `&RaftCore<D, R, N, S>`, `Pin<Box<dyn futures::Future<Output = Result<Vec<raft::Entry<D>>, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<Vec<raft::Entry<D>>, anyhow::Error>> + std::marker::Send>>`, `()`, `Vec<raft::Entry<D>>`, `Vec<(&u64, &D)>`, `&[(&u64, &D)]`, `&Vec<(&u64, &D)>`, `Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + std::marker::Send>>`
note: required because it's used within this `async` block
   --> async-raft/src/core/append_entries.rs:202:5
    |
202 |     #[tracing::instrument(level = "trace", skip(self, report_metrics))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required because it captures the following types: `ResumeTy`, `&mut RaftCore<D, R, N, S>`, `&mut bool`, `Span`, `[async block@async-raft/src/core/append_entries.rs:202:5: 202:72]`, `bool`, `tracing::instrument::Instrumented<[async block@async-raft/src/core/append_entries.rs:202:5: 202:72]>`, `()`
note: required because it's used within this `async fn` body
   --> async-raft/src/core/append_entries.rs:202:5
    |
202 |     #[tracing::instrument(level = "trace", skip(self, report_metrics))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required because it captures the following types: `ResumeTy`, `bool`, `&mut RaftCore<D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`, `()`, `&mut bool`, `impl futures::Future<Output = Result<(), RaftError>>`, `AppendEntriesRequest<D>`, `Vec<raft::Entry<D>>`, `&[raft::Entry<D>]`, `&Vec<raft::Entry<D>>`, `impl futures::Future<Output = Result<(), RaftError>>`, `RaftCore<D, R, N, S>`, `&S`, `Arc<S>`, `u64`, `Pin<Box<dyn futures::Future<Output = Result<Vec<raft::Entry<D>>, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<Vec<raft::Entry<D>>, anyhow::Error>> + std::marker::Send>>`, `&raft::Entry<D>`, `raft::Entry<D>`, `std::option::Option<u64>`, `Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<MembershipConfig, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<MembershipConfig, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<Vec<raft::Entry<D>>, anyhow::Error>> + std::marker::Send>>`
note: required because it's used within this `async` block
   --> async-raft/src/core/append_entries.rs:10:5
    |
10  | /     #[tracing::instrument(
11  | |         level="trace", skip(self, msg),
12  | |         fields(term=msg.term, leader_id=msg.leader_id, prev_log_index=msg.prev_log_index, prev_log_term=msg.prev_log...
13  | |     )]
    | |______^
    = note: required because it captures the following types: `ResumeTy`, `&mut RaftCore<D, R, N, S>`, `AppendEntriesRequest<D>`, `Span`, `[async block@async-raft/src/core/append_entries.rs:10:5: 13:7]`, `bool`, `tracing::instrument::Instrumented<[async block@async-raft/src/core/append_entries.rs:10:5: 13:7]>`, `()`
note: required because it's used within this `async fn` body
   --> async-raft/src/core/append_entries.rs:10:5
    |
10  | /     #[tracing::instrument(
11  | |         level="trace", skip(self, msg),
12  | |         fields(term=msg.term, leader_id=msg.leader_id, prev_log_index=msg.prev_log_index, prev_log_term=msg.prev_log...
13  | |     )]
    | |______^
    = note: required because it captures the following types: `ResumeTy`, `Vec<u64>`, `&mut LeaderState<'_, D, R, N, S>`, `LeaderState<'_, D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`, `impl futures::Future<Output = Result<(), RaftError>>`, `()`, `u8`, `(tokio::stream::next::Next<'_, tokio::sync::mpsc::UnboundedReceiver<RaftMsg<D, R>>>, tokio::stream::next::Next<'_, tokio::sync::mpsc::Receiver<SnapshotUpdate>>, tokio::stream::next::Next<'_, FuturesOrdered<tokio::sync::oneshot::Receiver<Result<u64, RaftError>>>>, tokio::stream::next::Next<'_, FuturesOrdered<tokio::sync::oneshot::Receiver<Result<u64, RaftError>>>>, tokio::stream::next::Next<'_, tokio::sync::mpsc::UnboundedReceiver<ReplicaEvent<<S as RaftStorage<D, R>>::Snapshot>>>)`, `[closure@/Users/ekuber/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.25/src/macros/select.rs:351:46: 351:50]`, `tokio::future::poll_fn::PollFn<[closure@/Users/ekuber/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.25/src/macros/select.rs:351:46: 351:50]>`, `LeaderState<'a, D, R, N, S>::run::{closure#0}::{closure#0}::util::Out<std::option::Option<RaftMsg<D, R>>, std::option::Option<SnapshotUpdate>, std::option::Option<Result<Result<u64, RaftError>, tokio::sync::oneshot::error::RecvError>>, std::option::Option<Result<Result<u64, RaftError>, tokio::sync::oneshot::error::RecvError>>, std::option::Option<ReplicaEvent<<S as RaftStorage<D, R>>::Snapshot>>>`, `RaftMsg<D, R>`, `AppendEntriesRequest<D>`, `tokio::sync::oneshot::Sender<Result<AppendEntriesResponse, RaftError>>`, `&mut RaftCore<D, R, N, S>`, `impl futures::Future<Output = Result<AppendEntriesResponse, RaftError>>`, `VoteRequest`, `tokio::sync::oneshot::Sender<Result<VoteResponse, RaftError>>`, `impl futures::Future<Output = Result<VoteResponse, RaftError>>`, `InstallSnapshotRequest`, `tokio::sync::oneshot::Sender<Result<InstallSnapshotResponse, RaftError>>`, `impl futures::Future<Output = Result<InstallSnapshotResponse, RaftError>>`, `tokio::sync::oneshot::Sender<Result<(), ClientReadError>>`, `impl futures::Future<Output = ()>`, `impl futures::Future<Output = ()>`, `ClientWriteRequest<D>`, `tokio::sync::oneshot::Sender<Result<ClientWriteResponse<R>, ClientWriteError<D>>>`, `impl futures::Future<Output = ()>`, `impl futures::Future<Output = ()>`, `HashSet<u64>`, `tokio::sync::oneshot::Sender<Result<(), ChangeConfigError>>`, `impl futures::Future<Output = ()>`, `impl futures::Future<Output = ()>`, `Result<u64, RaftError>`, `impl futures::Future<Output = Result<(), RaftError>>`, `impl futures::Future<Output = Result<(), RaftError>>`, `u64`, `impl futures::Future<Output = Result<(), RaftError>>`, `impl futures::Future<Output = Result<(), RaftError>>`, `ReplicaEvent<<S as RaftStorage<D, R>>::Snapshot>`, `impl futures::Future<Output = ()>`, `impl futures::Future<Output = ()>`
note: required because it's used within this `async` block
   --> async-raft/src/core/mod.rs:554:5
    |
554 |     #[tracing::instrument(level="trace", skip(self), fields(id=self.core.id, raft_state="leader"))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required because it captures the following types: `ResumeTy`, `LeaderState<'_, D, R, N, S>`, `Span`, `[async block@async-raft/src/core/mod.rs:554:5: 554:100]`, `bool`, `tracing::instrument::Instrumented<[async block@async-raft/src/core/mod.rs:554:5: 554:100]>`, `()`
note: required because it's used within this `async fn` body
   --> async-raft/src/core/mod.rs:554:5
    |
554 |     #[tracing::instrument(level="trace", skip(self), fields(id=self.core.id, raft_state="leader"))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required because it captures the following types: `ResumeTy`, `RaftCore<D, R, N, S>`, `&S`, `Arc<S>`, `Pin<Box<dyn futures::Future<Output = Result<InitialState, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<InitialState, anyhow::Error>> + std::marker::Send>>`, `()`, `InitialState`, `Pin<Box<dyn futures::Future<Output = Result<std::option::Option<CurrentSnapshotData<<S as RaftStorage<D, R>>::Snapshot>>, anyhow::Error>> + std::marker::Send>>`, `Pin<Box<dyn futures::Future<Output = Result<std::option::Option<CurrentSnapshotData<<S as RaftStorage<D, R>>::Snapshot>>, anyhow::Error>> + std::marker::Send>>`, `bool`, `State`, `&State`, `&mut RaftCore<D, R, N, S>`, `&mut RaftCore<D, R, N, S>`, `LeaderState<'_, D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`, `&mut RaftCore<D, R, N, S>`, `CandidateState<'_, D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`, `&mut RaftCore<D, R, N, S>`, `FollowerState<'_, D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`, `&mut RaftCore<D, R, N, S>`, `NonVoterState<'_, D, R, N, S>`, `impl futures::Future<Output = Result<(), RaftError>>`
note: required because it's used within this `async` block
   --> async-raft/src/core/mod.rs:140:5
    |
140 |     #[tracing::instrument(level="trace", skip(self), fields(id=self.id, cluster=%self.config.cluster_name))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: required because it captures the following types: `ResumeTy`, `RaftCore<D, R, N, S>`, `Span`, `[async block@async-raft/src/core/mod.rs:140:5: 140:109]`, `bool`, `tracing::instrument::Instrumented<[async block@async-raft/src/core/mod.rs:140:5: 140:109]>`, `()`
note: required because it's used within this `async fn` body
   --> async-raft/src/core/mod.rs:140:5
    |
140 |     #[tracing::instrument(level="trace", skip(self), fields(id=self.id, cluster=%self.config.cluster_name))]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `tokio::spawn`
   --> /Users/ekuber/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.25/src/task/spawn.rs:127:21
    |
127 |         T: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
    = note: this error originates in the attribute macro `tracing::instrument` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting the associated type
    |
109 |     ) -> JoinHandle<RaftResult<()>> where <S as RaftStorage<D, R>>::Snapshot: Sync {
    |                                     ++++++++++++++++++++++++++++++++++++++++++++++
