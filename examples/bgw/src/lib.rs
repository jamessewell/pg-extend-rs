// Functions for implementing a BGWorker
#[macro_use]
extern crate pg_extend;

use pg_extend::log::Level;
use pg_extend::pg_bgw::BackgroundWorker;
use pg_extend::{error, guard_pg, pg_datum, pg_sys, pg_type, warn};
use pg_extern_attr::{pg_bgw, pg_init};

// This tells Postges this library is a Postgres extension
pg_magic!(version: pg_sys::PG_VERSION_NUM);

#[pg_init]
fn my_pg_init() {
    BackgroundWorker::new("pg_extend_tester")
        .set_function("my_bgw_init")
        .set_library("libbgw")
        .load();
}

#[pg_bgw]
fn my_bgw_init() {
    pg_log!(
        Level::LogServerOnly,
        "Hello from inside the pg_extend BGWorker!"
    );
}
