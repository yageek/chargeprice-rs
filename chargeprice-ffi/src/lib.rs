use lazy_static::lazy_static;
pub mod client;
pub mod models;
pub use models::*;

lazy_static! {
    //runtime with threaded pool
    static ref RUN_TIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .thread_name("chageprice-rs runtime")
    .build()
    .unwrap();

}
