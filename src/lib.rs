#![warn(
    clippy::print_stdout,
    clippy::use_debug,
    clippy::dbg_macro,
    clippy::print_stderr
)]
use tokio::sync::oneshot;

pub(crate) type Responder<T> = oneshot::Sender<T>;

pub mod message;
pub mod realtime_channel;
pub mod realtime_client;
pub mod realtime_presence;
