use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
use crate::tl::tinylisp_h::*;

#[c2rust::header_src = "/usr/include/string.h:1"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "64:12"]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:2"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "378:12"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:3"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "555:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:4"]
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
use self::assert_h::__assert_fail;
pub use self::stddef_h::{size_t, NULL};
use self::stdio_h::snprintf;
use self::stdlib_h::free;
use self::string_h::{memcmp, memcpy};
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
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "37:1"]
static mut init_tl_cf_error: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_error
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-error\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
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
            let ref mut fresh0 = (*in_0).error;
            *fresh0 = if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).c2rust_unnamed.c2rust_unnamed.first
            } else {
                0 as *mut tl_object_s
            };
        };
        let ref mut fresh1 = (*in_0).values;
        *fresh1 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    } else {
        let ref mut fresh2 = (*in_0).values;
        *fresh2 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).error, (*in_0).false_),
            (*in_0).values,
        );
        return;
    };
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "46:1"]
static mut init_tl_cf_macro: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_macro
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-macro\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
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
    if !(!envn.is_null() && (*envn).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
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
            let ref mut fresh3 = (*in_0).error;
            *fresh3 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"bad macro envname\0" as *const u8 as *const libc::c_char,
                ),
                envn,
            );
        };
        let ref mut fresh4 = (*in_0).values;
        *fresh4 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    let ref mut fresh5 = (*in_0).values;
    *fresh5 = tl_new_pair(
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
    let ref mut fresh6 = (*in_0).values;
    *fresh6 = tl_new_pair(
        in_0,
        tl_new_pair(
            in_0,
            tl_new_macro(in_0, fargs, 0 as *mut tl_object, body, (*in_0).env),
            (*in_0).false_,
        ),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "57:1"]
static mut init_tl_cf_lambda: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_lambda
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-lambda\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
        };
        init
    })
};
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn _tl_cf_define_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    mut key: *mut tl_object,
) {
    let ref mut fresh7 = (*in_0).env;
    *fresh7 = tl_env_set_local(
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
    let ref mut fresh8 = (*in_0).values;
    *fresh8 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
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
            let ref mut fresh9 = (*in_0).error;
            *fresh9 = tl_new_sym(
                in_0,
                b"bad define arity\0" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    if !(!key.is_null() && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
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
            let ref mut fresh10 = (*in_0).error;
            *fresh10 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"define non-sym\0" as *const u8 as *const libc::c_char,
                ),
                key,
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
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "68:1"]
static mut init_tl_cf_define: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_define
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-define\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "78:1"]
static mut init_tl_cf_display: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_display
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-display\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
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
    let ref mut fresh12 = (*in_0).values;
    *fresh12 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn tl_cf_display_sep(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    let mut arg = 0 as *mut tl_object;
    if args.is_null() {
        let ref mut fresh13 = (*in_0).values;
        *fresh13 = tl_new_pair(
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
    if !(!arg.is_null() && (*arg).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
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
            let ref mut fresh14 = (*in_0).error;
            *fresh14 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"tl-display-sep with non-sym\0" as *const u8 as *const libc::c_char,
                ),
                arg,
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
    if (*(*arg).c2rust_unnamed.nm).here.len == 0 {
        if !((*in_0).error).is_null() {
            tl_new_sym(
                in_0,
                b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let ref mut fresh16 = (*in_0).error;
            *fresh16 = tl_new_sym(
                in_0,
                b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
            );
        };
        let ref mut fresh17 = (*in_0).values;
        *fresh17 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    (*in_0).disp_sep = *((*(*arg).c2rust_unnamed.nm).here.data).offset(0 as libc::c_int as isize);
    let ref mut fresh18 = (*in_0).values;
    *fresh18 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "87:1"]
static mut init_tl_cf_display_sep: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_display_sep
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-display-sep\0" as *const u8 as *const libc::c_char,
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
    let ref mut fresh19 = (*in_0).prefixes;
    *fresh19 = tl_new_pair(in_0, tl_new_pair(in_0, prefix, name), (*in_0).prefixes);
    let ref mut fresh20 = (*in_0).values;
    *fresh20 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "105:1"]
static mut init_tl_cf_prefix: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_prefix
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
            let ref mut fresh21 = (*in_0).error;
            *fresh21 = tl_new_sym(
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
static mut init_tl_cf_evalin: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_evalin
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
                b"bad call-with-current-continuation arity\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let ref mut fresh22 = (*in_0).error;
            *fresh22 = tl_new_sym(
                in_0,
                b"bad call-with-current-continuation arity\0" as *const u8 as *const libc::c_char,
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
    let ref mut fresh23 = (*in_0).values;
    *fresh23 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, cont, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "125:1"]
static mut init_tl_cf_call_with_current_continuation: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_call_with_current_continuation
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
            let ref mut fresh24 = (*in_0).error;
            *fresh24 = tl_new_sym(
                in_0,
                b"bad cons arity\0" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    let ref mut fresh25 = (*in_0).values;
    *fresh25 = tl_new_pair(
        in_0,
        tl_new_pair(
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
static mut init_tl_cf_cons: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_cons
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
            let ref mut fresh26 = (*in_0).error;
            *fresh26 = tl_new_sym(in_0, b"bad car arity\0" as *const u8 as *const libc::c_char);
        };
        return;
    }
    let ref mut fresh27 = (*in_0).values;
    *fresh27 = tl_new_pair(
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
                (*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
static mut init_tl_cf_car: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_car
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-car\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "142:1"]
static mut init_tl_cf_cdr: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_cdr
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-cdr\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh28 = (*in_0).error;
            *fresh28 = tl_new_sym(in_0, b"bad cdr arity\0" as *const u8 as *const libc::c_char);
        };
        return;
    }
    let ref mut fresh29 = (*in_0).values;
    *fresh29 = tl_new_pair(
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
                (*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
#[c2rust::src_loc = "147:1"]
static mut init_tl_cf_null: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_null
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-null?\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh30 = (*in_0).error;
            *fresh30 = tl_new_sym(
                in_0,
                b"bad null? arity\0" as *const u8 as *const libc::c_char,
            );
        };
        return;
    }
    let ref mut fresh31 = (*in_0).values;
    *fresh31 = tl_new_pair(
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
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn _tl_cf_if_k(
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
                    || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
            let ref mut fresh32 = (*in_0).error;
            *fresh32 = tl_new_sym(in_0, b"bad if arity\0" as *const u8 as *const libc::c_char);
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
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "161:1"]
static mut init_tl_cf_if: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_if
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-if\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
        };
        init
    })
};
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn _tl_cf_set_k(
    mut in_0: *mut tl_interp,
    mut result: *mut tl_object,
    mut key: *mut tl_object,
) {
    let ref mut fresh33 = (*in_0).env;
    *fresh33 = tl_env_set_global(
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
    let ref mut fresh34 = (*in_0).values;
    *fresh34 = tl_new_pair(
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
            let ref mut fresh35 = (*in_0).error;
            *fresh35 = tl_new_sym(in_0, b"bad set arity\0" as *const u8 as *const libc::c_char);
        };
        return;
    }
    if !(!key.is_null() && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
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
            let ref mut fresh36 = (*in_0).error;
            *fresh36 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"set! on non-sym\0" as *const u8 as *const libc::c_char,
                ),
                key,
            );
        };
        let ref mut fresh37 = (*in_0).values;
        *fresh37 = tl_new_pair(
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
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "172:1"]
static mut init_tl_cf_set: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_set
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
        let ref mut fresh38 = (*in_0).values;
        *fresh38 = tl_new_pair(
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
            let ref mut fresh39 = (*in_0).error;
            *fresh39 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"env of non-func or -macro\0" as *const u8 as *const libc::c_char,
                ),
                f,
            );
        };
        let ref mut fresh40 = (*in_0).values;
        *fresh40 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    let ref mut fresh41 = (*in_0).values;
    *fresh41 = tl_new_pair(
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
static mut init_tl_cf_env: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_env
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-env\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "191:1"]
static mut init_tl_cf_setenv: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_setenv
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-set-env!\0" as *const u8 as *const libc::c_char,
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
        let ref mut fresh42 = (*in_0).env;
        *fresh42 = first;
        let ref mut fresh43 = (*in_0).values;
        *fresh43 = tl_new_pair(
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
            let ref mut fresh44 = (*in_0).error;
            *fresh44 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"setenv on non-func or -macro\0" as *const u8 as *const libc::c_char,
                ),
                first,
            );
        };
        let ref mut fresh45 = (*in_0).values;
        *fresh45 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    let ref mut fresh46 = (*first).c2rust_unnamed.c2rust_unnamed_1.env;
    *fresh46 = next;
    let ref mut fresh47 = (*in_0).values;
    *fresh47 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[no_mangle]
