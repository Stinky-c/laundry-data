pub(crate) mod cancel {
    use tokio::sync::oneshot;
    // pub(crate) use tokio::sync::oneshot::error::TryRecvError;
    use tracing::{error, instrument};

    pub(crate) type CancelReceiver = oneshot::Receiver<()>;
    pub(crate) type CancelSender = oneshot::Sender<()>;

    /*
       match cancel.try_recv() {
           Ok(_) => break,
           Err(TryRecvError::Closed) => break,
           Err(TryRecvError::Empty) => (),
       }
    */

    pub(crate) fn cancel_channel() -> (CancelSender, CancelReceiver) {
        oneshot::channel()
    }

    #[instrument]
    pub(crate) fn vec_cancel(channels: Vec<CancelSender>) {
        for channel in channels {
            if channel.send(()).is_err() {
                error!("Failed to cancel a task. Task already closed.");
            }
        }
    }
}
