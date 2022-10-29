extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub s: *const libc::c_char,
    pub b: *mut tl_buffer,
}
#[no_mangle]
pub unsafe extern "C" fn _print_pairs(mut in_0: *mut tl_interp, mut cur: *mut tl_object) {
    while !cur.is_null() {
        if !(cur.is_null() || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            tl_printf(in_0, b". \0" as *const u8 as *const libc::c_char);
            tl_print(in_0, cur);
            cur = 0 as *mut tl_object;
        } else {
            tl_print(
                in_0,
                if !cur.is_null()
                    && (cur.is_null()
                        || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cur).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
            if !if !cur.is_null()
                && (cur.is_null()
                    || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cur).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
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
                0 as *mut tl_object_s
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _mempbrk(
    mut m: *mut libc::c_char,
    mut s: *mut libc::c_char,
    mut sz: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
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
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn tl_print(
    mut in_0: *mut tl_interp,
    mut obj: *mut tl_object,
) -> *mut tl_object {
    let mut cur: *mut tl_object = 0 as *mut tl_object;
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
                    b"0123456789.,'\"` \n\r\t\x08\x0B\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
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
                } else if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint {
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
pub unsafe extern "C" fn tl_puts(mut in_0: *mut tl_interp, mut s: *const libc::c_char) {
    while *s as libc::c_int != 0 as libc::c_int {
        let fresh0 = s;
        s = s.offset(1);
        ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tl_write(
    mut in_0: *mut tl_interp,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    loop {
        let fresh1 = i;
        i = i.wrapping_add(1);
        if !(fresh1 < len) {
            break;
        }
        let fresh2 = data;
        data = data.offset(1);
        ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tl_printf(
    mut in_0: *mut tl_interp,
    mut cur: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut temp: C2RustUnnamed_6 = C2RustUnnamed_6 {
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
                    let fresh3 = cur;
                    cur = cur.offset(1);
                    ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh3);
                }
            }
        } else {
            let fresh4 = cur;
            cur = cur.offset(1);
            ((*in_0).writef).expect("non-null function pointer")(in_0, *fresh4);
        }
    }
}
