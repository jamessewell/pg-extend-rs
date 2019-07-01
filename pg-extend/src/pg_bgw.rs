// Functions for implementing a BGWorker
use crate::log::Level;
use crate::{error, pg_datum, pg_sys, pg_type, warn};

pub static mut prev_shmem_startup_hook: Option<unsafe extern "C" fn()> = None;

bitflags! {
    pub struct BGW_Flags: i32 {
        const BGWORKER_NONE = 0x00;
        const BGWORKER_SHMEM_ACCESS = 0x01;
        const BGWORKER_BACKEND_DATABASE_CONNECTION = 0x02;
    }
}

pub enum BgWorkerStartTime {
    PostmasterStart = 0,
    ConsistentState = 1,
    RecoveryFinished = 2
}

const BGW_NEVER_RESTART:i32 = -1;

pub struct BackgroundWorker {
    bgw_name: String,
    bgw_type: String,
    bgw_flags: BGW_Flags,
    bgw_start_time: BgWorkerStartTime,
    bgw_restart_time: i32,
    bgw_library_name: String,
    bgw_function_name: String,
    bgw_main_arg: pg_sys::Datum,
    bgw_extra: String,
    bgw_notify_pid: pg_sys::pid_t,
}

impl BackgroundWorker {
    pub fn new(name: &str) -> BackgroundWorker {
        BackgroundWorker {
            bgw_name: name.to_string(),
            bgw_type: name.to_string(),
            bgw_flags: BGW_Flags::BGWORKER_NONE,
            bgw_start_time: BgWorkerStartTime::PostmasterStart,
            bgw_restart_time: 10,
            bgw_library_name: name.to_string(),
            bgw_function_name: format!("bgw_{}", name),
            bgw_main_arg: 0,
            bgw_extra: "".to_string(),
            bgw_notify_pid: 0,
        }
    }

    pub fn set_type(mut self: Self, input: &str) -> Self {
        self.bgw_type = input.to_string();
        self
    }

    pub fn set_flags(mut self: Self, input: BGW_Flags) -> Self {
        self.bgw_flags = input;
        self
    }

    pub fn set_start_time(mut self: Self, input: BgWorkerStartTime) -> Self {
        self.bgw_start_time = input;
        self
    }

    pub fn set_restart_time(mut self: Self, input: i32) -> Self {
        self.bgw_restart_time = input;
        self
    }

    pub fn set_library(mut self: Self, input: &str) -> Self {
        self.bgw_library_name = input.to_string();
        self
    }

    pub fn set_function(mut self: Self, input: &str) -> Self {
        self.bgw_function_name = format!("bgw_{}", input);
        self
    }

    pub fn set_argument(mut self: Self, input: pg_sys::Datum) -> Self {
        self.bgw_main_arg = input;
        self
    }

    pub fn set_extra(mut self: Self, input: &str) -> Self {
        self.bgw_extra = input.to_string();
        self
    }

    pub fn set_notify_pid(mut self: Self, input: i32) -> Self {
        self.bgw_notify_pid = input;
        self
    }



    pub fn load(self: Self) {
        let mut bgw = pg_sys::BackgroundWorker {
            bgw_name: RpgffiChar96::from(&self.bgw_name[..]).0,
            bgw_type: RpgffiChar96::from(&self.bgw_type[..]).0,
            bgw_flags: self.bgw_flags.bits(),
            bgw_start_time: self.bgw_start_time as u32,
            bgw_restart_time: self.bgw_restart_time,
            bgw_library_name: RpgffiChar96::from(&self.bgw_library_name[..]).0,
            bgw_function_name: RpgffiChar96::from(&self.bgw_function_name[..]).0,
            bgw_main_arg: self.bgw_main_arg,
            bgw_extra: RpgffiChar128::from(&self.bgw_extra[..]).0,
            bgw_notify_pid: self.bgw_notify_pid,
        };

        crate::guard_pg(|| unsafe { pg_sys::RegisterBackgroundWorker(&mut bgw) });
    }
}

pub fn wait_latch(timeout: i64) -> i32 {
    unsafe {
        let latch = pg_sys::WaitLatch(
            pg_sys::MyLatch,
            pg_sys::WL_LATCH_SET as i32
                | pg_sys::WL_TIMEOUT as i32
                | pg_sys::WL_POSTMASTER_DEATH as i32,
            timeout,
            pg_sys::PG_WAIT_EXTENSION,
        );
        pg_sys::ResetLatch(pg_sys::MyLatch);
        latch
    }
}

struct RpgffiChar96([i8; 96]);

impl<'a> From<&'a str> for RpgffiChar96 {
    fn from(string: &str) -> Self {
        let mut r = [0; 96];
        r[..string.as_bytes().len()].copy_from_slice(string.as_bytes());
        RpgffiChar96(unsafe { std::mem::transmute::<[u8; 96], [i8; 96]>(r) })
    }
}

struct RpgffiChar128([i8; 128]);

impl<'a> From<&'a str> for RpgffiChar128 {
    fn from(string: &str) -> Self {
        let mut r = [0; 128];
        r[..string.as_bytes().len()].copy_from_slice(string.as_bytes());
        RpgffiChar128(unsafe { std::mem::transmute::<[u8; 128], [i8; 128]>(r) })
    }
}

#[no_mangle]
// This functions gets called by Postgres when the shared memory is ready
// // it happens in the master process once only.
pub extern "C" fn pgm_shmem_startup() {
    unsafe {
        if let Some(i) = prev_shmem_startup_hook {
            i();
        }
    }
}
