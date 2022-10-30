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
#[c2rust::header_src = "/home/ember/src/tinylisp/tinylisp.h:4"]
pub mod tinylisp_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "417:8"]
    pub struct tl_interp_s {
        pub ns: tl_ns,
        pub top_env: *mut tl_object,
        pub env: *mut tl_object,
        pub true_: *mut tl_object,
        pub false_: *mut tl_object,
        pub error: *mut tl_object,
        pub prefixes: *mut tl_object,
        pub top_alloc: *mut tl_object,
        pub current: *mut tl_object,
        pub conts: *mut tl_object,
        pub values: *mut tl_object,
        pub rescue: *mut tl_object,
        pub gc_events: size_t,
        pub ctr_events: size_t,
        pub putback: libc::c_int,
        pub is_putback: libc::c_int,
        pub read_buffer: *mut libc::c_char,
        pub read_ptr: size_t,
        pub read_sz: size_t,
        pub disp_sep: libc::c_char,
        pub udata: *mut libc::c_void,
        pub readf: Option<unsafe extern "C" fn(*mut tl_interp_s) -> libc::c_int>,
        pub writef: Option<unsafe extern "C" fn(*mut tl_interp_s, libc::c_char) -> ()>,
        pub reallocf: Option<
            unsafe extern "C" fn(*mut tl_interp_s, *mut libc::c_void, size_t) -> *mut libc::c_void,
        >,
        pub modloadf:
            Option<unsafe extern "C" fn(*mut tl_interp_s, *const libc::c_char) -> libc::c_int>,
    }
    #[c2rust::src_loc = "83:1"]
    pub type tl_object = tl_object_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:16"]
    pub struct tl_object_s {
        pub kind: C2RustUnnamed_5,
        pub c2rust_unnamed: C2RustUnnamed_0,
        pub c2rust_unnamed_0: C2RustUnnamed,
        pub prev_alloc: *mut tl_object_s,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "175:2"]
    pub union C2RustUnnamed {
        pub next_alloc: *mut tl_object_s,
        pub next_alloc_i: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "132:2"]
    pub union C2RustUnnamed_0 {
        pub ival: libc::c_long,
        pub nm: *mut tl_name,
        pub c2rust_unnamed: C2RustUnnamed_4,
        pub c2rust_unnamed_0: C2RustUnnamed_3,
        pub c2rust_unnamed_1: C2RustUnnamed_2,
        pub c2rust_unnamed_2: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "161:3"]
    pub struct C2RustUnnamed_1 {
        pub ret_env: *mut tl_object_s,
        pub ret_conts: *mut tl_object_s,
        pub ret_values: *mut tl_object_s,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:3"]
    pub struct C2RustUnnamed_2 {
        pub args: *mut tl_object_s,
        pub body: *mut tl_object_s,
        pub env: *mut tl_object_s,
        pub envn: *mut tl_object_s,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:3"]
    pub struct C2RustUnnamed_3 {
        pub cfunc:
            Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object_s, *mut tl_object_s) -> ()>,
        pub state: *mut tl_object_s,
        pub name: *mut libc::c_char,
    }
    #[c2rust::src_loc = "72:1"]
    pub type tl_interp = tl_interp_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "137:3"]
    pub struct C2RustUnnamed_4 {
        pub first: *mut tl_object_s,
        pub next: *mut tl_object_s,
    }
    #[c2rust::src_loc = "73:1"]
    pub type tl_name = tl_name_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1043:8"]
    pub struct tl_name_s {
        pub here: tl_buffer,
        pub num_children: size_t,
        pub sz_children: size_t,
        pub children: *mut tl_child,
        pub chain: *mut tl_name_s,
    }
    #[c2rust::src_loc = "1038:1"]
    pub type tl_child = tl_child_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1038:16"]
    pub struct tl_child_s {
        pub seg: tl_buffer,
        pub name: *mut tl_name_s,
    }
    #[c2rust::src_loc = "1033:1"]
    pub type tl_buffer = tl_buffer_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1033:16"]
    pub struct tl_buffer_s {
        pub data: *mut libc::c_char,
        pub len: size_t,
    }
    #[c2rust::src_loc = "85:2"]
    pub type C2RustUnnamed_5 = libc::c_uint;
    #[c2rust::src_loc = "130:3"]
    pub const TL_CONT: C2RustUnnamed_5 = 8;
    #[c2rust::src_loc = "125:3"]
    pub const TL_FUNC: C2RustUnnamed_5 = 7;
    #[c2rust::src_loc = "120:3"]
    pub const TL_MACRO: C2RustUnnamed_5 = 6;
    #[c2rust::src_loc = "115:3"]
    pub const TL_CFUNC_BYVAL: C2RustUnnamed_5 = 5;
    #[c2rust::src_loc = "110:3"]
    pub const TL_CFUNC: C2RustUnnamed_5 = 4;
    #[c2rust::src_loc = "105:3"]
    pub const TL_THEN: C2RustUnnamed_5 = 3;
    #[c2rust::src_loc = "100:3"]
    pub const TL_PAIR: C2RustUnnamed_5 = 2;
    #[c2rust::src_loc = "95:3"]
    pub const TL_SYM: C2RustUnnamed_5 = 1;
    #[c2rust::src_loc = "90:3"]
    pub const TL_INT: C2RustUnnamed_5 = 0;
    #[c2rust::src_loc = "394:1"]
    pub type tl_ns = tl_ns_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "394:16"]
    pub struct tl_ns_s {
        pub root: *mut tl_name,
    }
    #[derive(Copy, Clone)]
    #[repr(C, align(8))]
    #[c2rust::src_loc = "992:16"]
    pub struct tl_init_ent_s(pub tl_init_ent_s_Inner);
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "992:16"]
    pub struct tl_init_ent_s_Inner {
        pub fn_0:
            Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        pub name: *const libc::c_char,
        pub flags: size_t,
    }
    #[allow(dead_code, non_upper_case_globals)]
    #[c2rust::src_loc = "992:16"]
    const tl_init_ent_s_PADDING: usize =
        ::std::mem::size_of::<tl_init_ent_s>() - ::std::mem::size_of::<tl_init_ent_s_Inner>();
    #[c2rust::src_loc = "992:1"]
    pub type tl_init_ent = tl_init_ent_s;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "295:11"]
        pub fn tl_new_pair(
            _: *mut tl_interp,
            _: *mut tl_object,
            _: *mut tl_object,
        ) -> *mut tl_object;
        #[c2rust::src_loc = "937:11"]
        pub fn _tl_eval_and_then(
            _: *mut tl_interp,
            _: *mut tl_object,
            _: *mut tl_object,
            _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
            _: *const libc::c_char,
        );
    }
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
#[c2rust::header_src = "/home/ember/src/tinylisp/debug.c:0"]
pub mod debug_c {
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
    pub unsafe extern "C" fn _tl_cf_debug_print_k(
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
    pub static mut init_tl_cf_debug_print: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_debug_print
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-debug-print\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::{fprintf, fputc, fwrite, stderr};
    use super::tinylisp_h::{
        tl_object, C2RustUnnamed_5, _tl_eval_and_then, tl_init_ent, tl_interp, tl_new_pair,
        tl_object_s, TL_CFUNC, TL_CFUNC_BYVAL, TL_MACRO, TL_PAIR,
    };
}
pub use self::debug_c::{
    _indent, _tl_cf_debug_print_k, init_tl_cf_debug_print, tl_cf_debug_print, tl_dbg_print,
};
pub use self::stddef_h::{size_t, NULL};
use self::stdio_h::{fprintf, fputc, fwrite, stderr};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::tinylisp_h::{
    _tl_eval_and_then, tl_buffer, tl_buffer_s, tl_child, tl_child_s, tl_init_ent, tl_init_ent_s,
    tl_init_ent_s_Inner, tl_init_ent_s_PADDING, tl_interp, tl_interp_s, tl_name, tl_name_s,
    tl_new_pair, tl_ns, tl_ns_s, tl_object, tl_object_s, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5, TL_CFUNC,
    TL_CFUNC_BYVAL, TL_CONT, TL_FUNC, TL_INT, TL_MACRO, TL_PAIR, TL_SYM, TL_THEN,
};
pub use self::types_h::{__off64_t, __off_t};
pub use self::FILE_h::FILE;
