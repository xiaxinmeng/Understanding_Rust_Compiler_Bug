
#0  0x000000000002c456 in ?? ()
#1  0x00007f934f2b270b in rd_kafka_sasl_scram_conf_validate (rk=0x5555564dee60, errstr=0x7ffdcccc2a90 "",
    errstr_size=512)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-sys-4.3.0+1.9.2/librdkafka/src/rdkafka_sasl_scram.c:857
#2  0x00007f934f24b8a9 in rd_kafka_sasl_select_provider (rk=0x5555564dee60, errstr=0x7ffdcccc2a90 "", errstr_size=512)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-sys-4.3.0+1.9.2/librdkafka/src/rdkafka_sasl.c:438
#3  0x00007f934f1c0cb0 in rd_kafka_new (type=RD_KAFKA_CONSUMER, app_conf=0x5555564de1e0, errstr=0x7ffdcccc2a90 "",
    errstr_size=512)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-sys-4.3.0+1.9.2/librdkafka/src/rdkafka.c:2379
#4  0x00007f934f0dabbb in rdkafka::client::Client<wawa::CustomContext>::new<wawa::CustomContext> (
    config=0x7ffdcccc37b8, native_config=..., rd_kafka_type=rdkafka_sys::bindings::rd_kafka_type_t::RD_KAFKA_CONSUMER,
    context=...) at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-0.29.0/src/client.rs:250
#5  0x00007f934f0ccb6e in rdkafka::consumer::base_consumer::BaseConsumer<wawa::CustomContext>::new<wawa::CustomContext>
    (config=0x7ffdcccc37b8, native_config=..., context=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-0.29.0/src/consumer/base_consumer.rs:109
#6  0x00007f934f0d91d7 in rdkafka::consumer::stream_consumer::{impl#5}::from_config_and_context<wawa::CustomContext, rdkafka::util::TokioRuntime> (config=0x7ffdcccc37b8, context=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-0.29.0/src/consumer/stream_consumer.rs:200
#7  0x00007f934f0d1095 in rdkafka::config::ClientConfig::create_with_context<wawa::CustomContext, rdkafka::consumer::stream_consumer::StreamConsumer<wawa::CustomContext, rdkafka::util::TokioRuntime>> (self=0x7ffdcccc37b8, context=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-0.29.0/src/config.rs:262
#8  0x00007f934f0c7d1c in wawa::main::{async_block#0} (_task_context=0x7ffdcccc3988) at src/main.rs:20
#9  0x00007f934f0c4014 in tokio::runtime::park::{impl#4}::block_on::{closure#0}<wawa::main::{async_block_env#0}> ()
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/park.rs:283
#10 0x00007f934f0c39c6 in tokio::runtime::coop::with_budget<core::task::poll::Poll<()>, tokio::runtime::park::{impl#4}::block_on::{closure_env#0}<wawa::main::{async_block_env#0}>> (budget=..., f=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/coop.rs:107
--Type <RET> for more, q to quit, c to continue without paging--
#11 tokio::runtime::coop::budget<core::task::poll::Poll<()>, tokio::runtime::park::{impl#4}::block_on::{closure_env#0}<wawa::main::{async_block_env#0}>> (f=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/coop.rs:73
#12 tokio::runtime::park::CachedParkThread::block_on<wawa::main::{async_block_env#0}> (self=0x7ffdcccc3a90, f=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/park.rs:283
#13 0x00007f934f0d2a82 in tokio::runtime::context::BlockingRegionGuard::block_on<wawa::main::{async_block_env#0}> (
    self=0x7ffdcccc3ad0, f=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/context.rs:315
#14 0x00007f934f0c56de in tokio::runtime::scheduler::multi_thread::MultiThread::block_on<wawa::main::{async_block_env#0}> (self=0x7ffdcccc3bd0, handle=0x7ffdcccc3bf8, future=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/scheduler/multi_thread/mod.rs:66
#15 0x00007f934f0c3269 in tokio::runtime::runtime::Runtime::block_on<wawa::main::{async_block_env#0}> (
    self=0x7ffdcccc3bb8, future=...)
    at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.27.0/src/runtime/runtime.rs:304
#16 0x00007f934f0d26c0 in wawa::main () at src/main.rs:36
