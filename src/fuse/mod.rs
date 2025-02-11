use std::{
    ffi::{c_char, c_int, c_void, CString},
    mem::MaybeUninit,
};

use crate::db::Db;

mod sys;

const HELLO_OPER: sys::fuse_operations = generate_fuse_ops();

unsafe extern "C" fn fuse_client_getattr(path: *const c_char, statbuf: *mut sys::stat) -> c_int {
    println!("Hello from fuse client getattr");

    (*statbuf).st_mode = sys::S_IFDIR | 0o755;
    0
}
unsafe extern "C" fn fuse_client_readdir(
    path: *const c_char,
    buf: *mut c_void,
    mut filler: sys::fuse_fill_dir_t,
    _offset: sys::off_t,
    _info: *mut sys::fuse_file_info,
) -> c_int {
    let context = sys::fuse_get_context();
    let client = (*context).private_data as *mut FuseClient;
    let client = &mut *client;

    let mut filler = filler.as_mut().unwrap();
    for db_item in client.db.iterate_items() {
        let name = CString::new(db_item.name.clone()).unwrap();
        filler(buf, name.as_ptr(), std::ptr::null(), 0);
    }
    0
}

#[derive(Debug)]
struct FuseClient {
    db: Db,
}

const fn generate_fuse_ops() -> sys::fuse_operations {
    unsafe {
        let mut ops: sys::fuse_operations = MaybeUninit::zeroed().assume_init();
        ops.getattr = Some(fuse_client_getattr);
        ops.readdir = Some(fuse_client_readdir);
        ops
    }
}

pub fn run_fuse_client(db: Db) {
    let mut client = FuseClient { db };
    let args: Vec<CString> = std::env::args().map(|s| CString::new(s).unwrap()).collect();
    let mut args: Vec<*mut i8> = args.into_iter().map(|s| s.into_raw()).collect();

    let mut args = sys::fuse_args {
        argc: args.len().try_into().unwrap(),
        argv: args.as_mut_ptr(),
        allocated: 0,
    };

    unsafe {
        let ret = sys::fuse_opt_parse(&mut args, std::ptr::null_mut(), std::ptr::null_mut(), None);
        if ret == -1 {
            panic!("Failed to parse fuse args");
        }

        sys::fuse_main_real(
            args.argc,
            args.argv,
            &HELLO_OPER,
            std::mem::size_of_val(&HELLO_OPER),
            &mut client as *mut FuseClient as *mut c_void,
        );
    }
}
