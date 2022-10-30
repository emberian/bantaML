use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:2"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
use crate::tl::tinylisp_h::*;
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
#[c2rust::header_src = "/usr/include/string.h:2"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "141:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
use self::assert_h::__assert_fail;
pub use self::stddef_h::{size_t, NULL};
use self::string_h::{memset, strcpy, strlen};
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
    *fresh1 = NULL as *mut tl_object_s;
    if !((*in_0).top_alloc).is_null() {
        let ref mut fresh2 = (*(*in_0).top_alloc).prev_alloc;
        *fresh2 = obj;
    }
    let ref mut fresh3 = (*in_0).top_alloc;
    *fresh3 = obj;
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
    let ref mut fresh4 = (*obj).c2rust_unnamed.nm;
    *fresh4 = name;
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
    let ref mut fresh5 = (*obj).c2rust_unnamed.c2rust_unnamed.first;
    *fresh5 = first;
    let ref mut fresh6 = (*obj).c2rust_unnamed.c2rust_unnamed.next;
    *fresh6 = next;
    return obj;
}
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn tl_new_then(
    mut in_0: *mut tl_interp,
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut state: *mut tl_object,
    mut name: *const libc::c_char,
) -> *mut tl_object {
    let mut obj = tl_new(in_0);
    (*obj).kind = TL_THEN;
    let ref mut fresh7 = (*obj).c2rust_unnamed.c2rust_unnamed_0.cfunc;
    *fresh7 = cfunc;
    let ref mut fresh8 = (*obj).c2rust_unnamed.c2rust_unnamed_0.state;
    *fresh8 = state;
    let ref mut fresh9 = (*obj).c2rust_unnamed.c2rust_unnamed_0.name;
    *fresh9 = if !name.is_null() {
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
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
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
    mut cfunc: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
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
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn tl_new_cont(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut conts: *mut tl_object,
    mut values: *mut tl_object,
) -> *mut tl_object {
    let mut obj = tl_new(in_0);
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
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn tl_free(mut in_0: *mut tl_interp, mut obj: *mut tl_object) {
    if obj.is_null() {
        return;
    }
    if !((*obj).prev_alloc).is_null() {
        let ref mut fresh17 = (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc;
        *fresh17 = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong
            | (*(*obj).prev_alloc).c2rust_unnamed_0.next_alloc as size_t
                & TL_FMASK as libc::c_ulong) as *mut tl_object;
    } else {
        let ref mut fresh18 = (*in_0).top_alloc;
        *fresh18 = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong
            | (*in_0).top_alloc as size_t & TL_FMASK as libc::c_ulong)
            as *mut tl_object;
    }
    if !(((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object)
        .is_null()
    {
        let ref mut fresh19 =
            (*(((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong)
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
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn _tl_mark_pass(mut obj: *mut tl_object) {
    if obj.is_null() {
        return;
    }
    if (*obj).c2rust_unnamed_0.next_alloc_i & TL_F_MARK as libc::c_ulong != 0 {
        return;
    }
    let ref mut fresh20 = (*obj).c2rust_unnamed_0.next_alloc_i;
    *fresh20 |= TL_F_MARK as libc::c_ulong;
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
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn tl_gc(mut in_0: *mut tl_interp) {
    let mut obj = (*in_0).top_alloc;
    let mut tmp = 0 as *mut tl_object;
    while !obj.is_null() {
        let ref mut fresh21 = (*obj).c2rust_unnamed_0.next_alloc_i;
        *fresh21 &= !TL_FMASK as libc::c_ulong;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object;
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
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object;
    }
    obj = (*in_0).top_alloc;
    while !obj.is_null() {
        tmp = obj;
        obj = ((*obj).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object;
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
