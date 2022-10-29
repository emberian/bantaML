extern "C" {
    fn tl_new_pair(_: *mut tl_interp, _: *mut tl_object, _: *mut tl_object) -> *mut tl_object;
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
pub unsafe extern "C" fn tl_env_get_kv(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut nm: *mut tl_object,
) -> *mut tl_object {
    let mut l_frame: *mut tl_object = env;
    let mut frame: *mut tl_object = if !env.is_null()
        && (env.is_null() || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*env).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    while !l_frame.is_null() {
        let mut l_kv: *mut tl_object = frame;
        let mut kv: *mut tl_object = if !frame.is_null()
            && (frame.is_null()
                || (*frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*frame).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        };
        while !l_kv.is_null() {
            let mut key: *mut tl_object = if !kv.is_null()
                && (kv.is_null()
                    || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*kv).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            };
            let mut val: *mut tl_object = if !kv.is_null()
                && (kv.is_null()
                    || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*kv).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
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
                0 as *mut tl_object_s
            });
            kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            });
        }
        l_frame = (if !l_frame.is_null()
            && (l_frame.is_null()
                || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_frame).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        frame = (if !l_frame.is_null()
            && (l_frame.is_null()
                || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_frame).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return 0 as *mut tl_object;
}
#[no_mangle]
pub unsafe extern "C" fn tl_env_set_global(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut nm: *mut tl_object,
    mut val: *mut tl_object,
) -> *mut tl_object {
    let mut kv: *mut tl_object = tl_env_get_kv(in_0, env, nm);
    if !kv.is_null()
        && (kv.is_null() || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        let ref mut fresh0 = (*kv).c2rust_unnamed.c2rust_unnamed.next;
        *fresh0 = val;
        return env;
    }
    if env.is_null() {
        env = tl_new_pair(in_0, 0 as *mut tl_object, env);
    }
    let mut l_frame: *mut tl_object = env;
    let mut frame: *mut tl_object = if !env.is_null()
        && (env.is_null() || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*env).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    while !l_frame.is_null() {
        if if !l_frame.is_null()
            && (l_frame.is_null()
                || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_frame).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        }
        .is_null()
        {
            let ref mut fresh1 = (*l_frame).c2rust_unnamed.c2rust_unnamed.first;
            *fresh1 = tl_frm_set(in_0, frame, nm, val);
        }
        l_frame = (if !l_frame.is_null()
            && (l_frame.is_null()
                || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_frame).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        frame = (if !l_frame.is_null()
            && (l_frame.is_null()
                || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_frame).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn tl_env_set_local(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut nm: *mut tl_object,
    mut val: *mut tl_object,
) -> *mut tl_object {
    if env.is_null() {
        env = tl_new_pair(in_0, 0 as *mut tl_object, env);
    }
    let ref mut fresh2 = (*env).c2rust_unnamed.c2rust_unnamed.first;
    *fresh2 = tl_frm_set(
        in_0,
        if !env.is_null()
            && (env.is_null()
                || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*env).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        },
        nm,
        val,
    );
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn tl_frm_set(
    mut in_0: *mut tl_interp,
    mut frm: *mut tl_object,
    mut nm: *mut tl_object,
    mut val: *mut tl_object,
) -> *mut tl_object {
    let mut l_kv: *mut tl_object = frm;
    let mut kv: *mut tl_object = if !frm.is_null()
        && (frm.is_null() || (*frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*frm).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
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
            let ref mut fresh3 = (*kv).c2rust_unnamed.c2rust_unnamed.next;
            *fresh3 = val;
            return frm;
        }
        l_kv = (if !l_kv.is_null()
            && (l_kv.is_null()
                || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_kv).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        });
        kv = (if !l_kv.is_null()
            && (l_kv.is_null()
                || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_kv).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
    return tl_new_pair(in_0, tl_new_pair(in_0, nm, val), frm);
}