#[c2rust::src_loc = "205:1"]
pub unsafe extern "C" fn tl_cf_topenv(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut _unused: *mut tl_object,
) {
    let ref mut fresh48 = (*in_0).values;
    *fresh48 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).top_env, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "205:1"]
static mut init_tl_cf_topenv: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_topenv
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-top-env\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "209:1"]
static mut init_tl_cf_concat: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_concat
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-concat\0" as *const u8 as *const libc::c_char,
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
        if !val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint {
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
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
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
                    b"builtin.c\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh49 = (*l_val).c2rust_unnamed.c2rust_unnamed.first;
            *fresh49 = val;
        }
        if !val.is_null() && (*val).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint {
            sz = (sz as libc::c_ulong).wrapping_add((*(*val).c2rust_unnamed.nm).here.len) as size_t
                as size_t;
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
                let ref mut fresh50 = (*in_0).error;
                *fresh50 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"concat on non-sym or int\0" as *const u8 as *const libc::c_char,
                    ),
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
    buffer =
        ((*in_0).reallocf).expect("non-null function pointer")(in_0, 0 as *mut libc::c_void, sz)
            as *mut libc::c_char;
    end = buffer;
    if buffer.is_null() {
        if !((*in_0).error).is_null() {
            tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
        } else {
            let ref mut fresh52 = (*in_0).error;
            *fresh52 = tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
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
            let fresh53 = src;
            src = src.offset(1);
            let fresh54 = end;
            end = end.offset(1);
            *fresh54 = *fresh53;
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
    let ref mut fresh55 = (*in_0).values;
    *fresh55 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "255:1"]
static mut init_tl_cf_length: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_length
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-length\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh56 = (*in_0).error;
            *fresh56 = tl_new_sym(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh57 = (*in_0).error;
            *fresh57 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"length on non-sym\0" as *const u8 as *const libc::c_char,
                ),
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        };
        let ref mut fresh58 = (*in_0).values;
        *fresh58 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    let ref mut fresh59 = (*in_0).values;
    *fresh59 = tl_new_pair(
        in_0,
        tl_new_pair(
            in_0,
            tl_new_int(
                in_0,
                (*(*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
#[c2rust::src_loc = "261:1"]
static mut init_tl_cf_ord: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_ord
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-ord\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh60 = (*in_0).error;
            *fresh60 = tl_new_sym(in_0, b"bad ord arity\0" as *const u8 as *const libc::c_char);
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh61 = (*in_0).error;
            *fresh61 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"ord on non-sym\0" as *const u8 as *const libc::c_char,
                ),
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        };
        let ref mut fresh62 = (*in_0).values;
        *fresh62 = tl_new_pair(
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
                    .first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh63 = (*in_0).error;
            *fresh63 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"ord on non-int\0" as *const u8 as *const libc::c_char,
                ),
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
        let ref mut fresh64 = (*in_0).values;
        *fresh64 = tl_new_pair(
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
            let ref mut fresh65 = (*in_0).error;
            *fresh65 = tl_new_pair(
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
    let ref mut fresh66 = (*in_0).values;
    *fresh66 = tl_new_pair(
        in_0,
        tl_new_pair(
            in_0,
            tl_new_int(
                in_0,
                *((*(*if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
#[c2rust::src_loc = "272:1"]
static mut init_tl_cf_chr: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_chr
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-chr\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh67 = (*in_0).error;
            *fresh67 = tl_new_sym(in_0, b"bad chr arity\0" as *const u8 as *const libc::c_char);
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh68 = (*in_0).error;
            *fresh68 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"chr on non-int\0" as *const u8 as *const libc::c_char,
                ),
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        };
        let ref mut fresh69 = (*in_0).values;
        *fresh69 = tl_new_pair(
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
    let ref mut fresh70 = (*in_0).values;
    *fresh70 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_sym(in_0, s.as_mut_ptr()), (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "280:1"]
static mut init_tl_cf_substr: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_substr
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-substr\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh71 = (*in_0).error;
            *fresh71 = tl_new_sym(
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
    if !(!sym.is_null() && (*sym).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
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
            let ref mut fresh72 = (*in_0).error;
            *fresh72 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"substr on non-sym\0" as *const u8 as *const libc::c_char,
                ),
                sym,
            );
        };
        let ref mut fresh73 = (*in_0).values;
        *fresh73 = tl_new_pair(
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
    if !(!start.is_null() && (*start).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
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
            let ref mut fresh74 = (*in_0).error;
            *fresh74 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"substr on non-int\0" as *const u8 as *const libc::c_char,
                ),
                start,
            );
        };
        let ref mut fresh75 = (*in_0).values;
        *fresh75 = tl_new_pair(
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
                let ref mut fresh76 = (*in_0).error;
                *fresh76 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    start,
                );
            };
            let ref mut fresh77 = (*in_0).values;
            *fresh77 = tl_new_pair(
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
    let ref mut fresh78 = (*in_0).values;
    *fresh78 = tl_new_pair(
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
#[c2rust::src_loc = "326:1"]
unsafe extern "C" fn _tl_readc_k(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut state: *mut tl_object,
) {
    let ref mut fresh79 = (*in_0).values;
    *fresh79 = tl_new_pair(
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
#[c2rust::src_loc = "331:1"]
static mut init_tl_cf_readc: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_readc
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-readc\0" as *const u8 as *const libc::c_char,
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
#[c2rust::src_loc = "335:1"]
static mut init_tl_cf_putbackc: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_putbackc
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-putbackc\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh80 = (*in_0).error;
            *fresh80 = tl_new_sym(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh81 = (*in_0).error;
            *fresh81 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"putback on non-int\0" as *const u8 as *const libc::c_char,
                ),
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
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
    let ref mut fresh83 = (*in_0).values;
    *fresh83 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "342:1"]
static mut init_tl_cf_writec: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_writec
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-writec\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh84 = (*in_0).error;
            *fresh84 = tl_new_sym(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh85 = (*in_0).error;
            *fresh85 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"write on non-int\0" as *const u8 as *const libc::c_char,
                ),
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        };
        let ref mut fresh86 = (*in_0).values;
        *fresh86 = tl_new_pair(
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
    let ref mut fresh87 = (*in_0).values;
    *fresh87 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "349:1"]
static mut init_tl_cf_add: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_add
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-+\0" as *const u8 as *const libc::c_char,
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
        if !(!val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            } else {
                let ref mut fresh88 = (*in_0).error;
                *fresh88 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            };
            let ref mut fresh89 = (*in_0).values;
            *fresh89 = tl_new_pair(
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
    let ref mut fresh90 = (*in_0).values;
    *fresh90 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
        (*in_0).values,
    );
}
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
        if !(!val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            } else {
                let ref mut fresh91 = (*in_0).error;
                *fresh91 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            };
            let ref mut fresh92 = (*in_0).values;
            *fresh92 = tl_new_pair(
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
    let ref mut fresh93 = (*in_0).values;
    *fresh93 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "358:1"]
static mut init_tl_cf_sub: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_sub
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl--\0" as *const u8 as *const libc::c_char,
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
        if !(!val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            } else {
                let ref mut fresh94 = (*in_0).error;
                *fresh94 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            };
            let ref mut fresh95 = (*in_0).values;
            *fresh95 = tl_new_pair(
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
    let ref mut fresh96 = (*in_0).values;
    *fresh96 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "373:1"]
static mut init_tl_cf_mul: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_mul
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-*\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "382:1"]
static mut init_tl_cf_div: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_div
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
        if !(!val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            } else {
                let ref mut fresh97 = (*in_0).error;
                *fresh97 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            };
            let ref mut fresh98 = (*in_0).values;
            *fresh98 = tl_new_pair(
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
    let ref mut fresh99 = (*in_0).values;
    *fresh99 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
        (*in_0).values,
    );
}
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
        if !(!val.is_null() && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
        {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            } else {
                let ref mut fresh100 = (*in_0).error;
                *fresh100 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                    val,
                );
            };
            let ref mut fresh101 = (*in_0).values;
            *fresh101 = tl_new_pair(
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
    let ref mut fresh102 = (*in_0).values;
    *fresh102 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "397:1"]
static mut init_tl_cf_mod: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_mod
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-%\0" as *const u8 as *const libc::c_char,
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
        let ref mut fresh103 = (*in_0).values;
        *fresh103 = tl_new_pair(
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
        let ref mut fresh104 = (*in_0).values;
        *fresh104 = tl_new_pair(
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
    let ref mut fresh105 = (*in_0).values;
    *fresh105 = tl_new_pair(
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
#[c2rust::src_loc = "412:1"]
static mut init_tl_cf_eq: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_eq
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-=\0" as *const u8 as *const libc::c_char,
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
        let ref mut fresh106 = (*in_0).values;
        *fresh106 = tl_new_pair(
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
        let ref mut fresh107 = (*in_0).values;
        *fresh107 = tl_new_pair(
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
        let ref mut fresh108 = (*in_0).error;
        *fresh108 = tl_new_pair(
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
    let ref mut fresh109 = (*in_0).values;
    *fresh109 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "423:1"]
static mut init_tl_cf_less: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_less
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-<\0" as *const u8 as *const libc::c_char,
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
        },
    );
    let ref mut fresh110 = (*in_0).values;
    *fresh110 = tl_new_pair(
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
#[c2rust::src_loc = "435:1"]
static mut init_tl_cf_nand: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_nand
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-nand\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "440:1"]
static mut init_tl_cf_type: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_type
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
        let ref mut fresh111 = (*in_0).values;
        *fresh111 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    if !obj.is_null() && (*obj).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint {
        let ref mut fresh112 = (*in_0).values;
        *fresh112 = tl_new_pair(
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
        let ref mut fresh113 = (*in_0).values;
        *fresh113 = tl_new_pair(
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
    if !obj.is_null() && (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint {
        let ref mut fresh114 = (*in_0).values;
        *fresh114 = tl_new_pair(
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
        let ref mut fresh115 = (*in_0).values;
        *fresh115 = tl_new_pair(
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
        let ref mut fresh116 = (*in_0).values;
        *fresh116 = tl_new_pair(
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
    if !obj.is_null() && (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
        let ref mut fresh117 = (*in_0).values;
        *fresh117 = tl_new_pair(
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
        let ref mut fresh118 = (*in_0).values;
        *fresh118 = tl_new_pair(
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
        let ref mut fresh119 = (*in_0).values;
        *fresh119 = tl_new_pair(
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
    let ref mut fresh120 = (*in_0).values;
    *fresh120 = tl_new_pair(
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
#[c2rust::src_loc = "457:1"]
static mut init_tl_cf_apply: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_apply
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
            let ref mut fresh121 = (*in_0).error;
            *fresh121 = tl_new_sym(
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
        let ref mut fresh122 = (*in_0).values;
        *fresh122 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !cur.is_null()
                    && (cur.is_null()
                        || (*cur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
    let ref mut fresh123 = (*in_0).values;
    *fresh123 = tl_new_pair(
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
#[c2rust::src_loc = "470:1"]
static mut init_tl_cf_gc: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_gc
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
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
    let ref mut fresh124 = (*in_0).values;
    *fresh124 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
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
#[c2rust::src_loc = "475:1"]
static mut init_tl_cf_read: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_read
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-read\0" as *const u8 as *const libc::c_char,
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
    if !(!name.is_null() && (*name).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint) {
        let ref mut fresh125 = (*in_0).values;
        *fresh125 = tl_new_pair(
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
            b"builtin.c\0" as *const u8 as *const libc::c_char,
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
    let ref mut fresh126 = (*in_0).values;
    *fresh126 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "479:1"]
static mut init_tl_cf_load_mod: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_load_mod
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-load-mod\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
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
            let ref mut fresh127 = (*in_0).error;
            *fresh127 = tl_new_sym(
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
    if !(!func.is_null() && (*func).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
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
            let ref mut fresh128 = (*in_0).error;
            *fresh128 = tl_new_pair(
                in_0,
                tl_new_sym(
                    in_0,
                    b"rescue on non-callable\0" as *const u8 as *const libc::c_char,
                ),
                func,
            );
        };
        let ref mut fresh129 = (*in_0).values;
        *fresh129 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
        return;
    }
    cont = tl_new_cont(in_0, (*in_0).env, (*in_0).conts, (*in_0).values);
    let ref mut fresh130 = (*in_0).rescue;
    *fresh130 = tl_new_pair(in_0, cont, (*in_0).rescue);
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
#[c2rust::src_loc = "500:1"]
static mut init_tl_cf_rescue: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_rescue
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-rescue\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
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
        cur = ((*cur).c2rust_unnamed_0.next_alloc_i & !TL_FMASK as libc::c_ulong) as *mut tl_object;
    }
    let ref mut fresh131 = (*in_0).values;
    *fresh131 = tl_new_pair(in_0, tl_new_pair(in_0, res, (*in_0).false_), (*in_0).values);
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "513:1"]
static mut init_tl_cf_all_objects: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_all_objects
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-all-objects\0" as *const u8 as *const libc::c_char,
            flags: 0x1 as libc::c_int as size_t,
        };
        init
    })
};
