use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
use crate::tl::tinylisp_h::*;
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:1"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:1"]
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
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:1"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:1"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "144:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "230:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:3"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "624:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:7"]
pub mod unistd_h {
    #[c2rust::src_loc = "210:9"]
    pub const STDIN_FILENO: libc::c_int = 0 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "809:1"]
        pub fn isatty(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/dlfcn.h:30"]
pub mod dlfcn_h {
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
            -> *mut libc::c_void;
        #[c2rust::src_loc = "84:1"]
        pub fn dlerror() -> *mut libc::c_char;
        #[c2rust::src_loc = "58:1"]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/dlfcn.h:30"]
pub mod bits_dlfcn_h {
    #[c2rust::src_loc = "25:9"]
    pub const RTLD_NOW: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const RTLD_GLOBAL: libc::c_int = 0x100 as libc::c_int;
}
pub use self::bits_dlfcn_h::{RTLD_GLOBAL, RTLD_NOW};
use self::dlfcn_h::{dlerror, dlopen, dlsym};
pub use self::stddef_h::{size_t, NULL};
use self::stdio_h::{fflush, fprintf, stderr, stdout};
use self::stdlib_h::exit;
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t};
pub use self::unistd_h::{isatty, STDIN_FILENO};
pub use self::FILE_h::FILE;
#[no_mangle]
#[c2rust::src_loc = "27:12"]
pub static mut _global_in: *mut tl_interp = 0 as *const tl_interp as *mut tl_interp;
#[no_mangle]
#[c2rust::src_loc = "31:1"]
pub unsafe extern "C" fn my_modloadf(
    mut in_0: *mut tl_interp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut hdl = dlopen(fname, RTLD_NOW | RTLD_GLOBAL);
    if hdl.is_null() {
        tl_printf(
            in_0,
            b"Module load error: %s\n\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
        return 0 as libc::c_int;
    }
    let mut ini = dlsym(hdl, b"tl_init\0" as *const u8 as *const libc::c_char);
    if ini.is_null() {
        tl_printf(
            in_0,
            b"Module init error: %s\n\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
        return 0 as libc::c_int;
    }
    return (*(&mut ini as *mut *mut libc::c_void
        as *mut Option<unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int>))
        .expect("non-null function pointer")(in_0, fname);
}
#[c2rust::src_loc = "46:9"]
pub const QUIET_OFF: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "48:9"]
pub const QUIET_NO_TRUE: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "49:9"]
pub const QUIET_NO_VALUE: libc::c_int = 3 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "50:5"]
pub static mut quiet: libc::c_int = QUIET_OFF;
#[no_mangle]
#[c2rust::src_loc = "53:5"]
pub static mut running: libc::c_int = 1 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn _main_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    if quiet == QUIET_OFF {
        fprintf(stderr, b"Value: \0" as *const u8 as *const libc::c_char);
    }
    if quiet != QUIET_NO_VALUE
        && (quiet != QUIET_NO_TRUE
            || (if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            }) != (*in_0).true_)
    {
        tl_print(
            in_0,
            if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
        );
        tl_printf(in_0, b"\n\0" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if !((*in_0).values).is_null() {
        if quiet == QUIET_OFF {
            fprintf(
                stderr,
                b"(Rest of stack: \0" as *const u8 as *const libc::c_char,
            );
        }
        tl_print(in_0, (*in_0).values);
        fflush(stdout);
        if quiet == QUIET_OFF {
            fprintf(stderr, b")\n\0" as *const u8 as *const libc::c_char);
        }
    }
    let ref mut fresh0 = (*in_0).values;
    *fresh0 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[no_mangle]
#[c2rust::src_loc = "71:1"]
pub unsafe extern "C" fn _main_read_k(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    let mut expr = if !args.is_null()
        && (args.is_null()
            || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*args).c2rust_unnamed.c2rust_unnamed.first
    } else {
        NULL as *mut tl_object_s
    };
    if expr.is_null() {
        if quiet == QUIET_OFF {
            fprintf(stderr, b"Done.\n\0" as *const u8 as *const libc::c_char);
        }
        tl_interp_cleanup(in_0);
        running = 0 as libc::c_int;
        return;
    }
    if quiet == QUIET_OFF {
        if quiet == QUIET_OFF {
            fprintf(stderr, b"Read: \0" as *const u8 as *const libc::c_char);
        }
        tl_print(in_0, expr);
        fflush(stdout);
        if quiet == QUIET_OFF {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    let ref mut fresh1 = (*in_0).current;
    *fresh1 = TL_EMPTY_LIST as *mut tl_object;
    _tl_eval_and_then(
        in_0,
        expr,
        0 as *mut tl_object,
        Some(_main_k as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()),
        b"tl_eval_and_then:_main_k\0" as *const u8 as *const libc::c_char,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "89:1"]
static mut init_tl_cf_quiet: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_quiet
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-quiet\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn tl_cf_quiet(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    if !args.is_null() {
        let mut arg = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if !arg.is_null() && (*arg).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint {
            quiet = (*arg).c2rust_unnamed.ival as libc::c_int;
            let ref mut fresh2 = (*in_0).values;
            *fresh2 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        } else {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-quiet on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    arg,
                );
            } else {
                let ref mut fresh3 = (*in_0).error;
                *fresh3 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-quiet on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    arg,
                );
            };
        }
    } else {
        let ref mut fresh4 = (*in_0).values;
        *fresh4 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_int(in_0, quiet as libc::c_long),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
        return;
    };
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "103:1"]
static mut init_tl_cf_exit: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_exit
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-exit\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[no_mangle]
#[c2rust::src_loc = "103:1"]
pub unsafe extern "C" fn tl_cf_exit(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    if args.is_null()
        || !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            }))
            .kind as libc::c_uint
                == TL_INT as libc::c_int as libc::c_uint)
    {
        if !((*in_0).error).is_null() {
            tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"tl-exit on non-int\0" as *const u8 as *const libc::c_char,
                ),
                args,
            );
        } else {
            let ref mut fresh5 = (*in_0).error;
            *fresh5 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"tl-exit on non-int\0" as *const u8 as *const libc::c_char,
                ),
                args,
            );
        };
        let ref mut fresh6 = (*in_0).values;
        *fresh6 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    exit(
        (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn _print_cont(
    mut in_0: *mut tl_interp,
    mut cont: *mut tl_object,
    mut level: libc::c_int,
) {
    let mut len = 0 as *mut tl_object;
    let mut callex = 0 as *mut tl_object;
    fprintf(stderr, b"Len \0" as *const u8 as *const libc::c_char);
    len = if !cont.is_null()
        && (cont.is_null()
            || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*cont).c2rust_unnamed.c2rust_unnamed.first
    } else {
        NULL as *mut tl_object_s
    };
    tl_print(in_0, len);
    fflush(stdout);
    if !len.is_null()
        && (*len).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
        && (*len).c2rust_unnamed.ival < 0 as libc::c_int as libc::c_long
    {
        match (*len).c2rust_unnamed.ival {
            -1 => {
                fprintf(
                    stderr,
                    b" (TL_APPLY_PUSH_EVAL)\0" as *const u8 as *const libc::c_char,
                );
            }
            -2 => {
                fprintf(
                    stderr,
                    b" (TL_APPLY_INDIRECT)\0" as *const u8 as *const libc::c_char,
                );
            }
            -3 => {
                fprintf(
                    stderr,
                    b" (TL_APPLY_DROP_EVAL)\0" as *const u8 as *const libc::c_char,
                );
            }
            -4 => {
                fprintf(
                    stderr,
                    b" (TL_APPLY_DROP)\0" as *const u8 as *const libc::c_char,
                );
            }
            -5 => {
                fprintf(
                    stderr,
                    b" (TL_APPLY_DROP_RESCUE)\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
    }
    fprintf(stderr, b" Callex \0" as *const u8 as *const libc::c_char);
    callex = if !(if !cont.is_null()
        && (cont.is_null()
            || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*cont).c2rust_unnamed.c2rust_unnamed.next
    } else {
        0 as *mut tl_object_s
    })
    .is_null()
        && ((if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            || (*(if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            }))
            .kind as libc::c_uint
                == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .c2rust_unnamed
        .c2rust_unnamed
        .first
    } else {
        NULL as *mut tl_object_s
    };
    tl_print(in_0, callex);
    fflush(stdout);
    if !callex.is_null()
        && (*callex).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
        && !((*callex).c2rust_unnamed.c2rust_unnamed_0.state).is_null()
    {
        fprintf(
            stderr,
            b" Returns to \0" as *const u8 as *const libc::c_char,
        );
        _print_cont(
            in_0,
            (*callex).c2rust_unnamed.c2rust_unnamed_0.state,
            level + 1 as libc::c_int,
        );
    }
    if !callex.is_null()
        && (*callex).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint
        && (*callex).c2rust_unnamed_0.next_alloc_i & TL_F_MARK as libc::c_ulong == 0
    {
        let ref mut fresh7 = (*callex).c2rust_unnamed_0.next_alloc_i;
        *fresh7 |= TL_F_MARK as libc::c_ulong;
        fprintf(stderr, b":\0" as *const u8 as *const libc::c_char);
        _print_cont_stack(
            in_0,
            (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_conts,
            level + 1 as libc::c_int,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn _print_cont_stack(
    mut in_0: *mut tl_interp,
    mut stack: *mut tl_object,
    mut level: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut l_cont = (*in_0).conts;
    let mut cont = if !((*in_0).conts).is_null()
        && (((*in_0).conts).is_null()
            || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
    } else {
        NULL as *mut tl_object_s
    };
    while !l_cont.is_null() {
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int;
        while i < level {
            fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        fprintf(stderr, b"Stack\0" as *const u8 as *const libc::c_char);
        if l_cont == (*in_0).conts {
            fprintf(stderr, b"(Top)\0" as *const u8 as *const libc::c_char);
        }
        if if !l_cont.is_null()
            && (l_cont.is_null()
                || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_cont).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        }
        .is_null()
        {
            fprintf(stderr, b"(Bottom)\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b": \0" as *const u8 as *const libc::c_char);
        _print_cont(in_0, cont, level);
        l_cont = (if !l_cont.is_null()
            && (l_cont.is_null()
                || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_cont).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        });
        cont = (if !l_cont.is_null()
            && (l_cont.is_null()
                || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_cont).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        });
    }
}
#[no_mangle]
#[c2rust::src_loc = "161:1"]
pub unsafe extern "C" fn print_cont_stack(mut in_0: *mut tl_interp, mut stack: *mut tl_object) {
    let mut obj = (*in_0).top_alloc;
    while !obj.is_null() {
        let ref mut fresh8 = (*obj).c2rust_unnamed_0.next_alloc_i;
        *fresh8 &= !TL_FMASK as libc::c_ulong;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object;
    }
    fprintf(stderr, b"\nCurrent: \0" as *const u8 as *const libc::c_char);
    _print_cont(in_0, (*in_0).current, 0 as libc::c_int);
    _print_cont_stack(in_0, stack, 0 as libc::c_int);
}
#[c2rust::src_loc = "183:1"]
unsafe fn main_0() -> libc::c_int {
    let mut in_0 = tl_interp {
        ns: tl_ns {
            root: 0 as *mut tl_name,
        },
        top_env: 0 as *mut tl_object,
        env: 0 as *mut tl_object,
        true_: 0 as *mut tl_object,
        false_: 0 as *mut tl_object,
        error: 0 as *mut tl_object,
        prefixes: 0 as *mut tl_object,
        top_alloc: 0 as *mut tl_object,
        current: 0 as *mut tl_object,
        conts: 0 as *mut tl_object,
        values: 0 as *mut tl_object,
        rescue: 0 as *mut tl_object,
        gc_events: 0,
        ctr_events: 0,
        putback: 0,
        is_putback: 0,
        read_buffer: 0 as *mut libc::c_char,
        read_ptr: 0,
        read_sz: 0,
        disp_sep: 0,
        udata: 0 as *mut libc::c_void,
        readf: None,
        writef: None,
        reallocf: None,
        modloadf: None,
    };
    let mut expr = 0 as *mut tl_object;
    let mut val = 0 as *mut tl_object;
    _global_in = &mut in_0;
    if isatty(STDIN_FILENO) == 0 {
        quiet = QUIET_NO_TRUE;
    }
    tl_interp_init(&mut in_0);
    in_0.modloadf = Some(
        my_modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
    );
    if quiet == QUIET_OFF {
        if quiet == QUIET_OFF {
            fprintf(stderr, b"Top Env: \0" as *const u8 as *const libc::c_char);
        }
        tl_print(&mut in_0, in_0.top_env);
        fflush(stdout);
        if quiet == QUIET_OFF {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    while running != 0 {
        if quiet == QUIET_OFF {
            fprintf(stderr, b"> \0" as *const u8 as *const libc::c_char);
        }
        tl_push_apply(
            &mut in_0,
            1 as libc::c_int as libc::c_long,
            tl_new_then(
                &mut in_0,
                Some(
                    _main_read_k
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                0 as *mut tl_object,
                b"tl_read_and_then:_main_read_k\0" as *const u8 as *const libc::c_char,
            ),
            in_0.env,
        );
        tl_read(&mut in_0);
        tl_run_until_done(&mut in_0);
        if running == 0 {
            break;
        }
        if !(in_0.error).is_null() {
            fprintf(stderr, b"Error: \0" as *const u8 as *const libc::c_char);
            tl_print(&mut in_0, in_0.error);
            fflush(stdout);
            print_cont_stack(&mut in_0, in_0.conts);
            fprintf(stderr, b"\nValues: \0" as *const u8 as *const libc::c_char);
            tl_print(&mut in_0, in_0.values);
            fflush(stdout);
            let mut l_frm = in_0.env;
            let mut frm = if !(in_0.env).is_null()
                && ((in_0.env).is_null()
                    || (*in_0.env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*in_0.env).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            };
            while !l_frm.is_null() {
                fprintf(stderr, b"\nFrame\0" as *const u8 as *const libc::c_char);
                if if !l_frm.is_null()
                    && (l_frm.is_null()
                        || (*l_frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_frm).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                }
                .is_null()
                {
                    fprintf(stderr, b"(Outer)\0" as *const u8 as *const libc::c_char);
                }
                if l_frm == in_0.env {
                    fprintf(stderr, b"(Inner)\0" as *const u8 as *const libc::c_char);
                }
                fprintf(stderr, b": \0" as *const u8 as *const libc::c_char);
                tl_print(&mut in_0, frm);
                fflush(stdout);
                l_frm = (if !l_frm.is_null()
                    && (l_frm.is_null()
                        || (*l_frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_frm).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                });
                frm = (if !l_frm.is_null()
                    && (l_frm.is_null()
                        || (*l_frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_frm).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                });
            }
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            in_0.error = NULL as *mut tl_object;
        }
        in_0.conts = TL_EMPTY_LIST as *mut tl_object;
        in_0.values = TL_EMPTY_LIST as *mut tl_object;
        tl_gc(&mut in_0);
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
