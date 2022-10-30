use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
use crate::tl::tinylisp_h::*;
pub use self::stddef_h::{size_t, NULL};
#[no_mangle]
#[c2rust::src_loc = "6:1"]
pub unsafe extern "C" fn tl_env_get_kv(
    mut in_0: *mut tl_interp,
    mut env: *mut tl_object,
    mut nm: *mut tl_object,
) -> *mut tl_object {
    let mut l_frame = env;
    let mut frame = if !env.is_null()
        && (env.is_null() || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
        && (kv.is_null() || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        let ref mut fresh0 = (*kv).c2rust_unnamed.c2rust_unnamed.next;
        *fresh0 = val;
        return env;
    }
    if env.is_null() {
        env = tl_new_pair(in_0, TL_EMPTY_LIST as *mut tl_object, env);
    }
    let mut l_frame = env;
    let mut frame = if !env.is_null()
        && (env.is_null() || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
            let ref mut fresh1 = (*l_frame).c2rust_unnamed.c2rust_unnamed.first;
            *fresh1 = tl_frm_set(in_0, frame, nm, val);
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
    let ref mut fresh2 = (*env).c2rust_unnamed.c2rust_unnamed.first;
    *fresh2 = tl_frm_set(
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
        && (frm.is_null() || (*frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
