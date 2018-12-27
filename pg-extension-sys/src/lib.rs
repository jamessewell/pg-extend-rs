
#[cfg(feature = "pg_v10")]
pub mod pg_bool;

pub mod pg_datum;
pub mod pg_sys;
pub mod pg_error;

use std::os::raw::c_uint;

#[macro_export]
macro_rules! pg_magic {
    (version: $vers:expr) => {
        #[allow(non_upper_case_globals)]
        static mut Pg_magic_data: pg_sys::Pg_magic_struct = pg_sys::Pg_magic_struct {
            len: 0,
            version: 0,
            funcmaxargs: 0,
            indexmaxkeys: 0,
            namedatalen: 0,
            float4byval: 0,
            float8byval: 0,
        };

        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(unused)]
        #[link_name = "Pg_magic_func"]
        pub extern "C" fn Pg_magic_func() -> &'static pg_sys::Pg_magic_struct {
            use pg_extension_sys::{pg_sys, register_panic_handler};
            use std::mem::size_of;
            use std::os::raw::c_int;

            // TODO: is this a good idea here?
            // register panic_handler
            register_panic_handler();

            unsafe {
                Pg_magic_data = pg_sys::Pg_magic_struct {
                    len: size_of::<pg_sys::Pg_magic_struct>() as c_int,
                    version: $vers as std::os::raw::c_int / 100,
                    funcmaxargs: pg_sys::FUNC_MAX_ARGS as std::os::raw::c_int,
                    indexmaxkeys: pg_sys::INDEX_MAX_KEYS as std::os::raw::c_int,
                    namedatalen: pg_sys::NAMEDATALEN as std::os::raw::c_int,
                    float4byval: pg_sys::USE_FLOAT4_BYVAL as std::os::raw::c_int,
                    float8byval: pg_sys::USE_FLOAT8_BYVAL as std::os::raw::c_int,
                };

                &Pg_magic_data
            }
        }
    };
}

/// Returns the slice of Datums, and a parallel slice which specifies if the Datum passed in is (SQL) NULL
pub fn get_args(
    func_call_info: &pg_sys::FunctionCallInfoData,
) -> (&[pg_sys::Datum], &[bool]) {
    use crate::pg_datum::TryFromPgDatum;

    let num_args = func_call_info.nargs as usize;

    let args: &[pg_sys::Datum] = &func_call_info.arg[..num_args];
    let args_null: &[bool] = &func_call_info.argnull[..num_args];

    (args, args_null)
}

/// This will replace the current panic_handler
pub fn register_panic_handler() {
    use std::panic::{self, PanicInfo};
    use crate::pg_error;

    panic::set_hook(Box::new(|info| {
        let level = pg_error::Level::Fatal;

        // FIXME: add this back when postgres linkage is fixed
        pg_error::log(level, file!(), line!(), module_path!(), format!("panic in rust extension: {:?}", info));
        // eprintln!("Panic in extension: {}", info);
    }));
}