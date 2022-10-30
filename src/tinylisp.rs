use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stdarg.h:1"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:1"]
pub mod types_h {
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
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
#[c2rust::header_src = "/home/ember/src/tinylisp/tinylisp.h:1"]
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
    #[c2rust::src_loc = "992:1"]
    pub type tl_init_ent = tl_init_ent_s;
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
    pub const tl_init_ent_s_PADDING: usize =
        ::std::mem::size_of::<tl_init_ent_s>() - ::std::mem::size_of::<tl_init_ent_s_Inner>();
    #[c2rust::src_loc = "27:9"]
    pub const TL_DEFAULT_GC_EVENTS: libc::c_int = 65536 as libc::c_int;
    #[c2rust::src_loc = "194:9"]
    pub const TL_FMASK: libc::c_int = 0x3 as libc::c_int;
    #[c2rust::src_loc = "201:9"]
    pub const TL_F_MARK: libc::c_int = 0x1 as libc::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const TL_F_PERMANENT: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "357:9"]
    pub const TL_EMPTY_LIST: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "913:9"]
    pub const TL_RESULT_DONE: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "922:9"]
    pub const TL_RESULT_AGAIN: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "890:9"]
    pub const TL_APPLY_DROP_EVAL: libc::c_int = -(3 as libc::c_int);
    #[c2rust::src_loc = "868:9"]
    pub const TL_APPLY_PUSH_EVAL: libc::c_int = -(1 as libc::c_int);
    #[c2rust::src_loc = "896:9"]
    pub const TL_APPLY_DROP: libc::c_int = -(4 as libc::c_int);
    #[c2rust::src_loc = "884:9"]
    pub const TL_APPLY_INDIRECT: libc::c_int = -(2 as libc::c_int);
    #[c2rust::src_loc = "934:9"]
    pub const TL_RESULT_GETCHAR: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "907:9"]
    pub const TL_APPLY_GETCHAR: libc::c_int = -(6 as libc::c_int);
    #[c2rust::src_loc = "902:9"]
    pub const TL_APPLY_DROP_RESCUE: libc::c_int = -(5 as libc::c_int);
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/home/ember/src/tinylisp/print.c:6"]
pub mod print_c {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:2"]
    pub union C2RustUnnamed_6 {
        pub s: *const libc::c_char,
        pub b: *mut tl_buffer,
    }
    #[c2rust::src_loc = "7:9"]
    pub const QUOTED_SYM_CHARS: [libc::c_char; 22] = unsafe {
        *::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
            b"0123456789.,'\"` \n\r\t\x08\x0B\0",
        )
    };
    #[no_mangle]
    #[c2rust::src_loc = "9:1"]
    pub unsafe extern "C" fn _print_pairs(mut in_0: *mut tl_interp, mut cur: *mut tl_object) {
        while !cur.is_null() {
            if !(cur.is_null()
                || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                tl_printf(in_0, b". \0" as *const u8 as *const libc::c_char);
                tl_print(in_0, cur);
                cur = NULL as *mut tl_object;
            } else {
                tl_print(
                    in_0,
                    if !cur.is_null()
                        && (cur.is_null()
                            || (*cur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*cur).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    },
                );
                if !if !cur.is_null()
                    && (cur.is_null()
                        || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cur).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                }
                .is_null()
                {
                    tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                }
                cur = if !cur.is_null()
                    && (cur.is_null()
                        || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cur).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                };
            }
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "25:1"]
    pub unsafe extern "C" fn _mempbrk(
        mut m: *mut libc::c_char,
        mut s: *mut libc::c_char,
        mut sz: size_t,
    ) -> *mut libc::c_char {
        let mut i: size_t = 0;
        let mut t = 0 as *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < sz {
            t = s;
            while *t != 0 {
                if *m as libc::c_int == *t as libc::c_int {
                    return m;
                }
                t = t.offset(1);
            }
            i = i.wrapping_add(1);
            m = m.offset(1);
        }
        return NULL as *mut libc::c_char;
    }
    #[no_mangle]
    #[c2rust::src_loc = "36:1"]
    pub unsafe extern "C" fn tl_print(
        mut in_0: *mut tl_interp,
        mut obj: *mut tl_object,
    ) -> *mut tl_object {
        let mut cur = 0 as *mut tl_object;
        if obj.is_null() {
            tl_printf(in_0, b"()\0" as *const u8 as *const libc::c_char);
            return (*in_0).true_;
        }
        match (*obj).kind as libc::c_uint {
            0 => {
                tl_printf(
                    in_0,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*obj).c2rust_unnamed.ival,
                );
            }
            1 => {
                if (*(*obj).c2rust_unnamed.nm).here.len == 0 as libc::c_int as libc::c_ulong
                    || !(_mempbrk(
                        (*(*obj).c2rust_unnamed.nm).here.data,
                        QUOTED_SYM_CHARS.as_mut_ptr(),
                        (*(*obj).c2rust_unnamed.nm).here.len,
                    ))
                    .is_null()
                {
                    ((*in_0).writef).expect("non-null function pointer")(
                        in_0,
                        '"' as i32 as libc::c_char,
                    );
                    tl_write(
                        in_0,
                        (*(*obj).c2rust_unnamed.nm).here.data,
                        (*(*obj).c2rust_unnamed.nm).here.len,
                    );
                    ((*in_0).writef).expect("non-null function pointer")(
                        in_0,
                        '"' as i32 as libc::c_char,
                    );
                } else {
                    tl_write(
                        in_0,
                        (*(*obj).c2rust_unnamed.nm).here.data,
                        (*(*obj).c2rust_unnamed.nm).here.len,
                    );
                }
            }
            2 => {
                tl_printf(in_0, b"(\0" as *const u8 as *const libc::c_char);
                _print_pairs(in_0, obj);
                tl_printf(in_0, b")\0" as *const u8 as *const libc::c_char);
            }
            4 | 5 | 3 => {
                tl_printf(
                    in_0,
                    b"%s:%p\0" as *const u8 as *const libc::c_char,
                    if !((*obj).c2rust_unnamed.c2rust_unnamed_0.name).is_null() {
                        (*obj).c2rust_unnamed.c2rust_unnamed_0.name as *const libc::c_char
                    } else if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
                    {
                        b"<cfunc>\0" as *const u8 as *const libc::c_char
                    } else if (*obj).kind as libc::c_uint
                        == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                    {
                        b"<cfunc_byval>\0" as *const u8 as *const libc::c_char
                    } else {
                        b"<then>\0" as *const u8 as *const libc::c_char
                    },
                    (*obj).c2rust_unnamed.c2rust_unnamed_0.cfunc,
                );
            }
            6 | 7 => {
                tl_printf(
                    in_0,
                    b"(%s \0" as *const u8 as *const libc::c_char,
                    if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                        b"macro\0" as *const u8 as *const libc::c_char
                    } else {
                        b"lambda\0" as *const u8 as *const libc::c_char
                    },
                );
                tl_print(in_0, (*obj).c2rust_unnamed.c2rust_unnamed_1.args);
                tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                if !obj.is_null()
                    && (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
                {
                    tl_print(in_0, (*obj).c2rust_unnamed.c2rust_unnamed_1.envn);
                    tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                }
                _print_pairs(in_0, (*obj).c2rust_unnamed.c2rust_unnamed_1.body);
                tl_printf(in_0, b")\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                tl_printf(in_0, b"cont:%p\0" as *const u8 as *const libc::c_char, obj);
            }
            _ => {
                tl_printf(
                    in_0,
                    b"<unknown object kind %d>\0" as *const u8 as *const libc::c_char,
                    (*obj).kind as libc::c_uint,
                );
            }
        }
        return (*in_0).true_;
    }
    #[no_mangle]
    #[c2rust::src_loc = "98:1"]
    pub unsafe extern "C" fn tl_puts(mut in_0: *mut tl_interp, mut s: *const libc::c_char) {
        while *s as libc::c_int != 0 as libc::c_int {
            let fresh214 = s;
            s = s.offset(1);
            ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh214);
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "102:1"]
    pub unsafe extern "C" fn tl_write(
        mut in_0: *mut tl_interp,
        mut data: *const libc::c_char,
        mut len: size_t,
    ) {
        let mut i = 0 as libc::c_int as size_t;
        loop {
            let fresh215 = i;
            i = i.wrapping_add(1);
            if !(fresh215 < len) {
                break;
            }
            let fresh216 = data;
            data = data.offset(1);
            ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh216);
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn tl_printf(
        mut in_0: *mut tl_interp,
        mut cur: *const libc::c_char,
        mut args: ...
    ) {
        let mut ap: ::std::ffi::VaListImpl;
        let mut temp = C2RustUnnamed_6 {
            s: 0 as *const libc::c_char,
        };
        let mut buf: [libc::c_char; 32] = [0; 32];
        ap = args.clone();
        while *cur as libc::c_int != 0 as libc::c_int {
            if *cur as libc::c_int == '%' as i32 {
                cur = cur.offset(1);
                match *cur as libc::c_int {
                    0 => {
                        ((*in_0).writef).expect("non-null function pointer")(
                            in_0,
                            '%' as i32 as libc::c_char,
                        );
                    }
                    37 => {
                        ((*in_0).writef).expect("non-null function pointer")(
                            in_0,
                            '%' as i32 as libc::c_char,
                        );
                        cur = cur.offset(1);
                    }
                    115 => {
                        temp.s = ap.arg::<*const libc::c_char>();
                        if (temp.s).is_null() {
                            tl_puts(in_0, b"<NULL>\0" as *const u8 as *const libc::c_char);
                        } else {
                            tl_puts(in_0, temp.s);
                        }
                        cur = cur.offset(1);
                    }
                    112 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%p\0" as *const u8 as *const libc::c_char,
                            ap.arg::<*mut libc::c_void>(),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    108 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            ap.arg::<libc::c_long>(),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(2 as libc::c_int as isize);
                    }
                    100 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            ap.arg::<libc::c_int>(),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    78 => {
                        temp.b = ap.arg::<*mut tl_buffer>();
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*temp.b).len,
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        ((*in_0).writef).expect("non-null function pointer")(
                            in_0,
                            ':' as i32 as libc::c_char,
                        );
                        tl_write(in_0, (*temp.b).data, (*temp.b).len);
                        cur = cur.offset(1);
                    }
                    79 => {
                        tl_print(in_0, ap.arg::<*mut tl_object>());
                        cur = cur.offset(1);
                    }
                    _ => {
                        ((*in_0).writef).expect("non-null function pointer")(
                            in_0,
                            '%' as i32 as libc::c_char,
                        );
                        let fresh217 = cur;
                        cur = cur.offset(1);
                        ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh217);
                    }
                }
            } else {
                let fresh218 = cur;
                cur = cur.offset(1);
                ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh218);
            }
        }
    }
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::snprintf;
    use super::tinylisp_h::{
        tl_buffer, tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_CFUNC, TL_CFUNC_BYVAL,
        TL_MACRO, TL_PAIR,
    };
}
#[c2rust::header_src = "/usr/include/ctype.h:7"]
pub mod ctype_h {
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_7 = 2048;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_7 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_7 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_7 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_7 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_7 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_7 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_7 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_7 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_7 = 4096;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_7 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_7 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_7 = 256;
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/string.h:1"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "141:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "64:12"]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:1"]
pub mod stdio_h {
    #[c2rust::src_loc = "104:9"]
    pub const EOF: libc::c_int = -1;
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
        #[c2rust::src_loc = "378:12"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
        #[c2rust::src_loc = "520:1"]
        pub fn getchar() -> libc::c_int;
        #[c2rust::src_loc = "549:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "556:1"]
        pub fn putchar(__c: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "681:15"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/byteswap.h:1"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
            | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
            as __uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
            | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
            | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
            | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong & 0xff00000000000000 as libc::c_ulonglong)
            >> 56 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
                >> 40 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
                >> 24 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong) >> 8 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong) << 24 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong) << 40 as libc::c_int
            | (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong) << 56 as libc::c_int)
            as __uint64_t;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/uintn-identity.h:1"]
