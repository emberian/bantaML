extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn tl_new_int(_: *mut tl_interp, _: libc::c_long) -> *mut tl_object;
    fn tl_new_sym(_: *mut tl_interp, _: *const libc::c_char) -> *mut tl_object;
    fn tl_new_pair(_: *mut tl_interp, _: *mut tl_object, _: *mut tl_object) -> *mut tl_object;
    fn tl_new_then(
        _: *mut tl_interp,
        _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        _: *mut tl_object,
        _: *const libc::c_char,
    ) -> *mut tl_object;
    fn tl_gc(_: *mut tl_interp);
    fn tl_interp_init(_: *mut tl_interp);
    fn tl_interp_cleanup(_: *mut tl_interp);
    fn tl_print(_: *mut tl_interp, _: *mut tl_object) -> *mut tl_object;
    fn tl_printf(_: *mut tl_interp, _: *const libc::c_char, _: ...);
    fn tl_push_apply(_: *mut tl_interp, _: libc::c_long, _: *mut tl_object, _: *mut tl_object);
    fn _tl_eval_and_then(
        _: *mut tl_interp,
        _: *mut tl_object,
        _: *mut tl_object,
        _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        _: *const libc::c_char,
    );
    fn tl_run_until_done(_: *mut tl_interp);
    fn tl_read(_: *mut tl_interp);
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type tl_object = tl_object_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_object_s {
    pub kind: C2RustUnnamed_5,
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub prev_alloc: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub next_alloc: *mut tl_object_s,
    pub next_alloc_i: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
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
pub struct C2RustUnnamed_1 {
    pub ret_env: *mut tl_object_s,
    pub ret_conts: *mut tl_object_s,
    pub ret_values: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub args: *mut tl_object_s,
    pub body: *mut tl_object_s,
    pub env: *mut tl_object_s,
    pub envn: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub cfunc:
        Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object_s, *mut tl_object_s) -> ()>,
    pub state: *mut tl_object_s,
    pub name: *mut libc::c_char,
}
pub type tl_interp = tl_interp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub first: *mut tl_object_s,
    pub next: *mut tl_object_s,
}
pub type tl_name = tl_name_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_name_s {
    pub here: tl_buffer,
    pub num_children: size_t,
    pub sz_children: size_t,
    pub children: *mut tl_child,
    pub chain: *mut tl_name_s,
}
pub type tl_child = tl_child_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_child_s {
    pub seg: tl_buffer,
    pub name: *mut tl_name_s,
}
pub type tl_buffer = tl_buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_buffer_s {
    pub data: *mut libc::c_char,
    pub len: size_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const TL_CONT: C2RustUnnamed_5 = 8;
pub const TL_FUNC: C2RustUnnamed_5 = 7;
pub const TL_MACRO: C2RustUnnamed_5 = 6;
pub const TL_CFUNC_BYVAL: C2RustUnnamed_5 = 5;
pub const TL_CFUNC: C2RustUnnamed_5 = 4;
pub const TL_THEN: C2RustUnnamed_5 = 3;
pub const TL_PAIR: C2RustUnnamed_5 = 2;
pub const TL_SYM: C2RustUnnamed_5 = 1;
pub const TL_INT: C2RustUnnamed_5 = 0;
pub type tl_ns = tl_ns_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_ns_s {
    pub root: *mut tl_name,
}
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct tl_init_ent_s(pub tl_init_ent_s_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_init_ent_s_Inner {
    pub fn_0: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    pub name: *const libc::c_char,
    pub flags: size_t,
}
#[allow(dead_code, non_upper_case_globals)]
const tl_init_ent_s_PADDING: usize =
    ::std::mem::size_of::<tl_init_ent_s>() - ::std::mem::size_of::<tl_init_ent_s_Inner>();
pub type tl_init_ent = tl_init_ent_s;
#[no_mangle]
pub static mut _global_in: *mut tl_interp = 0 as *const tl_interp as *mut tl_interp;
#[no_mangle]
pub unsafe extern "C" fn my_modloadf(
    mut in_0: *mut tl_interp,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut hdl: *mut libc::c_void = dlopen(fname, 0x2 as libc::c_int | 0x100 as libc::c_int);
    if hdl.is_null() {
        tl_printf(
            in_0,
            b"Module load error: %s\n\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
        return 0 as libc::c_int;
    }
    let mut ini: *mut libc::c_void = dlsym(hdl, b"tl_init\0" as *const u8 as *const libc::c_char);
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
#[no_mangle]
pub static mut quiet: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut running: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn _main_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    _: *mut tl_object,
) {
    if quiet == 0 as libc::c_int {
        fprintf(stderr, b"Value: \0" as *const u8 as *const libc::c_char);
    }
    if quiet != 3 as libc::c_int
        && (quiet != 2 as libc::c_int
            || (if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
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
                0 as *mut tl_object_s
            },
        );
        tl_printf(in_0, b"\n\0" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if !((*in_0).values).is_null() {
        if quiet == 0 as libc::c_int {
            fprintf(
                stderr,
                b"(Rest of stack: \0" as *const u8 as *const libc::c_char,
            );
        }
        tl_print(in_0, (*in_0).values);
        fflush(stdout);
        if quiet == 0 as libc::c_int {
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
pub unsafe extern "C" fn _main_read_k(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    _: *mut tl_object,
) {
    let mut expr: *mut tl_object = if !args.is_null()
        && (args.is_null()
            || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*args).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    if expr.is_null() {
        if quiet == 0 as libc::c_int {
            fprintf(stderr, b"Done.\n\0" as *const u8 as *const libc::c_char);
        }
        tl_interp_cleanup(in_0);
        running = 0 as libc::c_int;
        return;
    }
    if quiet == 0 as libc::c_int {
        if quiet == 0 as libc::c_int {
            fprintf(stderr, b"Read: \0" as *const u8 as *const libc::c_char);
        }
        tl_print(in_0, expr);
        fflush(stdout);
        if quiet == 0 as libc::c_int {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    let ref mut fresh1 = (*in_0).current;
    *fresh1 = 0 as *mut tl_object;
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
pub unsafe extern "C" fn tl_cf_quiet(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    _: *mut tl_object,
) {
    if !args.is_null() {
        let mut arg: *mut tl_object = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
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
pub unsafe extern "C" fn tl_cf_exit(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    _: *mut tl_object,
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
            0 as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _print_cont(
    mut in_0: *mut tl_interp,
    mut cont: *mut tl_object,
    mut level: libc::c_int,
) {
    let mut len: *mut tl_object = 0 as *mut tl_object;
    let mut callex: *mut tl_object = 0 as *mut tl_object;
    fprintf(stderr, b"Len \0" as *const u8 as *const libc::c_char);
    len = if !cont.is_null()
        && (cont.is_null()
            || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*cont).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
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
        0 as *mut tl_object_s
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
        && (*callex).c2rust_unnamed_0.next_alloc_i & 0x1 as libc::c_int as libc::c_ulong == 0
    {
        let ref mut fresh7 = (*callex).c2rust_unnamed_0.next_alloc_i;
        *fresh7 |= 0x1 as libc::c_int as libc::c_ulong;
        fprintf(stderr, b":\0" as *const u8 as *const libc::c_char);
        _print_cont_stack(
            in_0,
            (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_conts,
            level + 1 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _print_cont_stack(
    mut in_0: *mut tl_interp,
    mut stack: *mut tl_object,
    mut level: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut l_cont: *mut tl_object = (*in_0).conts;
    let mut cont: *mut tl_object = if !((*in_0).conts).is_null()
        && (((*in_0).conts).is_null()
            || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
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
            0 as *mut tl_object_s
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
            0 as *mut tl_object_s
        });
        cont = (if !l_cont.is_null()
            && (l_cont.is_null()
                || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_cont).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_cont_stack(mut in_0: *mut tl_interp, mut stack: *mut tl_object) {
    let mut obj: *mut tl_object = (*in_0).top_alloc;
    while !obj.is_null() {
        let ref mut fresh8 = (*obj).c2rust_unnamed_0.next_alloc_i;
        *fresh8 &= !(0x3 as libc::c_int) as libc::c_ulong;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong)
            as *mut tl_object;
    }
    fprintf(stderr, b"\nCurrent: \0" as *const u8 as *const libc::c_char);
    _print_cont(in_0, (*in_0).current, 0 as libc::c_int);
    _print_cont_stack(in_0, stack, 0 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    let mut in_0: tl_interp = tl_interp {
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
    let mut expr: *mut tl_object = 0 as *mut tl_object;
    let mut val: *mut tl_object = 0 as *mut tl_object;
    _global_in = &mut in_0;
    if isatty(0 as libc::c_int) == 0 {
        quiet = 2 as libc::c_int;
    }
    tl_interp_init(&mut in_0);
    in_0.modloadf = Some(
        my_modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
    );
    if quiet == 0 as libc::c_int {
        if quiet == 0 as libc::c_int {
            fprintf(stderr, b"Top Env: \0" as *const u8 as *const libc::c_char);
        }
        tl_print(&mut in_0, in_0.top_env);
        fflush(stdout);
        if quiet == 0 as libc::c_int {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    while running != 0 {
        if quiet == 0 as libc::c_int {
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
            let mut l_frm: *mut tl_object = in_0.env;
            let mut frm: *mut tl_object = if !(in_0.env).is_null()
                && ((in_0.env).is_null()
                    || (*in_0.env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*in_0.env).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            };
            while !l_frm.is_null() {
                fprintf(stderr, b"\nFrame\0" as *const u8 as *const libc::c_char);
                if if !l_frm.is_null()
                    && (l_frm.is_null()
                        || (*l_frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_frm).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
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
                    0 as *mut tl_object_s
                });
                frm = (if !l_frm.is_null()
                    && (l_frm.is_null()
                        || (*l_frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_frm).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                });
            }
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            in_0.error = 0 as *mut tl_object;
        }
        in_0.conts = 0 as *mut tl_object;
        in_0.values = 0 as *mut tl_object;
        tl_gc(&mut in_0);
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
