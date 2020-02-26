// Functions for implementing a BGWorker
#[macro_use]
extern crate pg_extend;

use pg_extend::{log::Level, pg_bgw, pg_sys};

use pg_extern_attr::{pg_bgw, pg_init};
use std::time::Duration;

// This tells Postges this library is a Postgres extension
pg_magic!(version: pg_sys::PG_VERSION_NUM);

//Nominate this function as the entry point for PostgreSQL
//It will be wrapped with guard_pg and called from a created _PG_init function
#[pg_init]
fn my_pg_init() {
    pg_bgw::BackgroundWorker::new("pg_extend_tester")
        .set_function("my_bgw_init")
        .set_library("libbgw")
        .load();
}

//Nominate this function as to be used as a BGWorker entrypoint
//It will be wrapped with a function which unblocks signals
#[pg_bgw]
fn my_bgw_init() {
    log!("Hello from inside the pg_extend BGWorker!");

    while pg_bgw::worker_continue() {
        log!("Hello from inside the pg_extend BGWorker loop!");
        pg_bgw::worker_wait(Duration::from_secs(10));
    }
}
