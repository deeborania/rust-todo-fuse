use std::{ffi::CString, mem::MaybeUninit};

mod sys;

const HELLO_OPER: sys::fuse_operations = generate_fuse_ops();

const fn generate_fuse_ops() -> sys::fuse_operations {
    unsafe {
        let ops = MaybeUninit::zeroed().assume_init();
        ops
    }
}

pub fn run_fuse_client() {
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
            std::ptr::null_mut(),
        );
    }
}
