extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn tl_new_pair(_: *mut tl_interp, _: *mut tl_object, _: *mut tl_object) -> *mut tl_object;
    fn _tl_eval_and_then(
        _: *mut tl_interp,
        _: *mut tl_object,
        _: *mut tl_object,
        _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        _: *const libc::c_char,
    );
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
pub unsafe extern "C" fn _indent(mut lev: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < lev {
        fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
#[no_mangle]
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
unsafe extern "C" fn _tl_cf_debug_print_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    _: *mut tl_object,
) {
    fprintf(stderr, b"VALUE:\n\0" as *const u8 as *const libc::c_char);
    tl_dbg_print(
        if !result.is_null()
            && (result.is_null()
                || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*result).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
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
pub unsafe extern "C" fn tl_cf_debug_print(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    _: *mut tl_object,
) {
    fprintf(stderr, b"EXPR:\n\0" as *const u8 as *const libc::c_char);
    tl_dbg_print(
        if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
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
