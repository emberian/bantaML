use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/ctype.h:2"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
use crate::tl::tinylisp_h::*;
#[c2rust::header_src = "/usr/include/assert.h:3"]
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
pub use self::ctype_h::{
    C2RustUnnamed, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint,
    _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc,
};
pub use self::stddef_h::{size_t, NULL};
#[c2rust::src_loc = "13:9"]
pub const DEFAULT_SYM_LEN: libc::c_int = 64 as libc::c_int;
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
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn _tl_read_top_prefix_k(
    mut in_0: *mut tl_interp,
    mut args: *mut tl_object,
    mut state: *mut tl_object,
) {
    let ref mut fresh0 = (*in_0).values;
    *fresh0 = tl_new_pair(
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
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn _tl_read_top_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh1 = (*in_0).error;
            *fresh1 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh2 = (*in_0).values;
        *fresh2 = tl_new_pair(
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
            let ref mut fresh3 = (*in_0).values;
            *fresh3 = tl_new_pair(
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
                                ) -> (),
                        ),
                        b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                    );
                    return;
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
unsafe extern "C" fn _tl_read_comment_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh4 = (*in_0).error;
            *fresh4 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh5 = (*in_0).values;
        *fresh5 = tl_new_pair(
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
            let ref mut fresh6 = (*in_0).values;
            *fresh6 = tl_new_pair(
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
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn _tl_read_string_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh7 = (*in_0).error;
            *fresh7 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh8 = (*in_0).values;
        *fresh8 = tl_new_pair(
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
        let ref mut fresh9 = (*in_0).read_buffer;
        *fresh9 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            (*in_0).read_sz,
        ) as *mut libc::c_char;
        if !((*in_0).read_buffer).is_null() {
        } else {
            __assert_fail(
                b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                b"read.c\0" as *const u8 as *const libc::c_char,
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
                let ref mut fresh10 = (*in_0).error;
                *fresh10 = tl_new_sym(in_0, b"EOF in string\0" as *const u8 as *const libc::c_char);
            };
            let ref mut fresh11 = (*in_0).values;
            *fresh11 = tl_new_pair(
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
            let ref mut fresh12 = (*in_0).read_buffer;
            *fresh12 = NULL as *mut libc::c_char;
            let ref mut fresh13 = (*in_0).values;
            *fresh13 = tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
            return;
        }
        _ => {
            let ref mut fresh14 = (*in_0).read_ptr;
            let fresh15 = *fresh14;
            *fresh14 = (*fresh14).wrapping_add(1);
            *((*in_0).read_buffer).offset(fresh15 as isize) = ch as libc::c_char;
            if (*in_0).read_ptr >= (*in_0).read_sz {
                (*in_0).read_sz <<= 1 as libc::c_int;
                let ref mut fresh16 = (*in_0).read_buffer;
                *fresh16 = ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    (*in_0).read_buffer as *mut libc::c_void,
                    (*in_0).read_sz,
                ) as *mut libc::c_char;
                if !((*in_0).read_buffer).is_null() {
                } else {
                    __assert_fail(
                        b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                        b"read.c\0" as *const u8 as *const libc::c_char,
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
#[c2rust::src_loc = "177:1"]
unsafe extern "C" fn _tl_read_pair_cons_k(
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
unsafe extern "C" fn _tl_read_pair_improp_check_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh17 = (*in_0).error;
            *fresh17 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh18 = (*in_0).values;
        *fresh18 = tl_new_pair(
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
            let ref mut fresh19 = (*in_0).values;
            *fresh19 = tl_new_pair(
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
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
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
unsafe extern "C" fn _tl_read_pair_improp_k(
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
unsafe extern "C" fn _tl_read_list_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh20 = (*in_0).error;
            *fresh20 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh21 = (*in_0).values;
        *fresh21 = tl_new_pair(
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
            let ref mut fresh22 = (*in_0).values;
            *fresh22 = tl_new_pair(
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
unsafe extern "C" fn _tl_read_int_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh23 = (*in_0).error;
            *fresh23 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh24 = (*in_0).values;
        *fresh24 = tl_new_pair(
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
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            b"tl_getc_and_then:_tl_read_int_k\0" as *const u8 as *const libc::c_char,
        );
    } else {
        (*in_0).is_putback = 1 as libc::c_int;
        (*in_0).putback = ch;
        let ref mut fresh25 = (*in_0).values;
        *fresh25 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, state, (*in_0).false_),
            (*in_0).values,
        );
        return;
    };
}
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn _tl_read_sym_k(
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
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).c2rust_unnamed.c2rust_unnamed.first
                } else {
                    0 as *mut tl_object_s
                },
            );
        } else {
            let ref mut fresh26 = (*in_0).error;
            *fresh26 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
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
        let ref mut fresh27 = (*in_0).values;
        *fresh27 = tl_new_pair(
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
        let ref mut fresh28 = (*in_0).read_buffer;
        *fresh28 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            (*in_0).read_sz,
        ) as *mut libc::c_char;
        if !((*in_0).read_buffer).is_null() {
        } else {
            __assert_fail(
                b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                b"read.c\0" as *const u8 as *const libc::c_char,
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
            let ref mut fresh31 = (*in_0).read_ptr;
            let fresh32 = *fresh31;
            *fresh31 = (*fresh31).wrapping_add(1);
            *((*in_0).read_buffer).offset(fresh32 as isize) = ch as libc::c_char;
            if (*in_0).read_ptr >= (*in_0).read_sz {
                (*in_0).read_sz <<= 1 as libc::c_int;
                let ref mut fresh33 = (*in_0).read_buffer;
                *fresh33 = ((*in_0).reallocf).expect("non-null function pointer")(
                    in_0,
                    (*in_0).read_buffer as *mut libc::c_void,
                    (*in_0).read_sz,
                ) as *mut libc::c_char;
                if !((*in_0).read_buffer).is_null() {
                } else {
                    __assert_fail(
                        b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                        b"read.c\0" as *const u8 as *const libc::c_char,
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
    let ref mut fresh29 = (*in_0).read_buffer;
    *fresh29 = NULL as *mut libc::c_char;
    let ref mut fresh30 = (*in_0).values;
    *fresh30 = tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
    return;
}
