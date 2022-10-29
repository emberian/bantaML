extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn tl_ns_resolve(_: *mut tl_interp, _: *mut tl_ns, _: tl_buffer) -> *mut tl_name;
}
pub type size_t = libc::c_ulong;
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
#[no_mangle]
pub unsafe extern "C" fn tl_new(mut in_0: *mut tl_interp) -> *mut tl_object {
    let mut obj: *mut tl_object = ((*in_0).reallocf).expect("non-null function pointer")(
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
                b"object.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"tl_object *tl_new(tl_interp *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    let ref mut fresh0 = (*obj).c2rust_unnamed_0.next_alloc;
    *fresh0 = (*in_0).top_alloc;
    let ref mut fresh1 = (*obj).prev_alloc;
    *fresh1 = 0 as *mut tl_object_s;
    if !((*in_0).top_alloc).is_null() {
        let ref mut fresh2 = (*(*in_0).top_alloc).prev_alloc;
        *fresh2 = obj;
    }
    let ref mut fresh3 = (*in_0).top_alloc;
    *fresh3 = obj;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_int(
    mut in_0: *mut tl_interp,
    mut ival: libc::c_long,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = TL_INT;
    (*obj).c2rust_unnamed.ival = ival;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_sym(
    mut in_0: *mut tl_interp,
    mut str: *const libc::c_char,
) -> *mut tl_object {
    if !str.is_null() {
        return tl_new_sym_data(in_0, str, strlen(str));
    } else {
        return tl_new_sym_data(in_0, 0 as *const libc::c_char, 0 as libc::c_int as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_sym_data(
    mut in_0: *mut tl_interp,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> *mut tl_object {
    let mut buf: tl_buffer = {
        let mut init = tl_buffer_s {
            data: data as *mut libc::c_char,
            len: len,
        };
        init
    };
    return tl_new_sym_name(in_0, tl_ns_resolve(in_0, &mut (*in_0).ns, buf));
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_sym_name(
    mut in_0: *mut tl_interp,
    mut name: *mut tl_name,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = TL_SYM;
    let ref mut fresh4 = (*obj).c2rust_unnamed.nm;
    *fresh4 = name;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_pair(
    mut in_0: *mut tl_interp,
    mut first: *mut tl_object,
    mut next: *mut tl_object,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = TL_PAIR;
    let ref mut fresh5 = (*obj).c2rust_unnamed.c2rust_unnamed.first;
    *fresh5 = first;
    let ref mut fresh6 = (*obj).c2rust_unnamed.c2rust_unnamed.next;
    *fresh6 = next;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_then(
    mut in_0: *mut tl_interp,
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut state: *mut tl_object,
    mut name: *const libc::c_char,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = TL_THEN;
    let ref mut fresh7 = (*obj).c2rust_unnamed.c2rust_unnamed_0.cfunc;
    *fresh7 = cfunc;
    let ref mut fresh8 = (*obj).c2rust_unnamed.c2rust_unnamed_0.state;
    *fresh8 = state;
    let ref mut fresh9 = (*obj).c2rust_unnamed.c2rust_unnamed_0.name;
    *fresh9 = if !name.is_null() {
        tl_strdup(in_0, name)
    } else {
        0 as *mut libc::c_char
    };
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn _tl_new_cfunc(
    mut in_0: *mut tl_interp,
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut name: *const libc::c_char,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new_then(in_0, cfunc, 0 as *mut tl_object, name);
    (*obj).kind = TL_CFUNC;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn _tl_new_cfunc_byval(
    mut in_0: *mut tl_interp,
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut name: *const libc::c_char,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new_then(in_0, cfunc, 0 as *mut tl_object, name);
    (*obj).kind = TL_CFUNC_BYVAL;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_macro(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut envn: *mut tl_object,
    mut body: *mut tl_object,
    mut env: *mut tl_object,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = (if !envn.is_null() {
        TL_MACRO as libc::c_int
    } else {
        TL_FUNC as libc::c_int
    }) as C2RustUnnamed_5;
    let ref mut fresh10 = (*obj).c2rust_unnamed.c2rust_unnamed_1.args;
    *fresh10 = args;
    let ref mut fresh11 = (*obj).c2rust_unnamed.c2rust_unnamed_1.body;
    *fresh11 = body;
    let ref mut fresh12 = (*obj).c2rust_unnamed.c2rust_unnamed_1.env;
    *fresh12 = env;
    let ref mut fresh13 = (*obj).c2rust_unnamed.c2rust_unnamed_1.envn;
    *fresh13 = envn;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_new_cont(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut conts: *mut tl_object,
    mut values: *mut tl_object,
) -> *mut tl_object {
    let mut obj: *mut tl_object = tl_new(in_0);
    (*obj).kind = TL_CONT;
    let ref mut fresh14 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_env;
    *fresh14 = env;
    let ref mut fresh15 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_conts;
    *fresh15 = conts;
    let ref mut fresh16 = (*obj).c2rust_unnamed.c2rust_unnamed_2.ret_values;
    *fresh16 = values;
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn tl_free(mut in_0: *mut tl_interp, mut obj: *mut tl_object) {
    if obj.is_null() {
        return;
    }
    if !((*obj).prev_alloc).is_null() {
        let ref mut fresh17 = (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc;
        *fresh17 = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong
            | (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc as size_t
                & 0x3 as libc::c_int as libc::c_ulong) as *mut tl_object;
    } else {
        let ref mut fresh18 = (*in_0).top_alloc;
        *fresh18 = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong
            | (*in_0).top_alloc as size_t & 0x3 as libc::c_int as libc::c_ulong)
            as *mut tl_object;
    }
    if !(((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong)
        as *mut tl_object)
        .is_null()
    {
        let ref mut fresh19 = (*(((*obj).c2rust_unnamed_0.next_alloc_i
            & !(0x3 as libc::c_int) as libc::c_ulong)
            as *mut tl_object))
            .prev_alloc;
        *fresh19 = (*obj).prev_alloc;
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
unsafe extern "C" fn _tl_mark_pass(mut obj: *mut tl_object) {
    if obj.is_null() {
        return;
    }
    if (*obj).c2rust_unnamed_0.next_alloc_i & 0x1 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    let ref mut fresh20 = (*obj).c2rust_unnamed_0.next_alloc_i;
    *fresh20 |= 0x1 as libc::c_int as libc::c_ulong;
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
                b"object.c\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn tl_gc(mut in_0: *mut tl_interp) {
    let mut obj: *mut tl_object = (*in_0).top_alloc;
    let mut tmp: *mut tl_object = 0 as *mut tl_object;
    while !obj.is_null() {
        let ref mut fresh21 = (*obj).c2rust_unnamed_0.next_alloc_i;
        *fresh21 &= !(0x3 as libc::c_int) as libc::c_ulong;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong)
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
        if (*obj).c2rust_unnamed_0.next_alloc_i & 0x2 as libc::c_int as libc::c_ulong != 0 {
            _tl_mark_pass(obj);
        }
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong)
            as *mut tl_object;
    }
    obj = (*in_0).top_alloc;
    while !obj.is_null() {
        tmp = obj;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !(0x3 as libc::c_int) as libc::c_ulong)
            as *mut tl_object;
        if (*tmp).c2rust_unnamed_0.next_alloc_i & 0x1 as libc::c_int as libc::c_ulong == 0 {
            tl_free(in_0, tmp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tl_list_len(mut l: *mut tl_object) -> size_t {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    if l.is_null()
        || !(l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        return cnt;
    }
    let mut l_item: *mut tl_object = l;
    let mut item: *mut tl_object = if !l.is_null()
        && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*l).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    while !l_item.is_null() {
        cnt = cnt.wrapping_add(1);
        l_item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn tl_list_rvs(
    mut in_0: *mut tl_interp,
    mut l: *mut tl_object,
) -> *mut tl_object {
    let mut res: *mut tl_object = 0 as *mut tl_object;
    let mut l_item: *mut tl_object = l;
    let mut item: *mut tl_object = if !l.is_null()
        && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*l).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    while !l_item.is_null() {
        res = tl_new_pair(in_0, item, res);
        l_item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn tl_list_rvs_improp(
    mut in_0: *mut tl_interp,
    mut l: *mut tl_object,
) -> *mut tl_object {
    let mut res: *mut tl_object = if !l.is_null()
        && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*l).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    l = if !l.is_null()
        && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*l).c2rust_unnamed.c2rust_unnamed.next
    } else {
        0 as *mut tl_object_s
    };
    let mut l_item: *mut tl_object = l;
    let mut item: *mut tl_object = if !l.is_null()
        && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*l).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    while !l_item.is_null() {
        res = tl_new_pair(in_0, item, res);
        l_item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        item = (if !l_item.is_null()
            && (l_item.is_null()
                || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_item).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn tl_strdup(
    mut in_0: *mut tl_interp,
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    let mut s: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    s = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if s == 0 {
        return 0 as *mut libc::c_char;
    }
    buf = ((*in_0).reallocf).expect("non-null function pointer")(in_0, 0 as *mut libc::c_void, s)
        as *mut libc::c_char;
    if buf.is_null() {
        tl_gc(in_0);
        buf =
            ((*in_0).reallocf).expect("non-null function pointer")(in_0, 0 as *mut libc::c_void, s)
                as *mut libc::c_char;
        if !buf.is_null() {
        } else {
            __assert_fail(
                b"buf\0" as *const u8 as *const libc::c_char,
                b"object.c\0" as *const u8 as *const libc::c_char,
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
pub unsafe extern "C" fn tl_calloc(
    mut in_0: *mut tl_interp,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut region: *mut libc::c_void = ((*in_0).reallocf).expect("non-null function pointer")(
        in_0,
        0 as *mut libc::c_void,
        n.wrapping_mul(s),
    );
    if region.is_null() {
        return 0 as *mut libc::c_void;
    }
    return memset(region, 0 as libc::c_int, n.wrapping_mul(s));
}