pub mod uintn_identity_h {
    #[inline]
    #[c2rust::src_loc = "32:1"]
    pub unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
        return __x;
    }
    #[inline]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
        return __x;
    }
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "551:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "555:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "624:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:1"]
pub mod assert_h {
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
    }
}
#[c2rust::header_src = "/home/ember/src/tinylisp/builtin.c:1"]
pub mod builtin_c {
    #[c2rust::src_loc = "326:1"]
    pub unsafe extern "C" fn _tl_readc_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let ref mut fresh0 = (*in_0).values;
        *fresh0 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "513:1"]
    pub static mut init_tl_cf_all_objects: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_all_objects
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-all-objects\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn tl_cf_macro(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut fargs = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut envn = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        let mut body = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .next
        } else {
            NULL as *mut tl_object_s
        };
        if !(!envn.is_null()
            && (*envn).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"bad macro envname\0" as *const u8 as *const libc::c_char,
                    ),
                    envn,
                );
            } else {
                let ref mut fresh1 = (*in_0).error;
                *fresh1 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"bad macro envname\0" as *const u8 as *const libc::c_char,
                    ),
                    envn,
                );
            };
            let ref mut fresh2 = (*in_0).values;
            *fresh2 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh3 = (*in_0).values;
        *fresh3 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_macro(in_0, fargs, envn, body, (*in_0).env),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "57:1"]
    pub unsafe extern "C" fn tl_cf_lambda(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut fargs = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut body = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        };
        let ref mut fresh4 = (*in_0).values;
        *fresh4 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_macro(in_0, fargs, 0 as *mut tl_object, body, (*in_0).env),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn tl_cf_define(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut key = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut val = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad define arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh5 = (*in_0).error;
                *fresh5 = tl_new_sym(
                    in_0,
                    b"bad define arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        if !(!key.is_null() && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"define non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
                );
            } else {
                let ref mut fresh6 = (*in_0).error;
                *fresh6 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"define non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
                );
            };
            let ref mut fresh7 = (*in_0).values;
            *fresh7 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        _tl_eval_and_then(
            in_0,
            val,
            key,
            Some(
                _tl_cf_define_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_eval_and_then:_tl_cf_define_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "63:1"]
    pub unsafe extern "C" fn _tl_cf_define_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut key: *mut tl_object,
    ) {
        let ref mut fresh8 = (*in_0).env;
        *fresh8 = tl_env_set_local(
            in_0,
            (*in_0).env,
            key,
            if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
        );
        let ref mut fresh9 = (*in_0).values;
        *fresh9 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "513:1"]
    pub unsafe extern "C" fn tl_cf_all_objects(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cur = (*in_0).top_alloc;
        let mut res = TL_EMPTY_LIST as *mut tl_object;
        while !cur.is_null() {
            res = tl_new_pair(in_0, cur, res);
            cur = ((*cur).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
                as *mut tl_object;
        }
        let ref mut fresh10 = (*in_0).values;
        *fresh10 = tl_new_pair(in_0, tl_new_pair(in_0, res, (*in_0).false_), (*in_0).values);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "500:1"]
    pub static mut init_tl_cf_rescue: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_rescue
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-rescue\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn tl_cf_prefix(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut prefix = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut name = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        let ref mut fresh11 = (*in_0).prefixes;
        *fresh11 = tl_new_pair(in_0, tl_new_pair(in_0, prefix, name), (*in_0).prefixes);
        let ref mut fresh12 = (*in_0).values;
        *fresh12 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "172:1"]
    pub unsafe extern "C" fn tl_cf_set(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut key = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut val = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad set arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh13 = (*in_0).error;
                *fresh13 = tl_new_sym(in_0, b"bad set arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!key.is_null() && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"set! on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
                );
            } else {
                let ref mut fresh14 = (*in_0).error;
                *fresh14 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"set! on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
                );
            };
            let ref mut fresh15 = (*in_0).values;
            *fresh15 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        _tl_eval_and_then(
            in_0,
            val,
            key,
            Some(
                _tl_cf_set_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_eval_and_then:_tl_cf_set_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "167:1"]
    pub unsafe extern "C" fn _tl_cf_set_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut key: *mut tl_object,
    ) {
        let ref mut fresh16 = (*in_0).env;
        *fresh16 = tl_env_set_global(
            in_0,
            (*in_0).env,
            key,
            if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
        );
        let ref mut fresh17 = (*in_0).values;
        *fresh17 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "161:1"]
    pub unsafe extern "C" fn tl_cf_if(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cond = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if tl_list_len(args) < 3 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad if arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh18 = (*in_0).error;
                *fresh18 = tl_new_sym(in_0, b"bad if arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        _tl_eval_and_then(
            in_0,
            cond,
            if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            },
            Some(
                _tl_cf_if_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_eval_and_then:_tl_cf_if_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn _tl_cf_if_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut branches: *mut tl_object,
    ) {
        let mut ift = if !branches.is_null()
            && (branches.is_null()
                || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*branches).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut iff = if !(if !branches.is_null()
            && (branches.is_null()
                || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*branches).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !branches.is_null()
                && (branches.is_null()
                    || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*branches).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !branches.is_null()
                    && (branches.is_null()
                        || (*branches).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*branches).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !branches.is_null()
                && (branches.is_null()
                    || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*branches).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if _unboolify(
            in_0,
            if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
        ) != 0
        {
            tl_push_eval(in_0, ift, (*in_0).env);
        } else {
            tl_push_eval(in_0, iff, (*in_0).env);
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "10:1"]
    pub unsafe extern "C" fn _unboolify(
        mut in_0: *mut tl_interp,
        mut obj: *mut tl_object,
    ) -> libc::c_int {
        if obj.is_null() {
            return 0 as libc::c_int;
        }
        if obj == (*in_0).false_ {
            return 0 as libc::c_int;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint {
            return (*obj).c2rust_unnamed.ival as libc::c_int;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint {
            return ((*(*obj).c2rust_unnamed.nm).here.len > 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    #[no_mangle]
    #[c2rust::src_loc = "500:1"]
    pub unsafe extern "C" fn tl_cf_rescue(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut func = 0 as *mut tl_object;
        let mut cont = 0 as *mut tl_object;
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad rescue arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh19 = (*in_0).error;
                *fresh19 = tl_new_sym(
                    in_0,
                    b"bad rescue arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        func = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if !(!func.is_null()
            && (*func).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
            || !func.is_null()
                && (*func).kind as libc::c_uint == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
            || !func.is_null()
                && (*func).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
            || !func.is_null()
                && (*func).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
            || !func.is_null()
                && (*func).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint
            || !func.is_null()
                && (*func).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"rescue on non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    func,
                );
            } else {
                let ref mut fresh20 = (*in_0).error;
                *fresh20 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"rescue on non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    func,
                );
            };
            let ref mut fresh21 = (*in_0).values;
            *fresh21 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        cont = tl_new_cont(in_0, (*in_0).env, (*in_0).conts, (*in_0).values);
        let ref mut fresh22 = (*in_0).rescue;
        *fresh22 = tl_new_pair(in_0, cont, (*in_0).rescue);
        tl_push_apply(
            in_0,
            TL_APPLY_DROP_RESCUE as libc::c_long,
            TL_EMPTY_LIST as *mut tl_object,
            TL_EMPTY_LIST as *mut tl_object,
        );
        tl_push_apply(in_0, 0 as libc::c_int as libc::c_long, func, (*in_0).env);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "479:1"]
    pub static mut init_tl_cf_load_mod: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_load_mod
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-load-mod\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "479:1"]
    pub unsafe extern "C" fn tl_cf_load_mod(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut name = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut name_cstr = 0 as *mut libc::c_char;
        let mut ret = 0 as *mut tl_object;
        if !(!name.is_null()
            && (*name).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            let ref mut fresh23 = (*in_0).values;
            *fresh23 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        name_cstr = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ((*(*name).c2rust_unnamed.nm).here.len).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if !name_cstr.is_null() {
        } else {
            __assert_fail(
                b"name_cstr\0" as *const u8 as *const libc::c_char,
                b"./builtin.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                    b"void tl_cf_load_mod(tl_interp *, tl_object *, tl_object *)\0",
                ))
                .as_ptr(),
            );
        }
        memcpy(
            name_cstr as *mut libc::c_void,
            (*(*name).c2rust_unnamed.nm).here.data as *const libc::c_void,
            (*(*name).c2rust_unnamed.nm).here.len,
        );
        *name_cstr.offset((*(*name).c2rust_unnamed.nm).here.len as isize) =
            0 as libc::c_int as libc::c_char;
        ret = if ((*in_0).modloadf).expect("non-null function pointer")(in_0, name_cstr) != 0 {
            (*in_0).true_
        } else {
            (*in_0).false_
        };
        free(name_cstr as *mut libc::c_void);
        let ref mut fresh24 = (*in_0).values;
        *fresh24 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "475:1"]
    pub static mut init_tl_cf_read: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_read
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-read\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "475:1"]
    pub unsafe extern "C" fn tl_cf_read(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_read(in_0);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "470:1"]
    pub static mut init_tl_cf_gc: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_gc
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-gc\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "470:1"]
    pub unsafe extern "C" fn tl_cf_gc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_gc(in_0);
        let ref mut fresh25 = (*in_0).values;
        *fresh25 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "457:1"]
    pub static mut init_tl_cf_apply: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_apply
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-apply\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "457:1"]
    pub unsafe extern "C" fn tl_cf_apply(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cur = 0 as *mut tl_object;
        let mut len: size_t = 0;
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad apply arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh26 = (*in_0).error;
                *fresh26 = tl_new_sym(
                    in_0,
                    b"bad apply arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        cur = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        len = 0 as libc::c_int as size_t;
        while !cur.is_null() {
            let ref mut fresh27 = (*in_0).values;
            *fresh27 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !cur.is_null()
                        && (cur.is_null()
                            || (*cur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*cur).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            cur = (if !cur.is_null()
                && (cur.is_null()
                    || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cur).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            len = len.wrapping_add(1);
        }
        let ref mut fresh28 = (*in_0).values;
        *fresh28 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
        tl_push_apply(
            in_0,
            TL_APPLY_INDIRECT as libc::c_long,
            tl_new_int(in_0, len as libc::c_long),
            (*in_0).env,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "440:1"]
    pub static mut init_tl_cf_type: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_type
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-type\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "440:1"]
    pub unsafe extern "C" fn tl_cf_type(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut obj = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if !((*in_0).error).is_null() {
            let ref mut fresh29 = (*in_0).values;
            *fresh29 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint {
            let ref mut fresh30 = (*in_0).values;
            *fresh30 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"int\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint {
            let ref mut fresh31 = (*in_0).values;
            *fresh31 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"sym\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
        {
            let ref mut fresh32 = (*in_0).values;
            *fresh32 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"cfunc\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null()
            && (*obj).kind as libc::c_uint == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
        {
            let ref mut fresh33 = (*in_0).values;
            *fresh33 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"cfunc_byval\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint {
            let ref mut fresh34 = (*in_0).values;
            *fresh34 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"func\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
        {
            let ref mut fresh35 = (*in_0).values;
            *fresh35 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"macro\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !obj.is_null() && (*obj).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint {
            let ref mut fresh36 = (*in_0).values;
            *fresh36 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"cont\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if obj.is_null() || (*obj).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint {
            let ref mut fresh37 = (*in_0).values;
            *fresh37 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"pair\0" as *const u8 as *const libc::c_char),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh38 = (*in_0).values;
        *fresh38 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"unknown\0" as *const u8 as *const libc::c_char),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "435:1"]
    pub static mut init_tl_cf_nand: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_nand
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-nand\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "435:1"]
    pub unsafe extern "C" fn tl_cf_nand(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = _unboolify(
            in_0,
            if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
        );
        let mut b = _unboolify(
            in_0,
            if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .c2rust_unnamed
                .c2rust_unnamed
                .first
            } else {
                NULL as *mut tl_object_s
            },
        );
        let ref mut fresh39 = (*in_0).values;
        *fresh39 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !(a != 0 && b != 0) {
                    (*in_0).true_
                } else {
                    (*in_0).false_
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "423:1"]
    pub static mut init_tl_cf_less: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_less
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-<\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "423:1"]
    pub unsafe extern "C" fn tl_cf_less(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut b = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if !a.is_null()
            && (*a).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
            && (!b.is_null() && (*b).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            let ref mut fresh40 = (*in_0).values;
            *fresh40 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if (*a).c2rust_unnamed.ival < (*b).c2rust_unnamed.ival {
                        (*in_0).true_
                    } else {
                        (*in_0).false_
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !a.is_null()
            && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
            && (!b.is_null() && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            let ref mut fresh41 = (*in_0).values;
            *fresh41 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !a.is_null()
                        && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
                        && (!b.is_null()
                            && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && ((*(*a).c2rust_unnamed.nm).here.len < (*(*b).c2rust_unnamed.nm).here.len
                            && !((*(*b).c2rust_unnamed.nm).here.len
                                < (*(*a).c2rust_unnamed.nm).here.len)
                            || memcmp(
                                (*(*a).c2rust_unnamed.nm).here.data as *const libc::c_void,
                                (*(*b).c2rust_unnamed.nm).here.data as *const libc::c_void,
                                (*(*a).c2rust_unnamed.nm).here.len,
                            ) < 0 as libc::c_int)
                    {
                        (*in_0).true_
                    } else {
                        (*in_0).false_
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !((*in_0).error).is_null() {
            tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"incomparable types\0" as *const u8 as *const libc::c_char,
                    ),
                    a,
                ),
                b,
            );
        } else {
            let ref mut fresh42 = (*in_0).error;
            *fresh42 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"incomparable types\0" as *const u8 as *const libc::c_char,
                    ),
                    a,
                ),
                b,
            );
        };
        let ref mut fresh43 = (*in_0).values;
        *fresh43 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "412:1"]
    pub static mut init_tl_cf_eq: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_eq
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-=\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "412:1"]
    pub unsafe extern "C" fn tl_cf_eq(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut b = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if !a.is_null()
            && (*a).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
            && (!b.is_null() && (*b).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            let ref mut fresh44 = (*in_0).values;
            *fresh44 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if (*a).c2rust_unnamed.ival == (*b).c2rust_unnamed.ival {
                        (*in_0).true_
                    } else {
                        (*in_0).false_
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        if !a.is_null()
            && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
            && (!b.is_null() && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            let ref mut fresh45 = (*in_0).values;
            *fresh45 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !a.is_null()
                        && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
                        && (!b.is_null()
                            && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && (*a).c2rust_unnamed.nm == (*b).c2rust_unnamed.nm
                    {
                        (*in_0).true_
                    } else {
                        (*in_0).false_
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh46 = (*in_0).values;
        *fresh46 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if a == b {
                    (*in_0).true_
                } else {
                    (*in_0).false_
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "397:1"]
    pub static mut init_tl_cf_mod: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_mod
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-%\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "397:1"]
    pub unsafe extern "C" fn tl_cf_mod(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut phase = 0 as libc::c_int as libc::c_long;
        let mut res = 1 as libc::c_int as libc::c_long;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh47 = (*in_0).error;
                    *fresh47 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh48 = (*in_0).values;
                *fresh48 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res *= (*val).c2rust_unnamed.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res %= (*val).c2rust_unnamed.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        let ref mut fresh49 = (*in_0).values;
        *fresh49 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "382:1"]
    pub static mut init_tl_cf_div: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_div
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-/\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "382:1"]
    pub unsafe extern "C" fn tl_cf_div(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut phase = 0 as libc::c_int as libc::c_long;
        let mut res = 1 as libc::c_int as libc::c_long;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh50 = (*in_0).error;
                    *fresh50 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh51 = (*in_0).values;
                *fresh51 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res *= (*val).c2rust_unnamed.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res /= (*val).c2rust_unnamed.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        let ref mut fresh52 = (*in_0).values;
        *fresh52 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "373:1"]
    pub static mut init_tl_cf_mul: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_mul
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-*\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "373:1"]
    pub unsafe extern "C" fn tl_cf_mul(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut res = 1 as libc::c_int as libc::c_long;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh53 = (*in_0).error;
                    *fresh53 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh54 = (*in_0).values;
                *fresh54 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            res *= (*val).c2rust_unnamed.ival;
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        let ref mut fresh55 = (*in_0).values;
        *fresh55 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "358:1"]
    pub static mut init_tl_cf_sub: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_sub
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl--\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "358:1"]
    pub unsafe extern "C" fn tl_cf_sub(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut phase = 0 as libc::c_int as libc::c_long;
        let mut res = 0 as libc::c_int as libc::c_long;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh56 = (*in_0).error;
                    *fresh56 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh57 = (*in_0).values;
                *fresh57 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res += (*val).c2rust_unnamed.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res -= (*val).c2rust_unnamed.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        let ref mut fresh58 = (*in_0).values;
        *fresh58 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "37:1"]
    pub unsafe extern "C" fn tl_cf_error(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if !args.is_null() {
            if !((*in_0).error).is_null() {
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                } else {
                };
            } else {
                let ref mut fresh59 = (*in_0).error;
                *fresh59 = if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                };
            };
            let ref mut fresh60 = (*in_0).values;
            *fresh60 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        } else {
            let ref mut fresh61 = (*in_0).values;
            *fresh61 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).error, (*in_0).false_),
                (*in_0).values,
            );
            return;
        };
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "37:1"]
    pub static mut init_tl_cf_error: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_error
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-error\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "46:1"]
    pub static mut init_tl_cf_macro: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_macro
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-macro\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "57:1"]
    pub static mut init_tl_cf_lambda: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_lambda
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-lambda\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "68:1"]
    pub static mut init_tl_cf_define: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_define
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-define\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "78:1"]
    pub unsafe extern "C" fn tl_cf_display(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut l_arg = args;
        let mut arg = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_arg.is_null() {
            tl_print(in_0, arg);
            if !if !l_arg.is_null()
                && (l_arg.is_null()
                    || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_arg).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            }
            .is_null()
            {
                ((*in_0).writef).expect("non-null function pointer")(in_0, (*in_0).disp_sep);
            }
            l_arg = (if !l_arg.is_null()
                && (l_arg.is_null()
                    || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_arg).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            arg = (if !l_arg.is_null()
                && (l_arg.is_null()
                    || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_arg).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        tl_printf(in_0, b"\n\0" as *const u8 as *const libc::c_char);
        let ref mut fresh62 = (*in_0).values;
        *fresh62 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "78:1"]
    pub static mut init_tl_cf_display: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_display
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-display\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn tl_cf_display_sep(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut arg = 0 as *mut tl_object;
        if args.is_null() {
            let ref mut fresh63 = (*in_0).values;
            *fresh63 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym_data(in_0, &mut (*in_0).disp_sep, 1 as libc::c_int as size_t),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        }
        arg = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if !(!arg.is_null() && (*arg).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-display-sep with non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    arg,
                );
            } else {
                let ref mut fresh64 = (*in_0).error;
                *fresh64 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-display-sep with non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    arg,
                );
            };
            let ref mut fresh65 = (*in_0).values;
            *fresh65 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if (*(*arg).c2rust_unnamed.nm).here.len == 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh66 = (*in_0).error;
                *fresh66 = tl_new_sym(
                    in_0,
                    b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
                );
            };
            let ref mut fresh67 = (*in_0).values;
            *fresh67 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        (*in_0).disp_sep =
            *((*(*arg).c2rust_unnamed.nm).here.data).offset(0 as libc::c_int as isize);
        let ref mut fresh68 = (*in_0).values;
        *fresh68 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "87:1"]
    pub static mut init_tl_cf_display_sep: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_display_sep
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-display-sep\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "105:1"]
    pub static mut init_tl_cf_prefix: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_prefix
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-prefix\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "112:1"]
    pub unsafe extern "C" fn tl_cf_evalin(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if tl_list_len(args) < 3 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad eval-in& arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh69 = (*in_0).error;
                *fresh69 = tl_new_sym(
                    in_0,
                    b"bad eval-in& arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let mut env = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut expr = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        let mut k = if !(if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            }))
            .c2rust_unnamed
            .c2rust_unnamed
            .next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, k, (*in_0).env);
        tl_push_eval(in_0, expr, env);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "112:1"]
    pub static mut init_tl_cf_evalin: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_evalin
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-eval-in&\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "125:1"]
    pub unsafe extern "C" fn tl_cf_call_with_current_continuation(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad call-with-current-continuation arity\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                let ref mut fresh70 = (*in_0).error;
                *fresh70 = tl_new_sym(
                    in_0,
                    b"bad call-with-current-continuation arity\0" as *const u8
                        as *const libc::c_char,
                );
            };
            return;
        }
        let mut cont = tl_new_cont(in_0, (*in_0).env, (*in_0).conts, (*in_0).values);
        tl_push_apply(
            in_0,
            1 as libc::c_int as libc::c_long,
            if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
            (*in_0).env,
        );
        let ref mut fresh71 = (*in_0).values;
        *fresh71 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, cont, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "125:1"]
    pub static mut init_tl_cf_call_with_current_continuation: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_call_with_current_continuation
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-call-with-current-continuation\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "132:1"]
    pub unsafe extern "C" fn tl_cf_cons(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad cons arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh72 = (*in_0).error;
                *fresh72 = tl_new_sym(
                    in_0,
                    b"bad cons arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let ref mut fresh73 = (*in_0).values;
        *fresh73 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "132:1"]
    pub static mut init_tl_cf_cons: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_cons
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-cons\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn tl_cf_car(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad car arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh74 = (*in_0).error;
                *fresh74 = tl_new_sym(in_0, b"bad car arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let ref mut fresh75 = (*in_0).values;
        *fresh75 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .first
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "137:1"]
    pub static mut init_tl_cf_car: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_car
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-car\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "142:1"]
    pub unsafe extern "C" fn tl_cf_cdr(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad cdr arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh76 = (*in_0).error;
                *fresh76 = tl_new_sym(in_0, b"bad cdr arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let ref mut fresh77 = (*in_0).values;
        *fresh77 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .next
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "142:1"]
    pub static mut init_tl_cf_cdr: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_cdr
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-cdr\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "147:1"]
    pub unsafe extern "C" fn tl_cf_null(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad null? arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh78 = (*in_0).error;
                *fresh78 = tl_new_sym(
                    in_0,
                    b"bad null? arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let ref mut fresh79 = (*in_0).values;
        *fresh79 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }
                .is_null()
                {
                    (*in_0).true_
                } else {
                    (*in_0).false_
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "147:1"]
    pub static mut init_tl_cf_null: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_null
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-null?\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "161:1"]
    pub static mut init_tl_cf_if: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_if
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-if\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "172:1"]
    pub static mut init_tl_cf_set: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_set
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-set!\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "179:1"]
    pub unsafe extern "C" fn tl_cf_env(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut f = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if f.is_null() {
            let ref mut fresh80 = (*in_0).values;
            *fresh80 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).env, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !(!f.is_null() && (*f).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
            || !f.is_null() && (*f).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"env of non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    f,
                );
            } else {
                let ref mut fresh81 = (*in_0).error;
                *fresh81 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"env of non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    f,
                );
            };
            let ref mut fresh82 = (*in_0).values;
            *fresh82 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh83 = (*in_0).values;
        *fresh83 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                (*f).c2rust_unnamed.c2rust_unnamed_1.env,
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "179:1"]
    pub static mut init_tl_cf_env: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_env
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-env\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "191:1"]
    pub unsafe extern "C" fn tl_cf_setenv(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut first = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut next = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if next.is_null() {
            let ref mut fresh84 = (*in_0).env;
            *fresh84 = first;
            let ref mut fresh85 = (*in_0).values;
            *fresh85 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !(!first.is_null()
            && (*first).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
            || !first.is_null()
                && (*first).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"setenv on non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    first,
                );
            } else {
                let ref mut fresh86 = (*in_0).error;
                *fresh86 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"setenv on non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    first,
                );
            };
            let ref mut fresh87 = (*in_0).values;
            *fresh87 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh88 = (*first).c2rust_unnamed.c2rust_unnamed_1.env;
        *fresh88 = next;
        let ref mut fresh89 = (*in_0).values;
        *fresh89 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "191:1"]
    pub static mut init_tl_cf_setenv: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_setenv
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-set-env!\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "205:1"]
    pub unsafe extern "C" fn tl_cf_topenv(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let ref mut fresh90 = (*in_0).values;
        *fresh90 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).top_env, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "205:1"]
    pub static mut init_tl_cf_topenv: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_topenv
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-top-env\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "209:1"]
    pub unsafe extern "C" fn tl_cf_concat(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut buffer = 0 as *mut libc::c_char;
        let mut end = 0 as *mut libc::c_char;
        let mut src = 0 as *mut libc::c_char;
        let mut sz = 0 as libc::c_int as size_t;
        let mut rsz: size_t = 0;
        let mut ret = 0 as *mut tl_object;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
            {
                let mut buf = 0 as *mut libc::c_char;
                let mut sz_0: libc::c_int = 0;
                let mut sm = 0 as *mut tl_object;
                sz_0 = snprintf(
                    NULL as *mut libc::c_char,
                    0 as libc::c_int as libc::c_ulong,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*val).c2rust_unnamed.ival,
                );
                if sz_0 > 0 as libc::c_int {
                } else {
                    __assert_fail(
                        b"sz > 0\0" as *const u8 as *const libc::c_char,
                        b"./builtin.c\0" as *const u8 as *const libc::c_char,
                        221 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                            b"void tl_cf_concat(tl_interp *, tl_object *, tl_object *)\0",
                        ))
                        .as_ptr(),
                    );
                }
                buf = ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    0 as *mut libc::c_void,
                    (sz_0 + 1 as libc::c_int) as size_t,
                ) as *mut libc::c_char;
                if !buf.is_null() {
                } else {
                    __assert_fail(
                        b"buf\0" as *const u8 as *const libc::c_char,
                        b"./builtin.c\0" as *const u8 as *const libc::c_char,
                        223 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                            b"void tl_cf_concat(tl_interp *, tl_object *, tl_object *)\0",
                        ))
                        .as_ptr(),
                    );
                }
                snprintf(
                    buf,
                    (sz_0 + 1 as libc::c_int) as libc::c_ulong,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*val).c2rust_unnamed.ival,
                );
                val = tl_new_sym(in_0, buf);
                free(buf as *mut libc::c_void);
                let ref mut fresh91 = (*l_val).c2rust_unnamed.c2rust_unnamed.first;
                *fresh91 = val;
            }
            if !val.is_null()
                && (*val).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
            {
                sz = (sz as libc::c_ulong).wrapping_add((*(*val).c2rust_unnamed.nm).here.len)
                    as size_t as size_t;
            } else {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"concat on non-sym or int\0" as *const u8 as *const libc::c_char,
                        ),
                        val,
                    );
                } else {
                    let ref mut fresh92 = (*in_0).error;
                    *fresh92 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"concat on non-sym or int\0" as *const u8 as *const libc::c_char,
                        ),
                        val,
                    );
                };
                let ref mut fresh93 = (*in_0).values;
                *fresh93 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        rsz = sz;
        buffer = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            sz,
        ) as *mut libc::c_char;
        end = buffer;
        if buffer.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh94 = (*in_0).error;
                *fresh94 = tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let mut l_val_0 = args;
        let mut val_0 = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val_0.is_null() {
            src = (*(*val_0).c2rust_unnamed.nm).here.data;
            sz = (*(*val_0).c2rust_unnamed.nm).here.len;
            while sz > 0 as libc::c_int as libc::c_ulong {
                let fresh95 = src;
                src = src.offset(1);
                let fresh96 = end;
                end = end.offset(1);
                *fresh96 = *fresh95;
                sz = sz.wrapping_sub(1);
            }
            l_val_0 = (if !l_val_0.is_null()
                && (l_val_0.is_null()
                    || (*l_val_0).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val_0).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val_0 = (if !l_val_0.is_null()
                && (l_val_0.is_null()
                    || (*l_val_0).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val_0).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        ret = tl_new_sym_data(in_0, buffer, rsz);
        free(buffer as *mut libc::c_void);
        let ref mut fresh97 = (*in_0).values;
        *fresh97 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "209:1"]
    pub static mut init_tl_cf_concat: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_concat
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-concat\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn tl_cf_length(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad length arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh98 = (*in_0).error;
                *fresh98 = tl_new_sym(
                    in_0,
                    b"bad length arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        if !(!(if !args.is_null()
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
                == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"length on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh99 = (*in_0).error;
                *fresh99 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"length on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh100 = (*in_0).values;
            *fresh100 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh101 = (*in_0).values;
        *fresh101 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_int(
                    in_0,
                    (*(*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .c2rust_unnamed
                    .nm)
                        .here
                        .len as libc::c_long,
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "255:1"]
    pub static mut init_tl_cf_length: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_length
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-length\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "261:1"]
    pub unsafe extern "C" fn tl_cf_ord(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad ord arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh102 = (*in_0).error;
                *fresh102 =
                    tl_new_sym(in_0, b"bad ord arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!(if !args.is_null()
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
                == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh103 = (*in_0).error;
                *fresh103 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh104 = (*in_0).values;
            *fresh104 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !(!(if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            }))
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .first
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
                        b"ord on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh105 = (*in_0).error;
                *fresh105 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh106 = (*in_0).values;
            *fresh106 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if (*(if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            }))
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        }))
        .c2rust_unnamed
        .ival as libc::c_ulong
            >= (*(*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            }))
            .c2rust_unnamed
            .nm)
                .here
                .len
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord index out of range\0" as *const u8 as *const libc::c_char,
                    ),
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.next
                                } else {
                                    0 as *mut tl_object_s
                                }))
                                .kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .first
                        } else {
                            0 as *mut tl_object_s
                        },
                        tl_new_int(
                            in_0,
                            (*(*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .nm)
                                .here
                                .len as libc::c_long,
                        ),
                    ),
                );
            } else {
                let ref mut fresh107 = (*in_0).error;
                *fresh107 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord index out of range\0" as *const u8 as *const libc::c_char,
                    ),
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.next
                                } else {
                                    0 as *mut tl_object_s
                                }))
                                .kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .first
                        } else {
                            0 as *mut tl_object_s
                        },
                        tl_new_int(
                            in_0,
                            (*(*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .nm)
                                .here
                                .len as libc::c_long,
                        ),
                    ),
                );
            };
            return;
        }
        let ref mut fresh108 = (*in_0).values;
        *fresh108 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_int(
                    in_0,
                    *((*(*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .c2rust_unnamed
                    .nm)
                        .here
                        .data)
                        .offset(
                            (*if !(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                && ((if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.next
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .is_null()
                                    || (*(if !args.is_null()
                                        && (args.is_null()
                                            || (*args).kind as libc::c_uint
                                                == TL_PAIR as libc::c_int as libc::c_uint)
                                    {
                                        (*args).c2rust_unnamed.c2rust_unnamed.next
                                    } else {
                                        0 as *mut tl_object_s
                                    }))
                                    .kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.next
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .ival as isize,
                        ) as libc::c_long,
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "261:1"]
    pub static mut init_tl_cf_ord: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_ord
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-ord\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "272:1"]
    pub unsafe extern "C" fn tl_cf_chr(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut s: [libc::c_char; 2] = [0, 0];
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad chr arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh109 = (*in_0).error;
                *fresh109 =
                    tl_new_sym(in_0, b"bad chr arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!(if !args.is_null()
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
                        b"chr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh110 = (*in_0).error;
                *fresh110 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"chr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh111 = (*in_0).values;
            *fresh111 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        s[0 as libc::c_int as usize] = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_char;
        let ref mut fresh112 = (*in_0).values;
        *fresh112 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_sym(in_0, s.as_mut_ptr()), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "272:1"]
    pub static mut init_tl_cf_chr: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_chr
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-chr\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "280:1"]
    pub unsafe extern "C" fn tl_cf_substr(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut sym = 0 as *mut tl_object;
        let mut start = 0 as *mut tl_object;
        let mut sidx: libc::c_long = 0;
        let mut eidx: libc::c_long = 0;
        let mut buf = 0 as *mut libc::c_char;
        if tl_list_len(args) < 2 as libc::c_int as libc::c_ulong {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad substr arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh113 = (*in_0).error;
                *fresh113 = tl_new_sym(
                    in_0,
                    b"bad substr arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        sym = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        if !(!sym.is_null() && (*sym).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    sym,
                );
            } else {
                let ref mut fresh114 = (*in_0).error;
                *fresh114 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    sym,
                );
            };
            let ref mut fresh115 = (*in_0).values;
            *fresh115 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        start = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        if !(!start.is_null()
            && (*start).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    start,
                );
            } else {
                let ref mut fresh116 = (*in_0).error;
                *fresh116 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    start,
                );
            };
            let ref mut fresh117 = (*in_0).values;
            *fresh117 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        sidx = (*start).c2rust_unnamed.ival;
        if !if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .next
        } else {
            NULL as *mut tl_object_s
        }
        .is_null()
        {
            start = if !(if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .next
                } else {
                    0 as *mut tl_object_s
                })
                .c2rust_unnamed
                .c2rust_unnamed
                .first
            } else {
                NULL as *mut tl_object_s
            };
            if !(!start.is_null()
                && (*start).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"substr on non-int\0" as *const u8 as *const libc::c_char,
                        ),
                        start,
                    );
                } else {
                    let ref mut fresh118 = (*in_0).error;
                    *fresh118 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"substr on non-int\0" as *const u8 as *const libc::c_char,
                        ),
                        start,
                    );
                };
                let ref mut fresh119 = (*in_0).values;
                *fresh119 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            eidx = (*start).c2rust_unnamed.ival;
        } else {
            eidx = (*(*sym).c2rust_unnamed.nm).here.len as libc::c_long;
        }
        if sidx < 0 as libc::c_int as libc::c_long {
            sidx = (sidx as libc::c_ulong).wrapping_add((*(*sym).c2rust_unnamed.nm).here.len)
                as libc::c_long as libc::c_long;
            if sidx < 0 as libc::c_int as libc::c_long {
                sidx = 0 as libc::c_int as libc::c_long;
            }
        }
        if eidx < 0 as libc::c_int as libc::c_long {
            eidx = (eidx as libc::c_ulong).wrapping_add((*(*sym).c2rust_unnamed.nm).here.len)
                as libc::c_long as libc::c_long;
            if eidx < 0 as libc::c_int as libc::c_long {
                eidx = 0 as libc::c_int as libc::c_long;
            }
        }
        if sidx as libc::c_ulong >= (*(*sym).c2rust_unnamed.nm).here.len {
            sidx = ((*(*sym).c2rust_unnamed.nm).here.len)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_long;
        }
        if eidx as libc::c_ulong > (*(*sym).c2rust_unnamed.nm).here.len {
            eidx = (*(*sym).c2rust_unnamed.nm).here.len as libc::c_long;
        }
        if sidx >= eidx {
            sidx = eidx;
        }
        let ref mut fresh120 = (*in_0).values;
        *fresh120 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_sym_data(
                    in_0,
                    ((*(*sym).c2rust_unnamed.nm).here.data).offset(sidx as isize),
                    (eidx - sidx) as size_t,
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "280:1"]
    pub static mut init_tl_cf_substr: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_substr
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-substr\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "331:1"]
    pub unsafe extern "C" fn tl_cf_readc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        _tl_getc_and_then(
            in_0,
            0 as *mut tl_object,
            Some(
                _tl_readc_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_getc_and_then:_tl_readc_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "331:1"]
    pub static mut init_tl_cf_readc: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_readc
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-readc\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "335:1"]
    pub unsafe extern "C" fn tl_cf_putbackc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad putback arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh121 = (*in_0).error;
                *fresh121 = tl_new_sym(
                    in_0,
                    b"bad putback arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        if !(!(if !args.is_null()
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
                        b"putback on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh122 = (*in_0).error;
                *fresh122 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"putback on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh123 = (*in_0).values;
            *fresh123 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        (*in_0).is_putback = 1 as libc::c_int;
        (*in_0).putback = (*(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        }))
        .c2rust_unnamed
        .ival as libc::c_int;
        let ref mut fresh124 = (*in_0).values;
        *fresh124 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "335:1"]
    pub static mut init_tl_cf_putbackc: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_putbackc
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-putbackc\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "342:1"]
    pub unsafe extern "C" fn tl_cf_writec(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad write arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh125 = (*in_0).error;
                *fresh125 = tl_new_sym(
                    in_0,
                    b"bad write arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        if !(!(if !args.is_null()
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
                        b"write on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh126 = (*in_0).error;
                *fresh126 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"write on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh127 = (*in_0).values;
            *fresh127 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ((*in_0).writef).expect("non-null function pointer")(
            in_0,
            (*if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .ival as libc::c_char,
        );
        let ref mut fresh128 = (*in_0).values;
        *fresh128 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "342:1"]
    pub static mut init_tl_cf_writec: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_writec
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-writec\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "349:1"]
    pub unsafe extern "C" fn tl_cf_add(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut res = 0 as libc::c_int as libc::c_long;
        let mut l_val = args;
        let mut val = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh129 = (*in_0).error;
                    *fresh129 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh130 = (*in_0).values;
                *fresh130 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            res += (*val).c2rust_unnamed.ival;
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        let ref mut fresh131 = (*in_0).values;
        *fresh131 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "349:1"]
    pub static mut init_tl_cf_add: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_add
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-+\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
    use super::assert_h::__assert_fail;
    use super::env_c::{tl_env_set_global, tl_env_set_local};
    use super::eval_c::{_tl_eval_and_then, _tl_getc_and_then, tl_push_apply, tl_push_eval};
    use super::object_c::{
        tl_gc, tl_list_len, tl_new_cont, tl_new_int, tl_new_macro, tl_new_pair, tl_new_sym,
        tl_new_sym_data,
    };
    use super::print_c::{tl_print, tl_printf};
    use super::read_c::tl_read;
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::snprintf;
    use super::stdlib_h::free;
    use super::string_h::{memcmp, memcpy};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_APPLY_DROP_RESCUE,
        TL_APPLY_INDIRECT, TL_CFUNC, TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FMASK, TL_FUNC,
        TL_INT, TL_MACRO, TL_PAIR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/env.c:2"]
pub mod env_c {
    #[no_mangle]
    #[c2rust::src_loc = "6:1"]
    pub unsafe extern "C" fn tl_env_get_kv(
        mut in_0: *mut tl_interp,
        mut env: *mut tl_object,
        mut nm: *mut tl_object,
    ) -> *mut tl_object {
        let mut l_frame = env;
        let mut frame = if !env.is_null()
            && (env.is_null()
                || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*env).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_frame.is_null() {
            let mut l_kv = frame;
            let mut kv = if !frame.is_null()
                && (frame.is_null()
                    || (*frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*frame).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            };
            while !l_kv.is_null() {
                let mut key = if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                };
                let mut val = if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                };
                if !key.is_null()
                    && (!key.is_null()
                        && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                    && (!key.is_null()
                        && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
                        && (!nm.is_null()
                            && (*nm).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && (*key).c2rust_unnamed.nm == (*nm).c2rust_unnamed.nm)
                {
                    return kv;
                }
                l_kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                });
                kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                });
            }
            l_frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return NULL as *mut tl_object;
    }
    #[no_mangle]
    #[c2rust::src_loc = "19:1"]
    pub unsafe extern "C" fn tl_env_set_global(
        mut in_0: *mut tl_interp,
        mut env: *mut tl_object,
        mut nm: *mut tl_object,
        mut val: *mut tl_object,
    ) -> *mut tl_object {
        let mut kv = tl_env_get_kv(in_0, env, nm);
        if !kv.is_null()
            && (kv.is_null()
                || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            let ref mut fresh132 = (*kv).c2rust_unnamed.c2rust_unnamed.next;
            *fresh132 = val;
            return env;
        }
        if env.is_null() {
            env = tl_new_pair(in_0, TL_EMPTY_LIST as *mut tl_object, env);
        }
        let mut l_frame = env;
        let mut frame = if !env.is_null()
            && (env.is_null()
                || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*env).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_frame.is_null() {
            if if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            }
            .is_null()
            {
                let ref mut fresh133 = (*l_frame).c2rust_unnamed.c2rust_unnamed.first;
                *fresh133 = tl_frm_set(in_0, frame, nm, val);
            }
            l_frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return env;
    }
    #[no_mangle]
    #[c2rust::src_loc = "36:1"]
    pub unsafe extern "C" fn tl_env_set_local(
        mut in_0: *mut tl_interp,
        mut env: *mut tl_object,
        mut nm: *mut tl_object,
        mut val: *mut tl_object,
    ) -> *mut tl_object {
        if env.is_null() {
            env = tl_new_pair(in_0, TL_EMPTY_LIST as *mut tl_object, env);
        }
        let ref mut fresh134 = (*env).c2rust_unnamed.c2rust_unnamed.first;
        *fresh134 = tl_frm_set(
            in_0,
            if !env.is_null()
                && (env.is_null()
                    || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*env).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
            nm,
            val,
        );
        return env;
    }
    #[no_mangle]
    #[c2rust::src_loc = "44:1"]
    pub unsafe extern "C" fn tl_frm_set(
        mut in_0: *mut tl_interp,
        mut frm: *mut tl_object,
        mut nm: *mut tl_object,
        mut val: *mut tl_object,
    ) -> *mut tl_object {
        let mut l_kv = frm;
        let mut kv = if !frm.is_null()
            && (frm.is_null()
                || (*frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*frm).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_kv.is_null() {
            if !kv.is_null()
                && (kv.is_null()
                    || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                && (!(if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && (*(if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_SYM as libc::c_int as libc::c_uint)
                && (!(if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && (*(if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_SYM as libc::c_int as libc::c_uint
                    && (!nm.is_null()
                        && (*nm).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                    && (*(if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .c2rust_unnamed
                    .nm == (*nm).c2rust_unnamed.nm)
            {
                let ref mut fresh135 = (*kv).c2rust_unnamed.c2rust_unnamed.next;
                *fresh135 = val;
                return frm;
            }
            l_kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return tl_new_pair(in_0, tl_new_pair(in_0, nm, val), frm);
    }
    use super::object_c::tl_new_pair;
    use super::stddef_h::NULL;
    use super::tinylisp_h::{
        tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_EMPTY_LIST, TL_PAIR, TL_SYM,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/eval.c:3"]
pub mod eval_c {
    #[no_mangle]
    #[c2rust::src_loc = "30:1"]
    pub unsafe extern "C" fn tl_push_eval(
        mut in_0: *mut tl_interp,
        mut expr: *mut tl_object,
        mut env: *mut tl_object,
    ) -> libc::c_int {
        if !((*in_0).error).is_null() {
            return TL_RESULT_DONE;
        }
        if expr.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh136 = (*in_0).error;
                *fresh136 = tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
            };
            return TL_RESULT_DONE;
        }
        if !expr.is_null() && (*expr).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
            || (!expr.is_null()
                && (*expr).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
                || !expr.is_null()
                    && (*expr).kind as libc::c_uint
                        == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                || !expr.is_null()
                    && (*expr).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
                || !expr.is_null()
                    && (*expr).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
                || !expr.is_null()
                    && (*expr).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint
                || !expr.is_null()
                    && (*expr).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint)
        {
            let ref mut fresh137 = (*in_0).values;
            *fresh137 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, expr, (*in_0).false_),
                (*in_0).values,
            );
            return TL_RESULT_DONE;
        }
        if !expr.is_null() && (*expr).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
        {
            let mut binding = tl_env_get_kv(in_0, env, expr);
            if binding.is_null() {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                        expr,
                    );
                } else {
                    let ref mut fresh138 = (*in_0).error;
                    *fresh138 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                        expr,
                    );
                };
                return TL_RESULT_DONE;
            }
            let ref mut fresh139 = (*in_0).values;
            *fresh139 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !binding.is_null()
                        && (binding.is_null()
                            || (*binding).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*binding).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return TL_RESULT_DONE;
        }
        if expr.is_null() || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint
        {
            let mut len = tl_list_len(expr);
            let mut l_subex = expr;
            let mut subex = if !expr.is_null()
                && (expr.is_null()
                    || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*expr).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            };
            while !l_subex.is_null() {
                if subex
                    == (if !expr.is_null()
                        && (expr.is_null()
                            || (*expr).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*expr).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    })
                {
                    tl_push_apply(
                        in_0,
                        len as libc::c_long - 1 as libc::c_int as libc::c_long,
                        subex,
                        env,
                    );
                } else {
                    let ref mut fresh140 = (*in_0).values;
                    *fresh140 = tl_new_pair(
                        in_0,
                        tl_new_pair(in_0, subex, (*in_0).true_),
                        (*in_0).values,
                    );
                }
                l_subex = (if !l_subex.is_null()
                    && (l_subex.is_null()
                        || (*l_subex).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_subex).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                });
                subex = (if !l_subex.is_null()
                    && (l_subex.is_null()
                        || (*l_subex).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_subex).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                });
            }
            return TL_RESULT_AGAIN;
        }
        if !((*in_0).error).is_null() {
            tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
                expr,
            );
        } else {
            let ref mut fresh141 = (*in_0).error;
            *fresh141 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
                expr,
            );
        };
        return TL_RESULT_DONE;
    }
    #[no_mangle]
    #[c2rust::src_loc = "126:1"]
    pub unsafe extern "C" fn tl_push_apply(
        mut in_0: *mut tl_interp,
        mut len: libc::c_long,
        mut expr: *mut tl_object,
        mut env: *mut tl_object,
    ) {
        let ref mut fresh142 = (*in_0).conts;
        *fresh142 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, len), tl_new_pair(in_0, expr, env)),
            (*in_0).conts,
        );
        let ref mut fresh143 = (*in_0).ctr_events;
        *fresh143 = (*fresh143).wrapping_add(1);
        if (*in_0).gc_events > 0 as libc::c_int as libc::c_ulong
            && (*in_0).ctr_events >= (*in_0).gc_events
        {
            tl_gc(in_0);
            (*in_0).ctr_events = 0 as libc::c_int as size_t;
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn _tl_apply_next_body_callable_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut cont: *mut tl_object,
    ) {
        let mut callex = if !(if !cont.is_null()
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
        let mut env = if !(if !cont.is_null()
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
            .next
        } else {
            NULL as *mut tl_object_s
        };
        let mut frm = TL_EMPTY_LIST as *mut tl_object;
        if !callex.is_null()
            && (*callex).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
        {
            ((*callex).c2rust_unnamed.c2rust_unnamed_0.cfunc).expect("non-null function pointer")(
                in_0,
                args,
                (*callex).c2rust_unnamed.c2rust_unnamed_0.state,
            );
            return;
        }
        if ((*callex).c2rust_unnamed.c2rust_unnamed_1.args).is_null()
            || (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args).kind as libc::c_uint
                == TL_PAIR as libc::c_int as libc::c_uint
        {
            let mut is_improp = 0 as libc::c_int as libc::c_char;
            let mut paramlen = 0 as libc::c_int as libc::c_long;
            let mut l_item = (*callex).c2rust_unnamed.c2rust_unnamed_1.args;
            let mut item = if !((*callex).c2rust_unnamed.c2rust_unnamed_1.args).is_null()
                && (((*callex).c2rust_unnamed.c2rust_unnamed_1.args).is_null()
                    || (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .first
            } else {
                NULL as *mut tl_object_s
            };
            while !l_item.is_null() {
                paramlen += 1;
                if !(if !l_item.is_null()
                    && (l_item.is_null()
                        || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_item).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && (*(if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_SYM as libc::c_int as libc::c_uint
                {
                    is_improp = 1 as libc::c_int as libc::c_char;
                    break;
                } else {
                    l_item = (if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    });
                    item = (if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    });
                }
            }
            if if is_improp as libc::c_int != 0 {
                (tl_list_len(args) < paramlen as libc::c_ulong) as libc::c_int
            } else {
                (tl_list_len(args) != paramlen as libc::c_ulong) as libc::c_int
            } != 0
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            tl_new_sym(in_0, b"bad arity\0" as *const u8 as *const libc::c_char),
                            tl_new_int(in_0, paramlen),
                        ),
                        args,
                    );
                } else {
                    let ref mut fresh144 = (*in_0).error;
                    *fresh144 = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            tl_new_sym(in_0, b"bad arity\0" as *const u8 as *const libc::c_char),
                            tl_new_int(in_0, paramlen),
                        ),
                        args,
                    );
                };
                let ref mut fresh145 = (*in_0).values;
                *fresh145 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            let mut acur = (*callex).c2rust_unnamed.c2rust_unnamed_1.args;
            while !acur.is_null() && !args.is_null() {
                frm = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !acur.is_null()
                            && (acur.is_null()
                                || (*acur).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*acur).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            NULL as *mut tl_object_s
                        },
                        if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            NULL as *mut tl_object_s
                        },
                    ),
                    frm,
                );
                args = if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                };
                if !((if !acur.is_null()
                    && (acur.is_null()
                        || (*acur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*acur).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !acur.is_null()
                        && (acur.is_null()
                            || (*acur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*acur).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    frm = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            if !acur.is_null()
                                && (acur.is_null()
                                    || (*acur).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*acur).c2rust_unnamed.c2rust_unnamed.next
                            } else {
                                NULL as *mut tl_object_s
                            },
                            args,
                        ),
                        frm,
                    );
                    break;
                } else {
                    acur = if !acur.is_null()
                        && (acur.is_null()
                            || (*acur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*acur).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    };
                }
            }
        } else if !((*callex).c2rust_unnamed.c2rust_unnamed_1.args).is_null()
            && (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args).kind as libc::c_uint
                == TL_SYM as libc::c_int as libc::c_uint
        {
            frm = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*callex).c2rust_unnamed.c2rust_unnamed_1.args, args),
                frm,
            );
        } else {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"bad arg kind\0" as *const u8 as *const libc::c_char),
                    (*callex).c2rust_unnamed.c2rust_unnamed_1.args,
                );
            } else {
                let ref mut fresh146 = (*in_0).error;
                *fresh146 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"bad arg kind\0" as *const u8 as *const libc::c_char),
                    (*callex).c2rust_unnamed.c2rust_unnamed_1.args,
                );
            };
            let ref mut fresh147 = (*in_0).values;
            *fresh147 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !((*callex).c2rust_unnamed.c2rust_unnamed_1.envn).is_null() {
            frm = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*callex).c2rust_unnamed.c2rust_unnamed_1.envn, env),
                frm,
            );
        }
        env = tl_new_pair(in_0, frm, (*callex).c2rust_unnamed.c2rust_unnamed_1.env);
        let mut body_rvs = tl_list_rvs(in_0, (*callex).c2rust_unnamed.c2rust_unnamed_1.body);
        let mut l_ex = body_rvs;
        let mut ex = if !body_rvs.is_null()
            && (body_rvs.is_null()
                || (*body_rvs).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*body_rvs).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_ex.is_null() {
            tl_push_apply(
                in_0,
                (if ex
                    == (if !body_rvs.is_null()
                        && (body_rvs.is_null()
                            || (*body_rvs).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*body_rvs).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    })
                {
                    TL_APPLY_PUSH_EVAL
                } else {
                    TL_APPLY_DROP_EVAL
                }) as libc::c_long,
                ex,
                env,
            );
            l_ex = (if !l_ex.is_null()
                && (l_ex.is_null()
                    || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_ex).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            ex = (if !l_ex.is_null()
                && (l_ex.is_null()
                    || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_ex).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn tl_apply_next(mut in_0: *mut tl_interp) -> libc::c_int {
        let mut cont = if !((*in_0).conts).is_null()
            && (((*in_0).conts).is_null()
                || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        let mut len: libc::c_long = 0;
        let mut callex = 0 as *mut tl_object;
        let mut env = 0 as *mut tl_object;
        let mut args = TL_EMPTY_LIST as *mut tl_object;
        let mut res: libc::c_int = 0;
        if !((*in_0).error).is_null() {
            let mut rescue = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            };
            if rescue.is_null() {
                return TL_RESULT_DONE;
            }
            let ref mut fresh148 = (*in_0).rescue;
            *fresh148 = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            };
            tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, rescue, (*in_0).env);
            let ref mut fresh149 = (*in_0).values;
            *fresh149 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).error, (*in_0).false_),
                (*in_0).values,
            );
            let ref mut fresh150 = (*in_0).error;
            *fresh150 = NULL as *mut tl_object;
            return TL_RESULT_AGAIN;
        }
        let ref mut fresh151 = (*in_0).current;
        *fresh151 = cont;
        if cont.is_null() {
            return TL_RESULT_DONE;
        }
        let ref mut fresh152 = (*in_0).conts;
        *fresh152 = if !((*in_0).conts).is_null()
            && (((*in_0).conts).is_null()
                || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        };
        if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            }))
            .kind as libc::c_uint
                == TL_INT as libc::c_int as libc::c_uint
        {
        } else {
            __assert_fail(
                b"tl_is_int(tl_first(cont))\0" as *const u8 as *const libc::c_char,
                b"./eval.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"int tl_apply_next(tl_interp *)\0",
                ))
                .as_ptr(),
            );
        }
        len = (*if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival;
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
        env = if !(if !cont.is_null()
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
            .next
        } else {
            NULL as *mut tl_object_s
        };
        if len == TL_APPLY_DROP as libc::c_long {
            let ref mut fresh153 = (*in_0).values;
            *fresh153 = if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            };
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_DROP_RESCUE as libc::c_long {
            let ref mut fresh154 = (*in_0).rescue;
            *fresh154 = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            };
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_GETCHAR as libc::c_long {
            if (*in_0).is_putback != 0 {
                let ref mut fresh155 = (*in_0).values;
                *fresh155 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        tl_new_int(in_0, (*in_0).putback as libc::c_long),
                        (*in_0).false_,
                    ),
                    (*in_0).values,
                );
                (*in_0).is_putback = 0 as libc::c_int;
                return TL_RESULT_AGAIN;
            } else {
                return TL_RESULT_GETCHAR;
            }
        }
        if len != TL_APPLY_INDIRECT as libc::c_long {
            res = tl_push_eval(in_0, callex, env);
            if res != 0 {
                if !(len == TL_APPLY_PUSH_EVAL as libc::c_long
                    || len == TL_APPLY_DROP_EVAL as libc::c_long)
                {
                    cont = if !((*in_0).conts).is_null()
                        && (((*in_0).conts).is_null()
                            || (*(*in_0).conts).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    };
                    let ref mut fresh156 = (*in_0).conts;
                    *fresh156 = if !((*in_0).conts).is_null()
                        && (((*in_0).conts).is_null()
                            || (*(*in_0).conts).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    };
                    tl_push_apply(
                        in_0,
                        TL_APPLY_INDIRECT as libc::c_long,
                        tl_new_int(in_0, len),
                        env,
                    );
                    let ref mut fresh157 = (*in_0).conts;
                    *fresh157 = tl_new_pair(in_0, cont, (*in_0).conts);
                } else if len == TL_APPLY_DROP_EVAL as libc::c_long {
                    cont = if !((*in_0).conts).is_null()
                        && (((*in_0).conts).is_null()
                            || (*(*in_0).conts).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    };
                    let ref mut fresh158 = (*in_0).conts;
                    *fresh158 = if !((*in_0).conts).is_null()
                        && (((*in_0).conts).is_null()
                            || (*(*in_0).conts).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    };
                    tl_push_apply(
                        in_0,
                        TL_APPLY_DROP as libc::c_long,
                        TL_EMPTY_LIST as *mut tl_object,
                        TL_EMPTY_LIST as *mut tl_object,
                    );
                    let ref mut fresh159 = (*in_0).conts;
                    *fresh159 = tl_new_pair(in_0, cont, (*in_0).conts);
                }
                return res;
            }
        } else {
            len = (*if !(if !cont.is_null()
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
                            || (*cont).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
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
            })
            .c2rust_unnamed
            .ival;
        }
        callex = if !(if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !((*in_0).values).is_null()
                    && (((*in_0).values).is_null()
                        || (*(*in_0).values).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        let ref mut fresh160 = (*in_0).values;
        *fresh160 = if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        };
        if len == TL_APPLY_DROP_EVAL as libc::c_long {
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_PUSH_EVAL as libc::c_long {
            let ref mut fresh161 = (*in_0).values;
            *fresh161 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, callex, (*in_0).false_),
                (*in_0).values,
            );
            return TL_RESULT_AGAIN;
        }
        if !(!callex.is_null()
            && (*callex).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint
            || !callex.is_null()
                && (*callex).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"call non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    callex,
                );
            } else {
                let ref mut fresh162 = (*in_0).error;
                *fresh162 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"call non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    callex,
                );
            };
            return TL_RESULT_AGAIN;
        }
        let mut i = 0 as libc::c_int;
        while (i as libc::c_long) < len {
            args = tl_new_pair(
                in_0,
                if !((*in_0).values).is_null()
                    && (((*in_0).values).is_null()
                        || (*(*in_0).values).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                },
                args,
            );
            let ref mut fresh163 = (*in_0).values;
            *fresh163 = if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            };
            i += 1;
        }
        let ref mut fresh164 = (*in_0).env;
        *fresh164 = env;
        let mut new_args = TL_EMPTY_LIST as *mut tl_object;
        match (*callex).kind as libc::c_uint {
            7 | 5 => {
                _tl_eval_all_args(
                    in_0,
                    args,
                    tl_new_pair(in_0, tl_new_int(in_0, len), tl_new_pair(in_0, callex, env)),
                    Some(
                        _tl_apply_next_body_callable_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_eval_all_args:_tl_apply_next_body_callable_k\0" as *const u8
                        as *const libc::c_char,
                );
            }
            6 | 4 | 3 => {
                let mut l_arg = args;
                let mut arg = if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                };
                while !l_arg.is_null() {
                    if (*callex).kind as libc::c_uint != TL_THEN as libc::c_int as libc::c_uint
                        && (if !arg.is_null()
                            && (arg.is_null()
                                || (*arg).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*arg).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            NULL as *mut tl_object_s
                        }) != (*in_0).true_
                    {
                        if !((*in_0).error).is_null() {
                            tl_new_pair(
                                in_0,
                                tl_new_pair(
                                    in_0,
                                    tl_new_sym(
                                        in_0,
                                        b"invoke macro/cfunc with non-syntactic arg\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    callex,
                                ),
                                arg,
                            );
                        } else {
                            let ref mut fresh165 = (*in_0).error;
                            *fresh165 = tl_new_pair(
                                in_0,
                                tl_new_pair(
                                    in_0,
                                    tl_new_sym(
                                        in_0,
                                        b"invoke macro/cfunc with non-syntactic arg\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    callex,
                                ),
                                arg,
                            );
                        };
                        return TL_RESULT_AGAIN;
                    }
                    new_args = tl_new_pair(
                        in_0,
                        if !arg.is_null()
                            && (arg.is_null()
                                || (*arg).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*arg).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            NULL as *mut tl_object_s
                        },
                        new_args,
                    );
                    l_arg = (if !l_arg.is_null()
                        && (l_arg.is_null()
                            || (*l_arg).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_arg).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    });
                    arg = (if !l_arg.is_null()
                        && (l_arg.is_null()
                            || (*l_arg).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_arg).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    });
                }
                _tl_apply_next_body_callable_k(
                    in_0,
                    tl_list_rvs(in_0, new_args),
                    tl_new_pair(in_0, tl_new_int(in_0, len), tl_new_pair(in_0, callex, env)),
                );
            }
            8 => {
                if len != 1 as libc::c_int as libc::c_long {
                    if !((*in_0).error).is_null() {
                        tl_new_pair(
                            in_0,
                            tl_new_sym(
                                in_0,
                                b"bad cont arity (1)\0" as *const u8 as *const libc::c_char,
                            ),
                            args,
                        );
                    } else {
                        let ref mut fresh166 = (*in_0).error;
                        *fresh166 = tl_new_pair(
                            in_0,
                            tl_new_sym(
                                in_0,
                                b"bad cont arity (1)\0" as *const u8 as *const libc::c_char,
                            ),
                            args,
                        );
                    };
                    return TL_RESULT_AGAIN;
                }
                let ref mut fresh167 = (*in_0).conts;
                *fresh167 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_conts;
                let ref mut fresh168 = (*in_0).values;
                *fresh168 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_values;
                let ref mut fresh169 = (*in_0).env;
                *fresh169 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_env;
                if (if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .next
                } else {
                    NULL as *mut tl_object_s
                }) == (*in_0).true_
                {
                    tl_push_eval(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.first
                                } else {
                                    0 as *mut tl_object_s
                                }))
                                .kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .first
                        } else {
                            NULL as *mut tl_object_s
                        },
                        env,
                    );
                } else {
                    let ref mut fresh170 = (*in_0).values;
                    *fresh170 = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            if !(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                && ((if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.first
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .is_null()
                                    || (*(if !args.is_null()
                                        && (args.is_null()
                                            || (*args).kind as libc::c_uint
                                                == TL_PAIR as libc::c_int as libc::c_uint)
                                    {
                                        (*args).c2rust_unnamed.c2rust_unnamed.first
                                    } else {
                                        0 as *mut tl_object_s
                                    }))
                                    .kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.first
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .first
                            } else {
                                0 as *mut tl_object_s
                            },
                            (*in_0).false_,
                        ),
                        (*in_0).values,
                    );
                }
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"./eval.c\0" as *const u8 as *const libc::c_char,
                    398 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"int tl_apply_next(tl_interp *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        return TL_RESULT_AGAIN;
    }
    #[no_mangle]
    #[c2rust::src_loc = "411:1"]
    pub unsafe extern "C" fn _tl_eval_and_then(
        mut in_0: *mut tl_interp,
        mut expr: *mut tl_object,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
        tl_push_eval(in_0, expr, (*in_0).env);
    }
    #[no_mangle]
    #[c2rust::src_loc = "425:1"]
    pub unsafe extern "C" fn _tl_getc_and_then(
        mut in_0: *mut tl_interp,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
        tl_push_apply(
            in_0,
            TL_APPLY_GETCHAR as libc::c_long,
            TL_EMPTY_LIST as *mut tl_object,
            TL_EMPTY_LIST as *mut tl_object,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "432:1"]
    pub unsafe extern "C" fn _tl_eval_all_args_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut args = if !(if !(if !state.is_null()
            && (state.is_null()
                || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*state).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            }))
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !state.is_null()
                            && (state.is_null()
                                || (*state).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*state).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .first
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .c2rust_unnamed
                .c2rust_unnamed
                .first
            } else {
                0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .c2rust_unnamed
            .first
        } else {
            NULL as *mut tl_object_s
        };
        let mut stack = tl_new_pair(
            in_0,
            if !result.is_null()
                && (result.is_null()
                    || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*result).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
            if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                NULL as *mut tl_object_s
            },
        );
        let mut then = if !state.is_null()
            && (state.is_null()
                || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*state).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        };
        let mut new_state = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    },
                    TL_EMPTY_LIST as *mut tl_object,
                ),
                stack,
            ),
            then,
        );
        if !args.is_null() {
            if (if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                NULL as *mut tl_object_s
            }) == (*in_0).true_
            {
                _tl_eval_and_then(
                    in_0,
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                    new_state,
                    Some(
                        _tl_eval_all_args_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_eval_and_then:_tl_eval_all_args_k\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh171 = (*in_0).values;
                *fresh171 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.first
                                } else {
                                    0 as *mut tl_object_s
                                }))
                                .kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .first
                        } else {
                            0 as *mut tl_object_s
                        },
                        (*in_0).false_,
                    ),
                    (*in_0).values,
                );
                tl_push_apply(
                    in_0,
                    1 as libc::c_int as libc::c_long,
                    tl_new_then(
                        in_0,
                        Some(
                            _tl_eval_all_args_k
                                as unsafe extern "C" fn(
                                    *mut tl_interp,
                                    *mut tl_object,
                                    *mut tl_object,
                                ) -> (),
                        ),
                        new_state,
                        b"_tl_apply_all_args_k<indirect>\0" as *const u8 as *const libc::c_char,
                    ),
                    (*in_0).env,
                );
            }
        } else {
            let mut l_elem = tl_list_rvs(in_0, stack);
            let mut elem = if !(tl_list_rvs(in_0, stack)).is_null()
                && ((tl_list_rvs(in_0, stack)).is_null()
                    || (*tl_list_rvs(in_0, stack)).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*tl_list_rvs(in_0, stack))
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .first
            } else {
                NULL as *mut tl_object_s
            };
            while !l_elem.is_null() {
                let ref mut fresh172 = (*in_0).values;
                *fresh172 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, elem, (*in_0).false_),
                    (*in_0).values,
                );
                l_elem = (if !l_elem.is_null()
                    && (l_elem.is_null()
                        || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_elem).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    NULL as *mut tl_object_s
                });
                elem = (if !l_elem.is_null()
                    && (l_elem.is_null()
                        || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_elem).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                });
            }
            tl_push_apply(in_0, tl_list_len(stack) as libc::c_long, then, (*in_0).env);
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "468:1"]
    pub unsafe extern "C" fn _tl_eval_all_args(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        if !args.is_null() {
            let mut state_0 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            NULL as *mut tl_object_s
                        },
                        TL_EMPTY_LIST as *mut tl_object,
                    ),
                    TL_EMPTY_LIST as *mut tl_object,
                ),
                tobj,
            );
            if (if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                }))
                .c2rust_unnamed
                .c2rust_unnamed
                .next
            } else {
                NULL as *mut tl_object_s
            }) == (*in_0).true_
            {
                _tl_eval_and_then(
                    in_0,
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            }))
                            .kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                    state_0,
                    Some(
                        _tl_eval_all_args_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_eval_and_then:_tl_eval_all_args_k\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh173 = (*in_0).values;
                *fresh173 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).c2rust_unnamed.c2rust_unnamed.first
                                } else {
                                    0 as *mut tl_object_s
                                }))
                                .kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).c2rust_unnamed.c2rust_unnamed.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .first
                        } else {
                            0 as *mut tl_object_s
                        },
                        (*in_0).false_,
                    ),
                    (*in_0).values,
                );
                tl_push_apply(
                    in_0,
                    1 as libc::c_int as libc::c_long,
                    tl_new_then(
                        in_0,
                        Some(
                            _tl_eval_all_args_k
                                as unsafe extern "C" fn(
                                    *mut tl_interp,
                                    *mut tl_object,
                                    *mut tl_object,
                                ) -> (),
                        ),
                        state_0,
                        b"_tl_apply_all_args_k<direct>\0" as *const u8 as *const libc::c_char,
                    ),
                    (*in_0).env,
                );
            }
        } else {
            tl_push_apply(
                in_0,
                0 as libc::c_int as libc::c_long,
                tl_new_then(in_0, then, state, name),
                (*in_0).env,
            );
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "495:1"]
    pub unsafe extern "C" fn tl_run_until_done(mut in_0: *mut tl_interp) {
        let mut res: libc::c_int = 0;
        loop {
            res = tl_apply_next(in_0);
            if !(res != 0) {
                break;
            }
            match res {
                TL_RESULT_AGAIN => {}
                TL_RESULT_GETCHAR => {
                    let ref mut fresh174 = (*in_0).values;
                    *fresh174 = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            tl_new_int(
                                in_0,
                                (if (*in_0).is_putback != 0 {
                                    (*in_0).is_putback = 0 as libc::c_int;
                                    (*in_0).putback
                                } else {
                                    ((*in_0).readf).expect("non-null function pointer")(in_0)
                                }) as libc::c_long,
                            ),
                            (*in_0).false_,
                        ),
                        (*in_0).values,
                    );
                }
                _ => {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"./eval.c\0" as *const u8 as *const libc::c_char,
                        507 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                            b"void tl_run_until_done(tl_interp *)\0",
                        ))
                        .as_ptr(),
                    );
                }
            }
        }
    }
    use super::assert_h::__assert_fail;
    use super::env_c::tl_env_get_kv;
    use super::object_c::{
        tl_gc, tl_list_len, tl_list_rvs, tl_new_int, tl_new_pair, tl_new_sym, tl_new_then,
    };
    use super::stddef_h::{size_t, NULL};
    use super::tinylisp_h::{
        tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_APPLY_DROP, TL_APPLY_DROP_EVAL,
        TL_APPLY_DROP_RESCUE, TL_APPLY_GETCHAR, TL_APPLY_INDIRECT, TL_APPLY_PUSH_EVAL, TL_CFUNC,
        TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FUNC, TL_INT, TL_MACRO, TL_PAIR,
        TL_RESULT_AGAIN, TL_RESULT_DONE, TL_RESULT_GETCHAR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/interp.c:4"]
pub mod interp_c {
    #[c2rust::src_loc = "9:1"]
    pub unsafe extern "C" fn _readf(mut in_0: *mut tl_interp) -> libc::c_int {
        return getchar();
    }
    #[c2rust::src_loc = "10:1"]
    pub unsafe extern "C" fn _writef(mut in_0: *mut tl_interp, c: libc::c_char) {
        putchar(c as libc::c_int);
    }
    #[c2rust::src_loc = "11:1"]
    pub unsafe extern "C" fn _modloadf(
        mut in_0: *mut tl_interp,
        mut fn_0: *const libc::c_char,
    ) -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[c2rust::src_loc = "12:1"]
    pub unsafe extern "C" fn _reallocf(
        mut in_0: *mut tl_interp,
        mut ptr: *mut libc::c_void,
        mut s: size_t,
    ) -> *mut libc::c_void {
        if s == 0 {
            free(ptr);
            return NULL as *mut libc::c_void;
        }
        return realloc(ptr, s);
    }
    #[no_mangle]
    #[c2rust::src_loc = "43:1"]
    pub unsafe extern "C" fn tl_interp_init(mut in_0: *mut tl_interp) {
        tl_interp_init_alloc(
            in_0,
            Some(
                _reallocf
                    as unsafe extern "C" fn(
                        *mut tl_interp,
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "58:1"]
    pub unsafe extern "C" fn tl_interp_init_alloc(
        mut in_0: *mut tl_interp,
        mut reallocf: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut libc::c_void, size_t) -> *mut libc::c_void,
        >,
    ) {
        let ref mut fresh175 = (*in_0).reallocf;
        *fresh175 = reallocf;
        let ref mut fresh176 = (*in_0).readf;
        *fresh176 = Some(_readf as unsafe extern "C" fn(*mut tl_interp) -> libc::c_int);
        let ref mut fresh177 = (*in_0).writef;
        *fresh177 = Some(_writef as unsafe extern "C" fn(*mut tl_interp, libc::c_char) -> ());
        let ref mut fresh178 = (*in_0).modloadf;
        *fresh178 = Some(
            _modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
        );
        tl_ns_init(in_0, &mut (*in_0).ns);
        let ref mut fresh179 = (*in_0).top_alloc;
        *fresh179 = NULL as *mut tl_object;
        let ref mut fresh180 = (*in_0).true_;
        *fresh180 = tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char);
        let ref mut fresh181 = (*in_0).false_;
        *fresh181 = tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char);
        let ref mut fresh182 = (*in_0).error;
        *fresh182 = NULL as *mut tl_object;
        let ref mut fresh183 = (*in_0).prefixes;
        *fresh183 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh184 = (*in_0).current;
        *fresh184 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh185 = (*in_0).conts;
        *fresh185 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh186 = (*in_0).values;
        *fresh186 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh187 = (*in_0).rescue;
        *fresh187 = TL_EMPTY_LIST as *mut tl_object;
        (*in_0).gc_events = TL_DEFAULT_GC_EVENTS as size_t;
        (*in_0).ctr_events = 0 as libc::c_int as size_t;
        (*in_0).putback = 0 as libc::c_int;
        (*in_0).is_putback = 0 as libc::c_int;
        let ref mut fresh188 = (*in_0).read_buffer;
        *fresh188 = NULL as *mut libc::c_char;
        (*in_0).disp_sep = '\t' as i32 as libc::c_char;
        let ref mut fresh189 = (*in_0).top_env;
        *fresh189 = TL_EMPTY_LIST as *mut tl_object;
        let mut top_frm = TL_EMPTY_LIST as *mut tl_object;
        let mut current: *mut tl_init_ent = &mut __start_tl_init_ents;
        top_frm = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char),
                (*in_0).true_,
            ),
            top_frm,
        );
        top_frm = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char),
                (*in_0).false_,
            ),
            top_frm,
        );
        while current != &mut __stop_tl_init_ents as *mut tl_init_ent {
            top_frm = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, (*current).0.name),
                    if (*current).0.flags & 0x1 as libc::c_int as libc::c_ulong != 0 {
                        _tl_new_cfunc_byval(in_0, (*current).0.fn_0, (*current).0.name)
                    } else {
                        _tl_new_cfunc(in_0, (*current).0.fn_0, (*current).0.name)
                    },
                ),
                top_frm,
            );
            current = current.offset(1);
        }
        let ref mut fresh190 = (*in_0).top_env;
        *fresh190 = tl_new_pair(in_0, top_frm, (*in_0).top_env);
        let ref mut fresh191 = (*in_0).env;
        *fresh191 = (*in_0).top_env;
    }
    #[no_mangle]
    #[c2rust::src_loc = "110:1"]
    pub unsafe extern "C" fn tl_interp_cleanup(mut in_0: *mut tl_interp) {
        while !((*in_0).top_alloc).is_null() {
            tl_free(in_0, (*in_0).top_alloc);
        }
        tl_ns_free(in_0, &mut (*in_0).ns);
    }
    use super::ns_c::{tl_ns_free, tl_ns_init};
    use super::object_c::{_tl_new_cfunc, _tl_new_cfunc_byval, tl_free, tl_new_pair, tl_new_sym};
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::{getchar, putchar};
    use super::stdlib_h::{free, realloc};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_interp_s, tl_ns, tl_object, TL_DEFAULT_GC_EVENTS, TL_EMPTY_LIST,
    };
    extern "C" {
        #[c2rust::src_loc = "3:20"]
        pub static mut __start_tl_init_ents: tl_init_ent;
        #[c2rust::src_loc = "3:42"]
        pub static mut __stop_tl_init_ents: tl_init_ent;
    }
}
#[c2rust::header_src = "/home/ember/src/tinylisp/object.c:5"]
pub mod object_c {
    #[no_mangle]
    #[c2rust::src_loc = "15:1"]
    pub unsafe extern "C" fn tl_new(mut in_0: *mut tl_interp) -> *mut tl_object {
        let mut obj = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_object>() as libc::c_ulong,
        ) as *mut tl_object;
        if obj.is_null() {
            tl_gc(in_0);
            obj = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                ::std::mem::size_of::<tl_object>() as libc::c_ulong,
            ) as *mut tl_object;
            if !obj.is_null() {
            } else {
                __assert_fail(
                    b"obj\0" as *const u8 as *const libc::c_char,
                    b"./object.c\0" as *const u8 as *const libc::c_char,
                    22 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"tl_object *tl_new(tl_interp *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        let ref mut fresh192 = (*obj).c2rust_unnamed_0.next_alloc;
        *fresh192 = (*in_0).top_alloc;
        let ref mut fresh193 = (*obj).prev_alloc;
        *fresh193 = NULL as *mut tl_object_s;
        if !((*in_0).top_alloc).is_null() {
            let ref mut fresh194 = (*(*in_0).top_alloc).prev_alloc;
            *fresh194 = obj;
        }
        let ref mut fresh195 = (*in_0).top_alloc;
        *fresh195 = obj;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "38:1"]
    pub unsafe extern "C" fn tl_new_int(
        mut in_0: *mut tl_interp,
        mut ival: libc::c_long,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_INT;
        (*obj).c2rust_unnamed.ival = ival;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "50:1"]
    pub unsafe extern "C" fn tl_new_sym(
        mut in_0: *mut tl_interp,
        mut str: *const libc::c_char,
    ) -> *mut tl_object {
        if !str.is_null() {
            return tl_new_sym_data(in_0, str, strlen(str));
        } else {
            return tl_new_sym_data(
                in_0,
                NULL as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn tl_new_sym_data(
        mut in_0: *mut tl_interp,
        mut data: *const libc::c_char,
        mut len: size_t,
    ) -> *mut tl_object {
        let mut buf = {
            let mut init = tl_buffer_s {
                data: data as *mut libc::c_char,
                len: len,
            };
            init
        };
        return tl_new_sym_name(in_0, tl_ns_resolve(in_0, &mut (*in_0).ns, buf));
    }
    #[no_mangle]
    #[c2rust::src_loc = "76:1"]
    pub unsafe extern "C" fn tl_new_sym_name(
        mut in_0: *mut tl_interp,
        mut name: *mut tl_name,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_SYM;
        let ref mut fresh196 = (*obj).c2rust_unnamed.nm;
        *fresh196 = name;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn tl_new_pair(
        mut in_0: *mut tl_interp,
        mut first: *mut tl_object,
        mut next: *mut tl_object,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_PAIR;
        let ref mut fresh197 = (*obj).c2rust_unnamed.c2rust_unnamed.first;
        *fresh197 = first;
        let ref mut fresh198 = (*obj).c2rust_unnamed.c2rust_unnamed.next;
        *fresh198 = next;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn tl_new_then(
        mut in_0: *mut tl_interp,
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut state: *mut tl_object,
        mut name: *const libc::c_char,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_THEN;
        let ref mut fresh199 = (*obj).c2rust_unnamed.c2rust_unnamed_0.cfunc;
        *fresh199 = cfunc;
        let ref mut fresh200 = (*obj).c2rust_unnamed.c2rust_unnamed_0.state;
        *fresh200 = state;
        let ref mut fresh201 = (*obj).c2rust_unnamed.c2rust_unnamed_0.name;
        *fresh201 = if !name.is_null() {
            tl_strdup(in_0, name)
        } else {
            NULL as *mut libc::c_char
        };
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "116:1"]
    pub unsafe extern "C" fn _tl_new_cfunc(
        mut in_0: *mut tl_interp,
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) -> *mut tl_object {
        let mut obj = tl_new_then(in_0, cfunc, TL_EMPTY_LIST as *mut tl_object, name);
        (*obj).kind = TL_CFUNC;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "128:1"]
    pub unsafe extern "C" fn _tl_new_cfunc_byval(
        mut in_0: *mut tl_interp,
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) -> *mut tl_object {
        let mut obj = tl_new_then(in_0, cfunc, TL_EMPTY_LIST as *mut tl_object, name);
        (*obj).kind = TL_CFUNC_BYVAL;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "138:1"]
    pub unsafe extern "C" fn tl_new_macro(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut envn: *mut tl_object,
        mut body: *mut tl_object,
        mut env: *mut tl_object,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = (if !envn.is_null() {
            TL_MACRO as libc::c_int
        } else {
            TL_FUNC as libc::c_int
        }) as C2RustUnnamed_5;
        let ref mut fresh202 = (*obj).c2rust_unnamed.c2rust_unnamed_1.args;
        *fresh202 = args;
        let ref mut fresh203 = (*obj).c2rust_unnamed.c2rust_unnamed_1.body;
        *fresh203 = body;
        let ref mut fresh204 = (*obj).c2rust_unnamed.c2rust_unnamed_1.env;
        *fresh204 = env;
        let ref mut fresh205 = (*obj).c2rust_unnamed.c2rust_unnamed_1.envn;
        *fresh205 = envn;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn tl_new_cont(
        mut in_0: *mut tl_interp,
        mut env: *mut tl_object,
        mut conts: *mut tl_object,
        mut values: *mut tl_object,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_CONT;
        let ref mut fresh206 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_env;
        *fresh206 = env;
        let ref mut fresh207 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_conts;
        *fresh207 = conts;
        let ref mut fresh208 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_values;
        *fresh208 = values;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "170:1"]
    pub unsafe extern "C" fn tl_free(mut in_0: *mut tl_interp, mut obj: *mut tl_object) {
        if obj.is_null() {
            return;
        }
        if !((*obj).prev_alloc).is_null() {
            let ref mut fresh209 = (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc;
            *fresh209 = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong
                | (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc as size_t
                    & TL_FMASK as libc::c_ulong) as *mut tl_object;
        } else {
            let ref mut fresh210 = (*in_0).top_alloc;
            *fresh210 = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong
                | (*in_0).top_alloc as size_t & TL_FMASK as libc::c_ulong)
                as *mut tl_object;
        }
        if !(((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object)
            .is_null()
        {
            let ref mut fresh211 = (*(((*obj).c2rust_unnamed_0.next_alloc_i
                & !TL_FMASK as libc::c_ulong)
                as *mut tl_object))
                .prev_alloc;
            *fresh211 = (*obj).prev_alloc;
        }
        match (*obj).kind as libc::c_uint {
            4 | 5 | 3 => {
                ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    (*obj).c2rust_unnamed.c2rust_unnamed_0.name as *mut libc::c_void,
                    0 as libc::c_int as size_t,
                );
            }
            _ => {}
        }
        ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            obj as *mut libc::c_void,
            0 as libc::c_int as size_t,
        );
    }
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn _tl_mark_pass(mut obj: *mut tl_object) {
        if obj.is_null() {
            return;
        }
        if (*obj).c2rust_unnamed_0.next_alloc_i & TL_F_MARK as libc::c_ulong != 0 {
            return;
        }
        let ref mut fresh212 = (*obj).c2rust_unnamed_0.next_alloc_i;
        *fresh212 |= TL_F_MARK as libc::c_ulong;
        match (*obj).kind as libc::c_uint {
            0 | 1 => {}
            4 | 5 | 3 => {
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_0.state);
            }
            7 | 6 => {
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_1.args);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_1.body);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_1.env);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_1.envn);
            }
            2 => {
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed.first);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed.next);
            }
            8 => {
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_2.ret_env);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_2.ret_conts);
                _tl_mark_pass((*obj).c2rust_unnamed.c2rust_unnamed_2.ret_values);
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"./object.c\0" as *const u8 as *const libc::c_char,
                    241 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void _tl_mark_pass(tl_object *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "251:1"]
    pub unsafe extern "C" fn tl_gc(mut in_0: *mut tl_interp) {
        let mut obj = (*in_0).top_alloc;
        let mut tmp = 0 as *mut tl_object;
        while !obj.is_null() {
            let ref mut fresh213 = (*obj).c2rust_unnamed_0.next_alloc_i;
            *fresh213 &= !TL_FMASK as libc::c_ulong;
            obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
                as *mut tl_object;
        }
        _tl_mark_pass((*in_0).true_);
        _tl_mark_pass((*in_0).false_);
        _tl_mark_pass((*in_0).error);
        _tl_mark_pass((*in_0).prefixes);
        _tl_mark_pass((*in_0).env);
        _tl_mark_pass((*in_0).top_env);
        _tl_mark_pass((*in_0).current);
        _tl_mark_pass((*in_0).conts);
        _tl_mark_pass((*in_0).values);
        obj = (*in_0).top_alloc;
        while !obj.is_null() {
            if (*obj).c2rust_unnamed_0.next_alloc_i & TL_F_PERMANENT as libc::c_ulong != 0 {
                _tl_mark_pass(obj);
            }
            obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
                as *mut tl_object;
        }
        obj = (*in_0).top_alloc;
        while !obj.is_null() {
            tmp = obj;
            obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
                as *mut tl_object;
            if (*tmp).c2rust_unnamed_0.next_alloc_i & TL_F_MARK as libc::c_ulong == 0 {
                tl_free(in_0, tmp);
            }
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "304:1"]
    pub unsafe extern "C" fn tl_list_len(mut l: *mut tl_object) -> size_t {
        let mut cnt = 0 as libc::c_int as size_t;
        if l.is_null()
            || !(l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            return cnt;
        }
        let mut l_item = l;
        let mut item = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            cnt = cnt.wrapping_add(1);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return cnt;
    }
    #[no_mangle]
    #[c2rust::src_loc = "317:1"]
    pub unsafe extern "C" fn tl_list_rvs(
        mut in_0: *mut tl_interp,
        mut l: *mut tl_object,
    ) -> *mut tl_object {
        let mut res = TL_EMPTY_LIST as *mut tl_object;
        let mut l_item = l;
        let mut item = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return res;
    }
    #[no_mangle]
    #[c2rust::src_loc = "324:1"]
    pub unsafe extern "C" fn tl_list_rvs_improp(
        mut in_0: *mut tl_interp,
        mut l: *mut tl_object,
    ) -> *mut tl_object {
        let mut res = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        l = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).c2rust_unnamed.c2rust_unnamed.next
        } else {
            NULL as *mut tl_object_s
        };
        let mut l_item = l;
        let mut item = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            });
        }
        return res;
    }
    #[no_mangle]
    #[c2rust::src_loc = "336:1"]
    pub unsafe extern "C" fn tl_strdup(
        mut in_0: *mut tl_interp,
        mut str: *const libc::c_char,
    ) -> *mut libc::c_char {
        let mut s: size_t = 0;
        let mut buf = 0 as *mut libc::c_char;
        if str.is_null() {
            return NULL as *mut libc::c_char;
        }
        s = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if s == 0 {
            return NULL as *mut libc::c_char;
        }
        buf =
            ((*in_0).reallocf).expect("non-null function pointer")(in_0, 0 as *mut libc::c_void, s)
                as *mut libc::c_char;
        if buf.is_null() {
            tl_gc(in_0);
            buf = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                s,
            ) as *mut libc::c_char;
            if !buf.is_null() {
            } else {
                __assert_fail(
                    b"buf\0" as *const u8 as *const libc::c_char,
                    b"./object.c\0" as *const u8 as *const libc::c_char,
                    349 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"char *tl_strdup(tl_interp *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        return strcpy(buf, str);
    }
    #[no_mangle]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn tl_calloc(
        mut in_0: *mut tl_interp,
        mut n: size_t,
        mut s: size_t,
    ) -> *mut libc::c_void {
        let mut region = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            n.wrapping_mul(s),
        );
        if region.is_null() {
            return NULL as *mut libc::c_void;
        }
        return memset(region, 0 as libc::c_int, n.wrapping_mul(s));
    }
    use super::assert_h::__assert_fail;
    use super::ns_c::tl_ns_resolve;
    use super::stddef_h::{size_t, NULL};
    use super::string_h::{memset, strcpy, strlen};
    use super::tinylisp_h::{
        tl_buffer, tl_interp, tl_name, tl_ns, tl_object, tl_object_s, C2RustUnnamed_5, TL_CFUNC,
        TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FMASK, TL_FUNC, TL_F_MARK, TL_F_PERMANENT,
        TL_INT, TL_MACRO, TL_PAIR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/read.c:7"]
pub mod read_c {
    #[c2rust::src_loc = "13:9"]
    pub const DEFAULT_SYM_LEN: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn _tl_read_string_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut sym = 0 as *mut tl_object;
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh219 = (*in_0).error;
                *fresh219 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh220 = (*in_0).values;
            *fresh220 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        if ((*in_0).read_buffer).is_null() {
            (*in_0).read_ptr = 0 as libc::c_int as size_t;
            (*in_0).read_sz = DEFAULT_SYM_LEN as size_t;
            let ref mut fresh221 = (*in_0).read_buffer;
            *fresh221 = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                (*in_0).read_sz,
            ) as *mut libc::c_char;
            if !((*in_0).read_buffer).is_null() {
            } else {
                __assert_fail(
                    b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                    b"./read.c\0" as *const u8 as *const libc::c_char,
                    155 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                        b"void _tl_read_string_k(tl_interp *, tl_object *, tl_object *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        match ch {
            EOF => {
                if !((*in_0).error).is_null() {
                    tl_new_sym(in_0, b"EOF in string\0" as *const u8 as *const libc::c_char);
                } else {
                    let ref mut fresh222 = (*in_0).error;
                    *fresh222 =
                        tl_new_sym(in_0, b"EOF in string\0" as *const u8 as *const libc::c_char);
                };
                let ref mut fresh223 = (*in_0).values;
                *fresh223 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            34 => {
                sym = tl_new_sym_data(in_0, (*in_0).read_buffer, (*in_0).read_ptr);
                ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    (*in_0).read_buffer as *mut libc::c_void,
                    0 as libc::c_int as size_t,
                );
                let ref mut fresh224 = (*in_0).read_buffer;
                *fresh224 = NULL as *mut libc::c_char;
                let ref mut fresh225 = (*in_0).values;
                *fresh225 =
                    tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
                return;
            }
            _ => {
                let ref mut fresh226 = (*in_0).read_ptr;
                let fresh227 = *fresh226;
                *fresh226 = (*fresh226).wrapping_add(1);
                *((*in_0).read_buffer).offset(fresh227 as isize) = ch as libc::c_char;
                if (*in_0).read_ptr >= (*in_0).read_sz {
                    (*in_0).read_sz <<= 1 as libc::c_int;
                    let ref mut fresh228 = (*in_0).read_buffer;
                    *fresh228 = ((*in_0).reallocf).expect("non-null function pointer")(
                        in_0,
                        (*in_0).read_buffer as *mut libc::c_void,
                        (*in_0).read_sz,
                    ) as *mut libc::c_char;
                    if !((*in_0).read_buffer).is_null() {
                    } else {
                        __assert_fail(
                            b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                            b"./read.c\0" as *const u8 as *const libc::c_char,
                            170 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                                b"void _tl_read_string_k(tl_interp *, tl_object *, tl_object *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                }
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_string_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_string_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn tl_read(mut in_0: *mut tl_interp) {
        _tl_getc_and_then(
            in_0,
            0 as *mut tl_object,
            Some(
                _tl_read_top_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn _tl_read_top_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh229 = (*in_0).error;
                *fresh229 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh230 = (*in_0).values;
            *fresh230 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        match ch {
            EOF => {
                let ref mut fresh231 = (*in_0).values;
                *fresh231 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, 0 as *mut tl_object, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            32 | 10 | 9 | 11 | 13 | 8 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_top_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            59 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_comment_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_comment_k\0" as *const u8 as *const libc::c_char,
                );
            }
            34 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_string_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_string_k\0" as *const u8 as *const libc::c_char,
                );
            }
            40 => {
                _tl_getc_and_then(
                    in_0,
                    0 as *mut tl_object,
                    Some(
                        _tl_read_list_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    _tl_getc_and_then(
                        in_0,
                        tl_new_int(in_0, (ch - '0' as i32) as libc::c_long),
                        Some(
                            _tl_read_int_k
                                as unsafe extern "C" fn(
                                    *mut tl_interp,
                                    *mut tl_object,
                                    *mut tl_object,
                                ) -> (),
                        ),
                        b"tl_getc_and_then:_tl_read_int_k\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                let mut l_kv = (*in_0).prefixes;
                let mut kv = if !((*in_0).prefixes).is_null()
                    && (((*in_0).prefixes).is_null()
                        || (*(*in_0).prefixes).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).prefixes).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL as *mut tl_object_s
                };
                while !l_kv.is_null() {
                    let mut k = if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    };
                    let mut v = if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    };
                    if !k.is_null()
                        && !v.is_null()
                        && (!k.is_null()
                            && (*k).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && (*(*k).c2rust_unnamed.nm).here.len > 0 as libc::c_int as libc::c_ulong
                        && *((*(*k).c2rust_unnamed.nm).here.data).offset(0 as libc::c_int as isize)
                            as libc::c_int
                            == ch
                    {
                        tl_push_apply(
                            in_0,
                            1 as libc::c_int as libc::c_long,
                            tl_new_then(
                                in_0,
                                Some(
                                    _tl_read_top_prefix_k
                                        as unsafe extern "C" fn(
                                            *mut tl_interp,
                                            *mut tl_object,
                                            *mut tl_object,
                                        )
                                            -> (),
                                ),
                                v,
                                b"_tl_read_top_k<prefix>\0" as *const u8 as *const libc::c_char,
                            ),
                            (*in_0).env,
                        );
                        _tl_getc_and_then(
                            in_0,
                            0 as *mut tl_object,
                            Some(
                                _tl_read_top_k
                                    as unsafe extern "C" fn(
                                        *mut tl_interp,
                                        *mut tl_object,
                                        *mut tl_object,
                                    )
                                        -> (),
                            ),
                            b"tl_getc_and_then:_tl_read_top_k\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    }
                    l_kv = (if !l_kv.is_null()
                        && (l_kv.is_null()
                            || (*l_kv).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_kv).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL as *mut tl_object_s
                    });
                    kv = (if !l_kv.is_null()
                        && (l_kv.is_null()
                            || (*l_kv).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_kv).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    });
                }
                (*in_0).is_putback = 1 as libc::c_int;
                (*in_0).putback = ch;
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_sym_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_sym_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "135:1"]
    pub unsafe extern "C" fn _tl_read_comment_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh232 = (*in_0).error;
                *fresh232 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh233 = (*in_0).values;
            *fresh233 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        match ch {
            EOF => {
                let ref mut fresh234 = (*in_0).values;
                *fresh234 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, 0 as *mut tl_object, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            10 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_top_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_comment_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_comment_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "85:1"]
    pub unsafe extern "C" fn _tl_read_top_prefix_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let ref mut fresh235 = (*in_0).values;
        *fresh235 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    state,
                    tl_new_pair(
                        in_0,
                        if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).c2rust_unnamed.c2rust_unnamed.first
                        } else {
                            0 as *mut tl_object_s
                        },
                        0 as *mut tl_object,
                    ),
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[c2rust::src_loc = "177:1"]
    pub unsafe extern "C" fn _tl_read_pair_cons_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        state = tl_new_pair(
            in_0,
            if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
            state,
        );
        _tl_getc_and_then(
            in_0,
            state,
            Some(
                _tl_read_list_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "182:1"]
    pub unsafe extern "C" fn _tl_read_pair_improp_check_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh236 = (*in_0).error;
                *fresh236 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh237 = (*in_0).values;
            *fresh237 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        match ch {
            32 | 10 | 9 | 11 | 13 | 8 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_pair_improp_check_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_pair_improp_check_k\0" as *const u8
                        as *const libc::c_char,
                );
            }
            41 => {
                let ref mut fresh238 = (*in_0).values;
                *fresh238 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, tl_list_rvs_improp(in_0, state), (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            _ => {
                (*in_0).is_putback = 1 as libc::c_int;
                (*in_0).putback = ch;
                state = tl_new_pair(
                    in_0,
                    if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL as *mut tl_object_s
                    },
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b".\0" as *const u8 as *const libc::c_char),
                        if !state.is_null()
                            && (state.is_null()
                                || (*state).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*state).c2rust_unnamed.c2rust_unnamed.next
                        } else {
                            NULL as *mut tl_object_s
                        },
                    ),
                );
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_list_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "200:1"]
    pub unsafe extern "C" fn _tl_read_pair_improp_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        state = tl_new_pair(
            in_0,
            if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL as *mut tl_object_s
            },
            state,
        );
        _tl_getc_and_then(
            in_0,
            state,
            Some(
                _tl_read_pair_improp_check_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_getc_and_then:_tl_read_pair_improp_check_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "205:1"]
    pub unsafe extern "C" fn _tl_read_list_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh239 = (*in_0).error;
                *fresh239 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh240 = (*in_0).values;
            *fresh240 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        match ch {
            32 | 10 | 9 | 11 | 13 | 8 => {
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_list_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
            41 => {
                let ref mut fresh241 = (*in_0).values;
                *fresh241 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, tl_list_rvs(in_0, state), (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            46 => {
                tl_push_apply(
                    in_0,
                    1 as libc::c_int as libc::c_long,
                    tl_new_then(
                        in_0,
                        Some(
                            _tl_read_pair_improp_k
                                as unsafe extern "C" fn(
                                    *mut tl_interp,
                                    *mut tl_object,
                                    *mut tl_object,
                                ) -> (),
                        ),
                        state,
                        b"_tl_read_pair<improp>\0" as *const u8 as *const libc::c_char,
                    ),
                    (*in_0).env,
                );
                _tl_getc_and_then(
                    in_0,
                    0 as *mut tl_object,
                    Some(
                        _tl_read_top_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                (*in_0).is_putback = 1 as libc::c_int;
                (*in_0).putback = ch;
                tl_push_apply(
                    in_0,
                    1 as libc::c_int as libc::c_long,
                    tl_new_then(
                        in_0,
                        Some(
                            _tl_read_pair_cons_k
                                as unsafe extern "C" fn(
                                    *mut tl_interp,
                                    *mut tl_object,
                                    *mut tl_object,
                                ) -> (),
                        ),
                        state,
                        b"_tl_read_list_k<cons>\0" as *const u8 as *const libc::c_char,
                    ),
                    (*in_0).env,
                );
                _tl_getc_and_then(
                    in_0,
                    0 as *mut tl_object,
                    Some(
                        _tl_read_top_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "236:1"]
    pub unsafe extern "C" fn _tl_read_int_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh242 = (*in_0).error;
                *fresh242 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh243 = (*in_0).values;
            *fresh243 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            state = tl_new_int(
                in_0,
                (*state).c2rust_unnamed.ival * 10 as libc::c_int as libc::c_long
                    + (ch - '0' as i32) as libc::c_long,
            );
            _tl_getc_and_then(
                in_0,
                state,
                Some(
                    _tl_read_int_k
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                b"tl_getc_and_then:_tl_read_int_k\0" as *const u8 as *const libc::c_char,
            );
        } else {
            (*in_0).is_putback = 1 as libc::c_int;
            (*in_0).putback = ch;
            let ref mut fresh244 = (*in_0).values;
            *fresh244 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, state, (*in_0).false_),
                (*in_0).values,
            );
            return;
        };
    }
    #[c2rust::src_loc = "247:1"]
    pub unsafe extern "C" fn _tl_read_sym_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut sym = 0 as *mut tl_object;
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
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
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh245 = (*in_0).error;
                *fresh245 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh246 = (*in_0).values;
            *fresh246 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        ch = (*if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).c2rust_unnamed.c2rust_unnamed.first
        } else {
            NULL as *mut tl_object_s
        })
        .c2rust_unnamed
        .ival as libc::c_int;
        if ((*in_0).read_buffer).is_null() {
            (*in_0).read_ptr = 0 as libc::c_int as size_t;
            (*in_0).read_sz = DEFAULT_SYM_LEN as size_t;
            let ref mut fresh247 = (*in_0).read_buffer;
            *fresh247 = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                (*in_0).read_sz,
            ) as *mut libc::c_char;
            if !((*in_0).read_buffer).is_null() {
            } else {
                __assert_fail(
                    b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                    b"./read.c\0" as *const u8 as *const libc::c_char,
                    250 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                        b"void _tl_read_sym_k(tl_interp *, tl_object *, tl_object *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        match ch {
            40 | 41 => {
                (*in_0).is_putback = 1 as libc::c_int;
                (*in_0).putback = ch;
            }
            32 | 10 | 9 | 11 | 13 | 8 | EOF => {}
            _ => {
                let ref mut fresh250 = (*in_0).read_ptr;
                let fresh251 = *fresh250;
                *fresh250 = (*fresh250).wrapping_add(1);
                *((*in_0).read_buffer).offset(fresh251 as isize) = ch as libc::c_char;
                if (*in_0).read_ptr >= (*in_0).read_sz {
                    (*in_0).read_sz <<= 1 as libc::c_int;
                    let ref mut fresh252 = (*in_0).read_buffer;
                    *fresh252 = ((*in_0).reallocf).expect("non-null function pointer")(
                        in_0,
                        (*in_0).read_buffer as *mut libc::c_void,
                        (*in_0).read_sz,
                    ) as *mut libc::c_char;
                    if !((*in_0).read_buffer).is_null() {
                    } else {
                        __assert_fail(
                            b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                            b"./read.c\0" as *const u8 as *const libc::c_char,
                            263 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                                b"void _tl_read_sym_k(tl_interp *, tl_object *, tl_object *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                }
                _tl_getc_and_then(
                    in_0,
                    state,
                    Some(
                        _tl_read_sym_k
                            as unsafe extern "C" fn(
                                *mut tl_interp,
                                *mut tl_object,
                                *mut tl_object,
                            ) -> (),
                    ),
                    b"tl_getc_and_then:_tl_read_sym_k\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        sym = tl_new_sym_data(in_0, (*in_0).read_buffer, (*in_0).read_ptr);
        ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            (*in_0).read_buffer as *mut libc::c_void,
            0 as libc::c_int as size_t,
        );
        let ref mut fresh248 = (*in_0).read_buffer;
        *fresh248 = NULL as *mut libc::c_char;
        let ref mut fresh249 = (*in_0).values;
        *fresh249 = tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
        return;
    }
    use super::assert_h::__assert_fail;
    use super::ctype_h::{_ISdigit, __ctype_b_loc};
    use super::eval_c::{_tl_getc_and_then, tl_push_apply};
    use super::object_c::{
        tl_list_rvs, tl_list_rvs_improp, tl_new_int, tl_new_pair, tl_new_sym, tl_new_sym_data,
        tl_new_then,
    };
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::EOF;
    use super::tinylisp_h::{
        tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_INT, TL_PAIR, TL_SYM,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/debug.c:8"]
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
        let ref mut fresh253 = (*in_0).values;
        *fresh253 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
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
    use super::eval_c::_tl_eval_and_then;
    use super::object_c::tl_new_pair;
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::{fprintf, fputc, fwrite, stderr};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_object, tl_object_s, C2RustUnnamed_5, TL_CFUNC, TL_CFUNC_BYVAL,
        TL_MACRO, TL_PAIR,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/ns.c:9"]
pub mod ns_c {
    #[no_mangle]
    #[c2rust::src_loc = "6:1"]
    pub unsafe extern "C" fn tl_buf_slice(
        mut in_0: *mut tl_interp,
        mut orig: tl_buffer,
        mut start: size_t,
        mut end: size_t,
    ) -> tl_buffer {
        let mut ret = tl_buffer {
            data: 0 as *mut libc::c_char,
            len: 0,
        };
        if start < orig.len && end <= orig.len && start < end {
        } else {
            __assert_fail(
                b"start < orig.len && end <= orig.len && start < end\0" as *const u8
                    as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"tl_buffer tl_buf_slice(tl_interp *, tl_buffer, size_t, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        ret.len = end.wrapping_sub(start);
        ret.data = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ret.len,
        ) as *mut libc::c_char;
        if !(ret.data).is_null() {
        } else {
            __assert_fail(
                b"ret.data\0" as *const u8 as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                    b"tl_buffer tl_buf_slice(tl_interp *, tl_buffer, size_t, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        memcpy(
            ret.data as *mut libc::c_void,
            (orig.data).offset(start as isize) as *const libc::c_void,
            ret.len,
        );
        return ret;
    }
    #[c2rust::src_loc = "20:1"]
    pub unsafe extern "C" fn tl_ns_split(
        mut in_0: *mut tl_interp,
        mut child: *mut tl_child,
        mut len: size_t,
    ) -> *mut tl_name {
        let mut new_name = 0 as *mut tl_name;
        if len > 0 as libc::c_int as libc::c_ulong && len < (*child).seg.len {
        } else {
            __assert_fail(
                b"len > 0 && len < child->seg.len\0" as *const u8 as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                26 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"tl_name *tl_ns_split(tl_interp *, tl_child *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        new_name = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name;
        if !new_name.is_null() {
        } else {
            __assert_fail(
                b"new_name\0" as *const u8 as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"tl_name *tl_ns_split(tl_interp *, tl_child *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        let ref mut fresh254 = (*new_name).sz_children;
        *fresh254 = 1 as libc::c_int as size_t;
        (*new_name).num_children = *fresh254;
        let ref mut fresh255 = (*new_name).children;
        *fresh255 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_child>() as libc::c_ulong,
        ) as *mut tl_child;
        if !((*new_name).children).is_null() {
        } else {
            __assert_fail(
                b"new_name->children\0" as *const u8 as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                34 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                    b"tl_name *tl_ns_split(tl_interp *, tl_child *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        let ref mut fresh256 = (*(*new_name).children).name;
        *fresh256 = (*child).name;
        (*new_name).here = tl_buf_slice(
            in_0,
            (*(*child).name).here,
            0 as libc::c_int as size_t,
            ((*(*child).name).here.len)
                .wrapping_sub((*child).seg.len)
                .wrapping_add(len),
        );
        (*(*new_name).children).seg = tl_buf_slice(in_0, (*child).seg, len, (*child).seg.len);
        (*child).seg.len = len;
        let ref mut fresh257 = (*child).seg.data;
        *fresh257 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            (*child).seg.data as *mut libc::c_void,
            len,
        ) as *mut libc::c_char;
        let ref mut fresh258 = (*child).name;
        *fresh258 = new_name;
        return new_name;
    }
    #[no_mangle]
    #[c2rust::src_loc = "49:1"]
    pub unsafe extern "C" fn tl_ns_resolve(
        mut in_0: *mut tl_interp,
        mut ns: *mut tl_ns,
        mut name: tl_buffer,
    ) -> *mut tl_name {
        let mut cur = (*ns).root;
        let mut children = 0 as *mut tl_child;
        let mut low: size_t = 0;
        let mut high: size_t = 0;
        let mut index: size_t = 0;
        let mut sign: libc::c_int = 0;
        let mut whole_name = name;
        'c_2714: loop {
            if name.len == 0 {
                return cur;
            }
            children = (*cur).children;
            low = 0 as libc::c_int as size_t;
            high = (*cur).num_children;
            loop {
                index = low
                    .wrapping_add(high)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong);
                if index >= (*cur).num_children {
                    break;
                }
                sign = memcmp(
                    name.data as *const libc::c_void,
                    (*children.offset(index as isize)).seg.data as *const libc::c_void,
                    if name.len < (*children.offset(index as isize)).seg.len {
                        name.len
                    } else {
                        (*children.offset(index as isize)).seg.len
                    },
                );
                if sign == 0 {
                    let mut match_len = if name.len < (*children.offset(index as isize)).seg.len {
                        name.len
                    } else {
                        (*children.offset(index as isize)).seg.len
                    };
                    if name.len < (*children.offset(index as isize)).seg.len {
                        cur = tl_ns_split(in_0, children.offset(index as isize), name.len);
                    } else {
                        cur = (*children.offset(index as isize)).name;
                    }
                    name.data = (name.data).offset(match_len as isize);
                    name.len =
                        (name.len as libc::c_ulong).wrapping_sub(match_len) as size_t as size_t;
                    continue 'c_2714;
                } else if sign > 0 as libc::c_int {
                    if low == index {
                        if low == high {
                            break;
                        }
                        low = low.wrapping_add(1);
                    } else {
                        low = index;
                    }
                } else {
                    if high == index {
                        break;
                    }
                    high = index;
                }
            }
            if low < (*cur).num_children {
                let mut matching = 0 as libc::c_int as size_t;
                while matching < name.len
                    && matching < (*((*cur).children).offset(low as isize)).seg.len
                    && *(name.data).offset(matching as isize) as libc::c_int
                        == *((*((*cur).children).offset(low as isize)).seg.data)
                            .offset(matching as isize) as libc::c_int
                {
                    matching = matching.wrapping_add(1);
                }
                if matching > 0 as libc::c_int as libc::c_ulong {
                    if matching == (*((*cur).children).offset(low as isize)).seg.len {
                        cur = (*((*cur).children).offset(low as isize)).name;
                    } else {
                        cur = tl_ns_split(in_0, ((*cur).children).offset(low as isize), matching);
                    }
                    name.data = (name.data).offset(matching as isize);
                    name.len =
                        (name.len as libc::c_ulong).wrapping_sub(matching) as size_t as size_t;
                    continue;
                }
            }
            if !(low > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            let mut matching_0 = 0 as libc::c_int as size_t;
            while matching_0 < name.len
                && matching_0
                    < (*((*cur).children)
                        .offset(low.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .seg
                    .len
                && *(name.data).offset(matching_0 as isize) as libc::c_int
                    == *((*((*cur).children)
                        .offset(low.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .seg
                    .data)
                        .offset(matching_0 as isize) as libc::c_int
            {
                matching_0 = matching_0.wrapping_add(1);
            }
            if !(matching_0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            if matching_0
                == (*((*cur).children)
                    .offset(low.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .seg
                .len
            {
                cur = (*((*cur).children)
                    .offset(low.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                .name;
            } else {
                cur = tl_ns_split(
                    in_0,
                    ((*cur).children)
                        .offset(low.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                    matching_0,
                );
            }
            name.data = (name.data).offset(matching_0 as isize);
            name.len = (name.len as libc::c_ulong).wrapping_sub(matching_0) as size_t as size_t;
        }
        if (*cur).num_children >= (*cur).sz_children {
            (*cur).sz_children =
                (*cur).sz_children << 1 as libc::c_int | 1 as libc::c_int as libc::c_ulong;
            let ref mut fresh259 = (*cur).children;
            *fresh259 = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                (*cur).children as *mut libc::c_void,
                (::std::mem::size_of::<tl_child>() as libc::c_ulong)
                    .wrapping_mul((*cur).sz_children),
            ) as *mut tl_child;
            if !((*cur).children).is_null() {
            } else {
                __assert_fail(
                    b"cur->children\0" as *const u8 as *const libc::c_char,
                    b"./ns.c\0" as *const u8 as *const libc::c_char,
                    183 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                        b"tl_name *tl_ns_resolve(tl_interp *, tl_ns *, tl_buffer)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        if low < (*cur).num_children {
            memmove(
                ((*cur).children)
                    .offset(low as isize)
                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                ((*cur).children).offset(low as isize) as *const libc::c_void,
                (::std::mem::size_of::<tl_child>() as libc::c_ulong)
                    .wrapping_mul(((*cur).num_children).wrapping_sub(low)),
            );
        }
        if low <= (*cur).num_children && low < (*cur).sz_children {
        } else {
            __assert_fail(
                b"low <= cur->num_children && low < cur->sz_children\0" as *const u8
                    as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"tl_name *tl_ns_resolve(tl_interp *, tl_ns *, tl_buffer)\0",
                ))
                .as_ptr(),
            );
        }
        let ref mut fresh260 = (*cur).num_children;
        *fresh260 = (*fresh260).wrapping_add(1);
        (*((*cur).children).offset(low as isize)).seg =
            tl_buf_slice(in_0, name, 0 as libc::c_int as size_t, name.len);
        let ref mut fresh261 = (*((*cur).children).offset(low as isize)).name;
        *fresh261 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name_s;
        if !((*((*cur).children).offset(low as isize)).name).is_null() {
        } else {
            __assert_fail(
                b"cur->children[low].name\0" as *const u8 as *const libc::c_char,
                b"./ns.c\0" as *const u8 as *const libc::c_char,
                222 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"tl_name *tl_ns_resolve(tl_interp *, tl_ns *, tl_buffer)\0",
                ))
                .as_ptr(),
            );
        }
        cur = (*((*cur).children).offset(low as isize)).name;
        (*cur).here = tl_buf_slice(in_0, whole_name, 0 as libc::c_int as size_t, whole_name.len);
        let ref mut fresh262 = (*cur).sz_children;
        *fresh262 = 0 as libc::c_int as size_t;
        (*cur).num_children = *fresh262;
        let ref mut fresh263 = (*cur).children;
        *fresh263 = NULL as *mut tl_child;
        return cur;
    }
    #[no_mangle]
    #[c2rust::src_loc = "244:1"]
    pub unsafe extern "C" fn tl_ns_init(mut in_0: *mut tl_interp, mut ns: *mut tl_ns) {
        let ref mut fresh264 = (*ns).root;
        *fresh264 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name;
        let ref mut fresh265 = (*(*ns).root).here.data;
        *fresh265 = NULL as *mut libc::c_char;
        (*(*ns).root).here.len = 0 as libc::c_int as size_t;
        let ref mut fresh266 = (*(*ns).root).sz_children;
        *fresh266 = 0 as libc::c_int as size_t;
        (*(*ns).root).num_children = *fresh266;
        let ref mut fresh267 = (*(*ns).root).children;
        *fresh267 = NULL as *mut tl_child;
    }
    #[no_mangle]
    #[c2rust::src_loc = "252:1"]
    pub unsafe extern "C" fn tl_ns_free(mut in_0: *mut tl_interp, mut ns: *mut tl_ns) {
        let mut cur = 0 as *mut tl_name;
        let mut temp = 0 as *mut tl_name;
        let mut child = 0 as *mut tl_child;
        let mut index: size_t = 0;
        if !(!ns.is_null() && !((*ns).root).is_null()) {
            return;
        }
        cur = (*ns).root;
        let ref mut fresh268 = (*cur).chain;
        *fresh268 = NULL as *mut tl_name_s;
        while !cur.is_null() {
            index = 0 as libc::c_int as size_t;
            while index < (*cur).num_children {
                child = &mut *((*cur).children).offset(index as isize) as *mut tl_child;
                ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    (*child).seg.data as *mut libc::c_void,
                    0 as libc::c_int as size_t,
                );
                let ref mut fresh269 = (*(*child).name).chain;
                *fresh269 = (*cur).chain;
                let ref mut fresh270 = (*cur).chain;
                *fresh270 = (*child).name;
                index = index.wrapping_add(1);
            }
            ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                (*cur).children as *mut libc::c_void,
                0 as libc::c_int as size_t,
            );
            ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                (*cur).here.data as *mut libc::c_void,
                0 as libc::c_int as size_t,
            );
            temp = cur;
            cur = (*cur).chain;
            ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                temp as *mut libc::c_void,
                0 as libc::c_int as size_t,
            );
        }
    }
    #[c2rust::src_loc = "278:1"]
    pub unsafe extern "C" fn tl_ns_print_name(
        mut in_0: *mut tl_interp,
        mut nm: *mut tl_name,
        mut lev: size_t,
    ) {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < lev {
            tl_puts(in_0, b"  \0" as *const u8 as *const libc::c_char);
            i = i.wrapping_add(1);
        }
        if nm.is_null() {
            tl_printf(in_0, b"[NULL]\n\0" as *const u8 as *const libc::c_char);
            return;
        }
        tl_printf(
            in_0,
            b"%N\n\0" as *const u8 as *const libc::c_char,
            &mut (*nm).here as *mut tl_buffer,
        );
        i = 0 as libc::c_int as size_t;
        while i < (*nm).num_children {
            tl_ns_print_child(in_0, ((*nm).children).offset(i as isize), lev);
            i = i.wrapping_add(1);
        }
    }
    #[c2rust::src_loc = "295:1"]
    pub unsafe extern "C" fn tl_ns_print_child(
        mut in_0: *mut tl_interp,
        mut child: *mut tl_child,
        mut lev: size_t,
    ) {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < lev {
            tl_puts(in_0, b"  \0" as *const u8 as *const libc::c_char);
            i = i.wrapping_add(1);
        }
        if child.is_null() {
            tl_printf(in_0, b" <NULL>\n\0" as *const u8 as *const libc::c_char);
            return;
        }
        tl_printf(
            in_0,
            b" -- %N -->\n\0" as *const u8 as *const libc::c_char,
            &mut (*child).seg as *mut tl_buffer,
        );
        tl_ns_print_name(
            in_0,
            (*child).name,
            lev.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "311:1"]
    pub unsafe extern "C" fn tl_ns_print(mut in_0: *mut tl_interp, mut ns: *mut tl_ns) {
        tl_ns_print_name(in_0, (*ns).root, 0 as libc::c_int as size_t);
    }
    #[no_mangle]
    #[c2rust::src_loc = "315:1"]
    pub unsafe extern "C" fn tl_ns_for_each(
        mut in_0: *mut tl_interp,
        mut ns: *mut tl_ns,
        mut cb: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_ns, *mut tl_name, *mut libc::c_void) -> (),
        >,
        mut data: *mut libc::c_void,
    ) {
        let mut i: size_t = 0;
        let mut cur = (*ns).root;
        let ref mut fresh271 = (*cur).chain;
        *fresh271 = NULL as *mut tl_name_s;
        while !cur.is_null() {
            i = 0 as libc::c_int as size_t;
            while i < (*cur).num_children {
                let ref mut fresh272 = (*(*((*cur).children).offset(i as isize)).name).chain;
                *fresh272 = (*cur).chain;
                let ref mut fresh273 = (*cur).chain;
                *fresh273 = (*((*cur).children).offset(i as isize)).name;
                i = i.wrapping_add(1);
            }
            cb.expect("non-null function pointer")(in_0, ns, cur, data);
            cur = (*cur).chain;
        }
    }
    #[c2rust::src_loc = "331:1"]
    pub unsafe extern "C" fn _tl_add_symbol(
        mut in_0: *mut tl_interp,
        mut _ns: *mut tl_ns,
        mut name: *mut tl_name,
        mut data: *mut libc::c_void,
    ) {
        let mut cell = data as *mut tl_object;
        let ref mut fresh274 = (*cell).c2rust_unnamed.c2rust_unnamed.first;
        *fresh274 = tl_new_pair(
            in_0,
            tl_new_sym_name(in_0, name),
            (*cell).c2rust_unnamed.c2rust_unnamed.first,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "336:1"]
    pub unsafe extern "C" fn tl_cf_all_symbols(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cell = tl_new_pair(
            in_0,
            TL_EMPTY_LIST as *mut tl_object,
            TL_EMPTY_LIST as *mut tl_object,
        );
        tl_ns_for_each(
            in_0,
            &mut (*in_0).ns,
            Some(
                _tl_add_symbol
                    as unsafe extern "C" fn(
                        *mut tl_interp,
                        *mut tl_ns,
                        *mut tl_name,
                        *mut libc::c_void,
                    ) -> (),
            ),
            cell as *mut libc::c_void,
        );
        let ref mut fresh275 = (*in_0).values;
        *fresh275 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !cell.is_null()
                    && (cell.is_null()
                        || (*cell).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cell).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "336:1"]
    pub static mut init_tl_cf_all_symbols: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_all_symbols
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-all-symbols\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "342:1"]
    pub static mut init_tl_cf_print_ns: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_print_ns
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-print-ns\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as size_t,
            };
            init
        })
    };
    #[no_mangle]
    #[c2rust::src_loc = "342:1"]
    pub unsafe extern "C" fn tl_cf_print_ns(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_ns_print(in_0, &mut (*in_0).ns);
        let ref mut fresh276 = (*in_0).values;
        *fresh276 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    use super::assert_h::__assert_fail;
    use super::object_c::{tl_new_pair, tl_new_sym_name};
    use super::print_c::{tl_printf, tl_puts};
    use super::stddef_h::{size_t, NULL};
    use super::string_h::{memcmp, memcpy, memmove};
    use super::tinylisp_h::{
        tl_buffer, tl_child, tl_init_ent, tl_interp, tl_name, tl_name_s, tl_ns, tl_object,
        tl_object_s, C2RustUnnamed_5, TL_EMPTY_LIST, TL_PAIR,
    };
}
#[c2rust::header_src = "/usr/include/unistd.h:10"]
pub mod unistd_h {
    #[c2rust::src_loc = "210:9"]
    pub const STDIN_FILENO: libc::c_int = 0 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "809:1"]
        pub fn isatty(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/ember/src/tinylisp/main.c:10"]
pub mod main_c {
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
                        || (*in_0.env).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*in_0.env).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    NULL_0 as *mut tl_object_s
                };
                while !l_frm.is_null() {
                    fprintf(stderr, b"\nFrame\0" as *const u8 as *const libc::c_char);
                    if if !l_frm.is_null()
                        && (l_frm.is_null()
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL_0 as *mut tl_object_s
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
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).c2rust_unnamed.c2rust_unnamed.next
                    } else {
                        NULL_0 as *mut tl_object_s
                    });
                    frm = (if !l_frm.is_null()
                        && (l_frm.is_null()
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        NULL_0 as *mut tl_object_s
                    });
                }
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                in_0.error = NULL_0 as *mut tl_object;
            }
            in_0.conts = TL_EMPTY_LIST as *mut tl_object;
            in_0.values = TL_EMPTY_LIST as *mut tl_object;
            tl_gc(&mut in_0);
        }
        return 0;
    }
    #[no_mangle]
    #[c2rust::src_loc = "161:1"]
    pub unsafe extern "C" fn print_cont_stack(mut in_0: *mut tl_interp, mut stack: *mut tl_object) {
        let mut obj = (*in_0).top_alloc;
        while !obj.is_null() {
            let ref mut fresh277 = (*obj).c2rust_unnamed_0.next_alloc_i;
            *fresh277 &= !TL_FMASK as libc::c_ulong;
            obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
                as *mut tl_object;
        }
        fprintf(stderr, b"\nCurrent: \0" as *const u8 as *const libc::c_char);
        _print_cont(in_0, (*in_0).current, 0 as libc::c_int);
        _print_cont_stack(in_0, stack, 0 as libc::c_int);
    }
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
            as *mut Option<
                unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
            >))
            .expect("non-null function pointer")(in_0, fname);
    }
    #[c2rust::src_loc = "46:9"]
    pub const QUIET_OFF: libc::c_int = 0 as libc::c_int;
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
                    NULL_0 as *mut tl_object_s
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
                    NULL_0 as *mut tl_object_s
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
        let ref mut fresh278 = (*in_0).values;
        *fresh278 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[c2rust::src_loc = "48:9"]
    pub const QUIET_NO_TRUE: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const QUIET_NO_VALUE: libc::c_int = 3 as libc::c_int;
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
            NULL_0 as *mut tl_object_s
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
        let ref mut fresh279 = (*in_0).current;
        *fresh279 = TL_EMPTY_LIST as *mut tl_object;
        _tl_eval_and_then(
            in_0,
            expr,
            0 as *mut tl_object,
            Some(
                _main_k
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_eval_and_then:_main_k\0" as *const u8 as *const libc::c_char,
        );
    }
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
                NULL_0 as *mut tl_object_s
            };
            if !arg.is_null()
                && (*arg).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
            {
                quiet = (*arg).c2rust_unnamed.ival as libc::c_int;
                let ref mut fresh280 = (*in_0).values;
                *fresh280 = tl_new_pair(
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
                    let ref mut fresh281 = (*in_0).error;
                    *fresh281 = tl_new_pair(
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
            let ref mut fresh282 = (*in_0).values;
            *fresh282 = tl_new_pair(
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
    #[c2rust::src_loc = "89:1"]
    pub static mut init_tl_cf_quiet: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_quiet
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-quiet\0" as *const u8 as *const libc::c_char,
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
                let ref mut fresh283 = (*in_0).error;
                *fresh283 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-exit on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    args,
                );
            };
            let ref mut fresh284 = (*in_0).values;
            *fresh284 = tl_new_pair(
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
                NULL_0 as *mut tl_object_s
            })
            .c2rust_unnamed
            .ival as libc::c_int,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "103:1"]
    pub static mut init_tl_cf_exit: tl_init_ent = unsafe {
        tl_init_ent_s({
            let mut init = tl_init_ent_s_Inner {
                fn_0: Some(
                    tl_cf_exit
                        as unsafe extern "C" fn(
                            *mut tl_interp,
                            *mut tl_object,
                            *mut tl_object,
                        ) -> (),
                ),
                name: b"tl-exit\0" as *const u8 as *const libc::c_char,
                flags: 0x1 as libc::c_int as size_t,
            };
            init
        })
    };
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
            NULL_0 as *mut tl_object_s
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
                NULL_0 as *mut tl_object_s
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
                NULL_0 as *mut tl_object_s
            });
            cont = (if !l_cont.is_null()
                && (l_cont.is_null()
                    || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_cont).c2rust_unnamed.c2rust_unnamed.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
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
            NULL_0 as *mut tl_object_s
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
            NULL_0 as *mut tl_object_s
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
            let ref mut fresh285 = (*callex).c2rust_unnamed_0.next_alloc_i;
            *fresh285 |= TL_F_MARK as libc::c_ulong;
            fprintf(stderr, b":\0" as *const u8 as *const libc::c_char);
            _print_cont_stack(
                in_0,
                (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_conts,
                level + 1 as libc::c_int,
            );
        }
    }
    use super::bits_dlfcn_h::{RTLD_GLOBAL, RTLD_NOW};
    use super::dlfcn_h::{dlerror, dlopen, dlsym};
    use super::eval_c::{_tl_eval_and_then, tl_push_apply, tl_run_until_done};
    use super::interp_c::{tl_interp_cleanup, tl_interp_init};
    use super::object_c::{tl_gc, tl_new_int, tl_new_pair, tl_new_sym, tl_new_then};
    use super::print_c::{tl_print, tl_printf};
    use super::read_c::tl_read;
    use super::stddef_h::{size_t, NULL_0};
    use super::stdio_h::{fflush, fprintf, stderr, stdout};
    use super::stdlib_h::exit;
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_interp_s, tl_name, tl_ns, tl_object, tl_object_s,
        C2RustUnnamed_5, TL_CONT, TL_EMPTY_LIST, TL_FMASK, TL_F_MARK, TL_INT, TL_PAIR, TL_THEN,
    };
    use super::unistd_h::{isatty, STDIN_FILENO};
}
#[c2rust::header_src = "/usr/include/dlfcn.h:10"]
pub mod dlfcn_h {
    extern "C" {
        #[c2rust::src_loc = "58:1"]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
        #[c2rust::src_loc = "66:1"]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
            -> *mut libc::c_void;
        #[c2rust::src_loc = "84:1"]
        pub fn dlerror() -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/dlfcn.h:10"]
pub mod bits_dlfcn_h {
    #[c2rust::src_loc = "25:9"]
    pub const RTLD_NOW: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const RTLD_GLOBAL: libc::c_int = 0x100 as libc::c_int;
}
use self::assert_h::__assert_fail;
pub use self::bits_dlfcn_h::{RTLD_GLOBAL, RTLD_NOW};
pub use self::builtin_c::{
    _tl_cf_define_k, _tl_cf_if_k, _tl_cf_set_k, _tl_readc_k, _unboolify, init_tl_cf_add,
    init_tl_cf_all_objects, init_tl_cf_apply, init_tl_cf_call_with_current_continuation,
    init_tl_cf_car, init_tl_cf_cdr, init_tl_cf_chr, init_tl_cf_concat, init_tl_cf_cons,
    init_tl_cf_define, init_tl_cf_display, init_tl_cf_display_sep, init_tl_cf_div, init_tl_cf_env,
    init_tl_cf_eq, init_tl_cf_error, init_tl_cf_evalin, init_tl_cf_gc, init_tl_cf_if,
    init_tl_cf_lambda, init_tl_cf_length, init_tl_cf_less, init_tl_cf_load_mod, init_tl_cf_macro,
    init_tl_cf_mod, init_tl_cf_mul, init_tl_cf_nand, init_tl_cf_null, init_tl_cf_ord,
    init_tl_cf_prefix, init_tl_cf_putbackc, init_tl_cf_read, init_tl_cf_readc, init_tl_cf_rescue,
    init_tl_cf_set, init_tl_cf_setenv, init_tl_cf_sub, init_tl_cf_substr, init_tl_cf_topenv,
    init_tl_cf_type, init_tl_cf_writec, tl_cf_add, tl_cf_all_objects, tl_cf_apply,
    tl_cf_call_with_current_continuation, tl_cf_car, tl_cf_cdr, tl_cf_chr, tl_cf_concat,
    tl_cf_cons, tl_cf_define, tl_cf_display, tl_cf_display_sep, tl_cf_div, tl_cf_env, tl_cf_eq,
    tl_cf_error, tl_cf_evalin, tl_cf_gc, tl_cf_if, tl_cf_lambda, tl_cf_length, tl_cf_less,
    tl_cf_load_mod, tl_cf_macro, tl_cf_mod, tl_cf_mul, tl_cf_nand, tl_cf_null, tl_cf_ord,
    tl_cf_prefix, tl_cf_putbackc, tl_cf_read, tl_cf_readc, tl_cf_rescue, tl_cf_set, tl_cf_setenv,
    tl_cf_sub, tl_cf_substr, tl_cf_topenv, tl_cf_type, tl_cf_writec,
};
pub use self::byteswap_h::{__bswap_16, __bswap_32, __bswap_64};
pub use self::ctype_h::{
    C2RustUnnamed_7, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower,
    _ISprint, _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc,
};
pub use self::debug_c::{
    _indent, _tl_cf_debug_print_k, init_tl_cf_debug_print, tl_cf_debug_print, tl_dbg_print,
};
use self::dlfcn_h::{dlerror, dlopen, dlsym};
pub use self::env_c::{tl_env_get_kv, tl_env_set_global, tl_env_set_local, tl_frm_set};
pub use self::eval_c::{
    _tl_apply_next_body_callable_k, _tl_eval_all_args, _tl_eval_all_args_k, _tl_eval_and_then,
    _tl_getc_and_then, tl_apply_next, tl_push_apply, tl_push_eval, tl_run_until_done,
};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::interp_c::{
    __start_tl_init_ents, __stop_tl_init_ents, _modloadf, _readf, _reallocf, _writef,
    tl_interp_cleanup, tl_interp_init, tl_interp_init_alloc,
};
pub use self::main_c::{
    _global_in, _main_k, _main_read_k, _print_cont, _print_cont_stack, init_tl_cf_exit,
    init_tl_cf_quiet, main_0, my_modloadf, print_cont_stack, quiet, running, tl_cf_exit,
    tl_cf_quiet, QUIET_NO_TRUE, QUIET_NO_VALUE, QUIET_OFF,
};
pub use self::ns_c::{
    _tl_add_symbol, init_tl_cf_all_symbols, init_tl_cf_print_ns, tl_buf_slice, tl_cf_all_symbols,
    tl_cf_print_ns, tl_ns_for_each, tl_ns_free, tl_ns_init, tl_ns_print, tl_ns_print_child,
    tl_ns_print_name, tl_ns_resolve, tl_ns_split,
};
pub use self::object_c::{
    _tl_mark_pass, _tl_new_cfunc, _tl_new_cfunc_byval, tl_calloc, tl_free, tl_gc, tl_list_len,
    tl_list_rvs, tl_list_rvs_improp, tl_new, tl_new_cont, tl_new_int, tl_new_macro, tl_new_pair,
    tl_new_sym, tl_new_sym_data, tl_new_sym_name, tl_new_then, tl_strdup,
};
pub use self::print_c::{
    C2RustUnnamed_6, _mempbrk, _print_pairs, tl_print, tl_printf, tl_puts, tl_write,
    QUOTED_SYM_CHARS,
};
pub use self::read_c::{
    _tl_read_comment_k, _tl_read_int_k, _tl_read_list_k, _tl_read_pair_cons_k,
    _tl_read_pair_improp_check_k, _tl_read_pair_improp_k, _tl_read_string_k, _tl_read_sym_k,
    _tl_read_top_k, _tl_read_top_prefix_k, tl_read, DEFAULT_SYM_LEN,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::stdio_h::{
    fflush, fprintf, fputc, fwrite, getchar, putchar, snprintf, stderr, stdout, EOF,
};
use self::stdlib_h::{exit, free, realloc};
use self::string_h::{memcmp, memcpy, memmove, memset, strcpy, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::tinylisp_h::{
    tl_buffer, tl_buffer_s, tl_child, tl_child_s, tl_init_ent, tl_init_ent_s, tl_init_ent_s_Inner,
    tl_init_ent_s_PADDING, tl_interp, tl_interp_s, tl_name, tl_name_s, tl_ns, tl_ns_s, tl_object,
    tl_object_s, C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
    C2RustUnnamed_4, C2RustUnnamed_5, TL_APPLY_DROP, TL_APPLY_DROP_EVAL, TL_APPLY_DROP_RESCUE,
    TL_APPLY_GETCHAR, TL_APPLY_INDIRECT, TL_APPLY_PUSH_EVAL, TL_CFUNC, TL_CFUNC_BYVAL, TL_CONT,
    TL_DEFAULT_GC_EVENTS, TL_EMPTY_LIST, TL_FMASK, TL_FUNC, TL_F_MARK, TL_F_PERMANENT, TL_INT,
    TL_MACRO, TL_PAIR, TL_RESULT_AGAIN, TL_RESULT_DONE, TL_RESULT_GETCHAR, TL_SYM, TL_THEN,
};
pub use self::types_h::{__off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t};
pub use self::uintn_identity_h::{__uint16_identity, __uint32_identity, __uint64_identity};
pub use self::unistd_h::{isatty, STDIN_FILENO};
pub use self::FILE_h::FILE;
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
