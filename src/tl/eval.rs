extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
    fn tl_list_len(_: *mut tl_object) -> size_t;
    fn tl_list_rvs(_: *mut tl_interp, _: *mut tl_object) -> *mut tl_object;
    fn tl_env_get_kv(_: *mut tl_interp, _: *mut tl_object, _: *mut tl_object) -> *mut tl_object;
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
pub unsafe extern "C" fn tl_push_eval(
    mut in_0: *mut tl_interp,
    mut expr: *mut tl_object,
    mut env: *mut tl_object,
) -> libc::c_int {
    if !((*in_0).error).is_null() {
        return 0 as libc::c_int;
    }
    if expr.is_null() {
        if !((*in_0).error).is_null() {
            tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
        } else {
            let ref mut fresh0 = (*in_0).error;
            *fresh0 = tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int;
    }
    if !expr.is_null() && (*expr).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint
        || (!expr.is_null()
            && (*expr).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
            || !expr.is_null()
                && (*expr).kind as libc::c_uint == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
            || !expr.is_null()
                && (*expr).kind as libc::c_uint == TL_THEN as libc::c_int as libc::c_uint
            || !expr.is_null()
                && (*expr).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint
            || !expr.is_null()
                && (*expr).kind as libc::c_uint == TL_FUNC as libc::c_int as libc::c_uint
            || !expr.is_null()
                && (*expr).kind as libc::c_uint == TL_CONT as libc::c_int as libc::c_uint)
    {
        let ref mut fresh1 = (*in_0).values;
        *fresh1 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, expr, (*in_0).false_),
            (*in_0).values,
        );
        return 0 as libc::c_int;
    }
    if !expr.is_null() && (*expr).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint {
        let mut binding: *mut tl_object = tl_env_get_kv(in_0, env, expr);
        if binding.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                    expr,
                );
            } else {
                let ref mut fresh2 = (*in_0).error;
                *fresh2 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                    expr,
                );
            };
            return 0 as libc::c_int;
        }
        let ref mut fresh3 = (*in_0).values;
        *fresh3 = tl_new_pair(
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
        return 0 as libc::c_int;
    }
    if expr.is_null() || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint {
        let mut len: size_t = tl_list_len(expr);
        let mut l_subex: *mut tl_object = expr;
        let mut subex: *mut tl_object = if !expr.is_null()
            && (expr.is_null()
                || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*expr).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        };
        while !l_subex.is_null() {
            if subex
                == (if !expr.is_null()
                    && (expr.is_null()
                        || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*expr).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                })
            {
                tl_push_apply(
                    in_0,
                    len as libc::c_long - 1 as libc::c_int as libc::c_long,
                    subex,
                    env,
                );
            } else {
                let ref mut fresh4 = (*in_0).values;
                *fresh4 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, subex, (*in_0).true_),
                    (*in_0).values,
                );
            }
            l_subex = (if !l_subex.is_null()
                && (l_subex.is_null()
                    || (*l_subex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_subex).c2rust_unnamed.c2rust_unnamed.next
            } else {
                0 as *mut tl_object_s
            });
            subex = (if !l_subex.is_null()
                && (l_subex.is_null()
                    || (*l_subex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_subex).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            });
        }
        return 1 as libc::c_int;
    }
    if !((*in_0).error).is_null() {
        tl_new_pair(
            in_0,
            tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
            expr,
        );
    } else {
        let ref mut fresh5 = (*in_0).error;
        *fresh5 = tl_new_pair(
            in_0,
            tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
            expr,
        );
    };
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tl_push_apply(
    mut in_0: *mut tl_interp,
    mut len: libc::c_long,
    mut expr: *mut tl_object,
    mut env: *mut tl_object,
) {
    let ref mut fresh6 = (*in_0).conts;
    *fresh6 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, len), tl_new_pair(in_0, expr, env)),
        (*in_0).conts,
    );
    let ref mut fresh7 = (*in_0).ctr_events;
    *fresh7 = (*fresh7).wrapping_add(1);
    if (*in_0).gc_events > 0 as libc::c_int as libc::c_ulong
        && (*in_0).ctr_events >= (*in_0).gc_events
    {
        tl_gc(in_0);
        (*in_0).ctr_events = 0 as libc::c_int as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _tl_apply_next_body_callable_k(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut cont: *mut tl_object,
) {
    let mut callex: *mut tl_object = if !(if !cont.is_null()
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
    let mut env: *mut tl_object = if !(if !cont.is_null()
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
        0 as *mut tl_object_s
    };
    let mut frm: *mut tl_object = 0 as *mut tl_object;
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
        let mut is_improp: libc::c_char = 0 as libc::c_int as libc::c_char;
        let mut paramlen: libc::c_long = 0 as libc::c_int as libc::c_long;
        let mut l_item: *mut tl_object = (*callex).c2rust_unnamed.c2rust_unnamed_1.args;
        let mut item: *mut tl_object = if !((*callex).c2rust_unnamed.c2rust_unnamed_1.args)
            .is_null()
            && (((*callex).c2rust_unnamed.c2rust_unnamed_1.args).is_null()
                || (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args).kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*callex).c2rust_unnamed.c2rust_unnamed_1.args)
                .c2rust_unnamed
                .c2rust_unnamed
                .first
        } else {
            0 as *mut tl_object_s
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
                        || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
                let ref mut fresh8 = (*in_0).error;
                *fresh8 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"bad arity\0" as *const u8 as *const libc::c_char),
                        tl_new_int(in_0, paramlen),
                    ),
                    args,
                );
            };
            let ref mut fresh9 = (*in_0).values;
            *fresh9 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let mut acur: *mut tl_object = (*callex).c2rust_unnamed.c2rust_unnamed_1.args;
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
                        0 as *mut tl_object_s
                    },
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).c2rust_unnamed.c2rust_unnamed.first
                    } else {
                        0 as *mut tl_object_s
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
                0 as *mut tl_object_s
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
                        || (*acur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
                            0 as *mut tl_object_s
                        },
                        args,
                    ),
                    frm,
                );
                break;
            } else {
                acur = if !acur.is_null()
                    && (acur.is_null()
                        || (*acur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*acur).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
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
            let ref mut fresh10 = (*in_0).error;
            *fresh10 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"bad arg kind\0" as *const u8 as *const libc::c_char),
                (*callex).c2rust_unnamed.c2rust_unnamed_1.args,
            );
        };
        let ref mut fresh11 = (*in_0).values;
        *fresh11 = tl_new_pair(
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
    let mut body_rvs: *mut tl_object =
        tl_list_rvs(in_0, (*callex).c2rust_unnamed.c2rust_unnamed_1.body);
    let mut l_ex: *mut tl_object = body_rvs;
    let mut ex: *mut tl_object = if !body_rvs.is_null()
        && (body_rvs.is_null()
            || (*body_rvs).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*body_rvs).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
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
                    0 as *mut tl_object_s
                })
            {
                -(1 as libc::c_int)
            } else {
                -(3 as libc::c_int)
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
            0 as *mut tl_object_s
        });
        ex = (if !l_ex.is_null()
            && (l_ex.is_null()
                || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l_ex).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn tl_apply_next(mut in_0: *mut tl_interp) -> libc::c_int {
    let mut cont: *mut tl_object = if !((*in_0).conts).is_null()
        && (((*in_0).conts).is_null()
            || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
    } else {
        0 as *mut tl_object_s
    };
    let mut len: libc::c_long = 0;
    let mut callex: *mut tl_object = 0 as *mut tl_object;
    let mut env: *mut tl_object = 0 as *mut tl_object;
    let mut args: *mut tl_object = 0 as *mut tl_object;
    let mut res: libc::c_int = 0;
    if !((*in_0).error).is_null() {
        let mut rescue: *mut tl_object = if !((*in_0).rescue).is_null()
            && (((*in_0).rescue).is_null()
                || (*(*in_0).rescue).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        };
        if rescue.is_null() {
            return 0 as libc::c_int;
        }
        let ref mut fresh12 = (*in_0).rescue;
        *fresh12 = if !((*in_0).rescue).is_null()
            && (((*in_0).rescue).is_null()
                || (*(*in_0).rescue).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        };
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, rescue, (*in_0).env);
        let ref mut fresh13 = (*in_0).values;
        *fresh13 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).error, (*in_0).false_),
            (*in_0).values,
        );
        let ref mut fresh14 = (*in_0).error;
        *fresh14 = 0 as *mut tl_object;
        return 1 as libc::c_int;
    }
    let ref mut fresh15 = (*in_0).current;
    *fresh15 = cont;
    if cont.is_null() {
        return 0 as libc::c_int;
    }
    let ref mut fresh16 = (*in_0).conts;
    *fresh16 = if !((*in_0).conts).is_null()
        && (((*in_0).conts).is_null()
            || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
    } else {
        0 as *mut tl_object_s
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
            b"eval.c\0" as *const u8 as *const libc::c_char,
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
        0 as *mut tl_object_s
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
        0 as *mut tl_object_s
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
        0 as *mut tl_object_s
    };
    if len == -(4 as libc::c_int) as libc::c_long {
        let ref mut fresh17 = (*in_0).values;
        *fresh17 = if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        };
        return 1 as libc::c_int;
    }
    if len == -(5 as libc::c_int) as libc::c_long {
        let ref mut fresh18 = (*in_0).rescue;
        *fresh18 = if !((*in_0).rescue).is_null()
            && (((*in_0).rescue).is_null()
                || (*(*in_0).rescue).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).rescue).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        };
        return 1 as libc::c_int;
    }
    if len == -(6 as libc::c_int) as libc::c_long {
        if (*in_0).is_putback != 0 {
            let ref mut fresh19 = (*in_0).values;
            *fresh19 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_int(in_0, (*in_0).putback as libc::c_long),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            (*in_0).is_putback = 0 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            return 2 as libc::c_int;
        }
    }
    if len != -(2 as libc::c_int) as libc::c_long {
        res = tl_push_eval(in_0, callex, env);
        if res != 0 {
            if !(len == -(1 as libc::c_int) as libc::c_long
                || len == -(3 as libc::c_int) as libc::c_long)
            {
                cont = if !((*in_0).conts).is_null()
                    && (((*in_0).conts).is_null()
                        || (*(*in_0).conts).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                };
                let ref mut fresh20 = (*in_0).conts;
                *fresh20 = if !((*in_0).conts).is_null()
                    && (((*in_0).conts).is_null()
                        || (*(*in_0).conts).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                };
                tl_push_apply(
                    in_0,
                    -(2 as libc::c_int) as libc::c_long,
                    tl_new_int(in_0, len),
                    env,
                );
                let ref mut fresh21 = (*in_0).conts;
                *fresh21 = tl_new_pair(in_0, cont, (*in_0).conts);
            } else if len == -(3 as libc::c_int) as libc::c_long {
                cont = if !((*in_0).conts).is_null()
                    && (((*in_0).conts).is_null()
                        || (*(*in_0).conts).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                };
                let ref mut fresh22 = (*in_0).conts;
                *fresh22 = if !((*in_0).conts).is_null()
                    && (((*in_0).conts).is_null()
                        || (*(*in_0).conts).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).conts).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                };
                tl_push_apply(
                    in_0,
                    -(4 as libc::c_int) as libc::c_long,
                    0 as *mut tl_object,
                    0 as *mut tl_object,
                );
                let ref mut fresh23 = (*in_0).conts;
                *fresh23 = tl_new_pair(in_0, cont, (*in_0).conts);
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
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
        })
        .c2rust_unnamed
        .c2rust_unnamed
        .first
    } else {
        0 as *mut tl_object_s
    };
    let ref mut fresh24 = (*in_0).values;
    *fresh24 = if !((*in_0).values).is_null()
        && (((*in_0).values).is_null()
            || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
    } else {
        0 as *mut tl_object_s
    };
    if len == -(3 as libc::c_int) as libc::c_long {
        return 1 as libc::c_int;
    }
    if len == -(1 as libc::c_int) as libc::c_long {
        let ref mut fresh25 = (*in_0).values;
        *fresh25 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, callex, (*in_0).false_),
            (*in_0).values,
        );
        return 1 as libc::c_int;
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
            let ref mut fresh26 = (*in_0).error;
            *fresh26 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"call non-callable\0" as *const u8 as *const libc::c_char,
                ),
                callex,
            );
        };
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
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
                0 as *mut tl_object_s
            },
            args,
        );
        let ref mut fresh27 = (*in_0).values;
        *fresh27 = if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).c2rust_unnamed.c2rust_unnamed.next
        } else {
            0 as *mut tl_object_s
        };
        i += 1;
    }
    let ref mut fresh28 = (*in_0).env;
    *fresh28 = env;
    let mut new_args: *mut tl_object = 0 as *mut tl_object;
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
            let mut l_arg: *mut tl_object = args;
            let mut arg: *mut tl_object = if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
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
                        0 as *mut tl_object_s
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
                        let ref mut fresh29 = (*in_0).error;
                        *fresh29 = tl_new_pair(
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
                    return 1 as libc::c_int;
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
                        0 as *mut tl_object_s
                    },
                    new_args,
                );
                l_arg = (if !l_arg.is_null()
                    && (l_arg.is_null()
                        || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_arg).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                });
                arg = (if !l_arg.is_null()
                    && (l_arg.is_null()
                        || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_arg).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
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
                    let ref mut fresh30 = (*in_0).error;
                    *fresh30 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"bad cont arity (1)\0" as *const u8 as *const libc::c_char,
                        ),
                        args,
                    );
                };
                return 1 as libc::c_int;
            }
            let ref mut fresh31 = (*in_0).conts;
            *fresh31 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_conts;
            let ref mut fresh32 = (*in_0).values;
            *fresh32 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_values;
            let ref mut fresh33 = (*in_0).env;
            *fresh33 = (*callex).c2rust_unnamed.c2rust_unnamed_2.ret_env;
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
                0 as *mut tl_object_s
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
                        0 as *mut tl_object_s
                    },
                    env,
                );
            } else {
                let ref mut fresh34 = (*in_0).values;
                *fresh34 = tl_new_pair(
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
                b"eval.c\0" as *const u8 as *const libc::c_char,
                398 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"int tl_apply_next(tl_interp *)\0",
                ))
                .as_ptr(),
            );
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _tl_eval_and_then(
    mut in_0: *mut tl_interp,
    mut expr: *mut tl_object,
    mut state: *mut tl_object,
    mut then: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut name: *const libc::c_char,
) {
    let mut tobj: *mut tl_object = tl_new_then(in_0, then, state, name);
    tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
    tl_push_eval(in_0, expr, (*in_0).env);
}
#[no_mangle]
pub unsafe extern "C" fn _tl_getc_and_then(
    mut in_0: *mut tl_interp,
    mut state: *mut tl_object,
    mut then: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut name: *const libc::c_char,
) {
    let mut tobj: *mut tl_object = tl_new_then(in_0, then, state, name);
    tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
    tl_push_apply(
        in_0,
        -(6 as libc::c_int) as libc::c_long,
        0 as *mut tl_object,
        0 as *mut tl_object,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _tl_eval_all_args_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    mut state: *mut tl_object,
) {
    let mut args: *mut tl_object = if !(if !(if !state.is_null()
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
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
        0 as *mut tl_object_s
    };
    let mut stack: *mut tl_object = tl_new_pair(
        in_0,
        if !result.is_null()
            && (result.is_null()
                || (*result).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*result).c2rust_unnamed.c2rust_unnamed.first
        } else {
            0 as *mut tl_object_s
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
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
            0 as *mut tl_object_s
        },
    );
    let mut then: *mut tl_object = if !state.is_null()
        && (state.is_null()
            || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
    {
        (*state).c2rust_unnamed.c2rust_unnamed.next
    } else {
        0 as *mut tl_object_s
    };
    let mut new_state: *mut tl_object = tl_new_pair(
        in_0,
        tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.next
                } else {
                    0 as *mut tl_object_s
                },
                0 as *mut tl_object,
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
            0 as *mut tl_object_s
        }) == (*in_0).true_
        {
            _tl_eval_and_then(
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
            let ref mut fresh35 = (*in_0).values;
            *fresh35 = tl_new_pair(
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
        let mut l_elem: *mut tl_object = tl_list_rvs(in_0, stack);
        let mut elem: *mut tl_object = if !(tl_list_rvs(in_0, stack)).is_null()
            && ((tl_list_rvs(in_0, stack)).is_null()
                || (*tl_list_rvs(in_0, stack)).kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*tl_list_rvs(in_0, stack))
                .c2rust_unnamed
                .c2rust_unnamed
                .first
        } else {
            0 as *mut tl_object_s
        };
        while !l_elem.is_null() {
            let ref mut fresh36 = (*in_0).values;
            *fresh36 = tl_new_pair(
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
                0 as *mut tl_object_s
            });
            elem = (if !l_elem.is_null()
                && (l_elem.is_null()
                    || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_elem).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            });
        }
        tl_push_apply(in_0, tl_list_len(stack) as libc::c_long, then, (*in_0).env);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _tl_eval_all_args(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut state: *mut tl_object,
    mut then: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    mut name: *const libc::c_char,
) {
    let mut tobj: *mut tl_object = tl_new_then(in_0, then, state, name);
    if !args.is_null() {
        let mut state_0: *mut tl_object = tl_new_pair(
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
                        0 as *mut tl_object_s
                    },
                    0 as *mut tl_object,
                ),
                0 as *mut tl_object,
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
            0 as *mut tl_object_s
        }) == (*in_0).true_
        {
            _tl_eval_and_then(
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
            let ref mut fresh37 = (*in_0).values;
            *fresh37 = tl_new_pair(
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
pub unsafe extern "C" fn tl_run_until_done(mut in_0: *mut tl_interp) {
    let mut res: libc::c_int = 0;
    loop {
        res = tl_apply_next(in_0);
        if !(res != 0) {
            break;
        }
        match res {
            1 => {}
            2 => {
                let ref mut fresh38 = (*in_0).values;
                *fresh38 = tl_new_pair(
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
                    b"eval.c\0" as *const u8 as *const libc::c_char,
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
