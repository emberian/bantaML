use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:2"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:2"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:2"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:2"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:2"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "549:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "681:15"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
    }
}
pub use self::stddef_h::{size_t, NULL};
use self::stdio_h::{fprintf, fputc, fwrite, stderr};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t};
pub use self::FILE_h::FILE;
use crate::tl::tinylisp_h::*;
#[no_mangle]
#[c2rust::src_loc = "6:1"]
pub unsafe extern "C" fn _indent(mut lev: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < lev {
        fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "13:1"]
pub unsafe extern "C" fn tl_dbg_print(mut obj: *mut tl_object, mut level: libc::c_int) {
    _indent(level);
    if obj.is_null() {
        fprintf(
            stderr,
            b"() (NULL object)\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    match (*obj).kind as libc::c_uint {
        0 => {
            fprintf(
                stderr,
                b"INT: %ld\n\0" as *const u8 as *const libc::c_char,
                (*obj).c2rust_unnamed.ival,
            );
        }
        1 => {
            fprintf(
                stderr,
                b"SYM: len=%lu: \0" as *const u8 as *const libc::c_char,
                (*(*obj).c2rust_unnamed.nm).here.len,
            );
            fwrite(
                (*(*obj).c2rust_unnamed.nm).here.data as *const libc::c_void,
                (*(*obj).c2rust_unnamed.nm).here.len,
                1 as libc::c_int as libc::c_ulong,
                stderr,
            );
            fputc('\n' as i32, stderr);
        }
        2 => {
            fprintf(stderr, b"PAIR:\n\0" as *const u8 as *const libc::c_char);
            _indent(level + 1 as libc::c_int);
            fprintf(stderr, b"first:\n\0" as *const u8 as *const libc::c_char);
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed.first,
                level + 2 as libc::c_int,
            );
            _indent(level + 1 as libc::c_int);
            fprintf(stderr, b"next:\n\0" as *const u8 as *const libc::c_char);
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed.next,
                level + 2 as libc::c_int,
            );
        }
        4 | 5 | 3 => {
            fprintf(
                stderr,
                b"%s: %p\n\0" as *const u8 as *const libc::c_char,
                if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint {
                    b"CFUNC\0" as *const u8 as *const libc::c_char
                } else if (*obj).kind as libc::c_uint
                    == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                {
                    b"CFUNC_BYVAL\0" as *const u8 as *const libc::c_char
                } else {
                    b"THEN\0" as *const u8 as *const libc::c_char
                },
                (*obj).c2rust_unnamed.c2rust_unnamed_0.cfunc,
            );
            _indent(level + 1 as libc::c_int);
            fprintf(stderr, b"state:\n\0" as *const u8 as *const libc::c_char);
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed_0.state,
                level + 2 as libc::c_int,
            );
        }
        6 | 7 => {
            fprintf(
                stderr,
                b"%s:\n\0" as *const u8 as *const libc::c_char,
                if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                    b"MACRO\0" as *const u8 as *const libc::c_char
                } else {
                    b"FUNC\0" as *const u8 as *const libc::c_char
                },
            );
            _indent(level + 1 as libc::c_int);
            fprintf(stderr, b"args:\n\0" as *const u8 as *const libc::c_char);
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed_1.args,
                level + 2 as libc::c_int,
            );
            if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"envn:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).c2rust_unnamed.c2rust_unnamed_1.envn,
                    level + 2 as libc::c_int,
                );
            }
            _indent(level + 1 as libc::c_int);
            fprintf(stderr, b"body:\n\0" as *const u8 as *const libc::c_char);
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed_1.body,
                level + 2 as libc::c_int,
            );
        }
        8 => {
            fprintf(
                stderr,
                b"CONTINUATION:\n\0" as *const u8 as *const libc::c_char,
            );
            _indent(level + 1 as libc::c_int);
            fprintf(
                stderr,
                b"ret_conts:\n\0" as *const u8 as *const libc::c_char,
            );
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_conts,
                level + 2 as libc::c_int,
            );
            _indent(level + 1 as libc::c_int);
            fprintf(
                stderr,
                b"ret_values:\n\0" as *const u8 as *const libc::c_char,
            );
            tl_dbg_print(
                (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_values,
                level + 2 as libc::c_int,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"!!! UNKNOWN OBJECT KIND %d\n\0" as *const u8 as *const libc::c_char,
                (*obj).kind as libc::c_uint,
            );
        }
    };
}
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn _tl_cf_debug_print_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    fprintf(stderr, b"VALUE:\n\0" as *const u8 as *const libc::c_char);
    tl_dbg_print(
        if !result.is_null()
            && (result.is_null()
                || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*result).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        },
        0 as libc::c_int,
    );
    let ref mut fresh0 = (*in_0).values;
    *fresh0 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[no_mangle]
#[c2rust::src_loc = "97:1"]
pub unsafe extern "C" fn tl_cf_debug_print(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    fprintf(stderr, b"EXPR:\n\0" as *const u8 as *const libc::c_char);
    tl_dbg_print(
        if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        },
        0 as libc::c_int,
    );
    _tl_eval_and_then(
        in_0,
        if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        },
        0 as *mut tl_object,
        Some(
            _tl_cf_debug_print_k
                as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        ),
        b"tl_eval_and_then:_tl_cf_debug_print_k\0" as *const u8 as *const libc::c_char,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "97:1"]
static mut init_tl_cf_debug_print: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_debug_print
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-debug-print\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
        };
        init
    })
};
