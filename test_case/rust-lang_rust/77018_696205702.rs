rust
use crate::{
    elastic::{EsCmd, EsResult},
    immutable::{
        // lazy_static stuff, they are actix actors
        ES_EXECUTOR, REDIS_DB, REDIS_HOST, REDIS_PORT, RSMQ_EXECUTOR,
        SHARED_MESSAGE_QUEUE,
    },
    model::elastic::RsmqMsg,
    rsmq::RsmqCmd,
};

use backoff::{backoff::Backoff, ExponentialBackoff};
use log::error;
use rsmq_async::{Rsmq, RsmqMessage, RsmqOptions};
use tokio::{task::spawn, time::delay_for};

use std::time::Duration;

pub async fn start() {
    let mut backoff = ExponentialBackoff::default();
    backoff.max_elapsed_time = None;

    let opts = RsmqOptions {
        host: REDIS_HOST.to_owned(),
        port: REDIS_PORT.to_owned(),
        db: REDIS_DB.to_owned(),
        ..Default::default()
    };

    'outer: loop {
        match Rsmq::new(opts.clone()).await {
            Ok(mut rsmq) => {
                backoff.reset();

                loop {
                    match rsmq
                        .receive_message(
                            SHARED_MESSAGE_QUEUE.as_str(),
                            Some(90),
                        )
                        .await
                    {
                        Ok(Some(msg)) => {
                            spawn(handle_msg(msg));
                        }
                        Err(err) => {
                            error!("failed to receive message: {}", err);
                            break 'outer;
                        }
                        Ok(None) => {
                            delay_for(Duration::from_secs(3)).await;
                        }
                    }
                }
            }
            Err(err) => {
                error!("Cannot conect to redis server: {:?}", err);
                if let Some(timeout) = backoff.next_backoff() {
                    delay_for(timeout).await
                } else {
                    backoff.reset()
                }
            }
        }
    }
}

async fn handle_msg(msg: RsmqMessage) {
    let msg_id = msg.id.to_owned();
    match serde_json::from_str::<RsmqMsg>(&msg.message) {
        Ok(rsmq_msg) => {
            let res = match rsmq_msg {
                RsmqMsg::OperationLog(ref _operation_log) => {
                    ES_EXECUTOR
                        .send(EsCmd::Index("logs".to_owned(), rsmq_msg))
                        .await
                }
                RsmqMsg::CouponLog(ref _coupon_log) => {
                    ES_EXECUTOR
                        .send(EsCmd::Index("coupons".to_owned(), rsmq_msg))
                        .await
                }
            };

            match res {
                Ok(Ok(EsResult::Index)) => {
                    RSMQ_EXECUTOR.do_send(RsmqCmd::DeleteMessage(msg_id));
                }
                Ok(Err(err)) => error!("failed to index message: {}", err),
                _ => {}
            }
        }
        Err(err) => error!("failed to deserialize rsmq message: {}", err),
    }
}
