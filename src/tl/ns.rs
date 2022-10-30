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
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
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
#[c2rust::header_src = "/usr/include/assert.h:2"]
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
use self::string_h::{memcmp, memcpy, memmove};
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
            b"ns.c\0" as *const u8 as *const libc::c_char,
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
            b"ns.c\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn tl_ns_split(
    mut in_0: *mut tl_interp,
    mut child: *mut tl_child,
    mut len: size_t,
) -> *mut tl_name {
    let mut new_name = 0 as *mut tl_name;
    if len > 0 as libc::c_int as libc::c_ulong && len < (*child).seg.len {
    } else {
        __assert_fail(
            b"len > 0 && len < child->seg.len\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
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
            b"ns.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"tl_name *tl_ns_split(tl_interp *, tl_child *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh0 = (*new_name).sz_children;
    *fresh0 = 1 as libc::c_int as size_t;
    (*new_name).num_children = *fresh0;
    let ref mut fresh1 = (*new_name).children;
    *fresh1 = ((*in_0).reallocf).expect("non-null function pointer")(
        in_0,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<tl_child>() as libc::c_ulong,
    ) as *mut tl_child;
    if !((*new_name).children).is_null() {
    } else {
        __assert_fail(
            b"new_name->children\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 54], &[libc::c_char; 54]>(
                b"tl_name *tl_ns_split(tl_interp *, tl_child *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh2 = (*(*new_name).children).name;
    *fresh2 = (*child).name;
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
    let ref mut fresh3 = (*child).seg.data;
    *fresh3 = ((*in_0).reallocf).expect("non-null function pointer")(
        in_0,
        (*child).seg.data as *mut libc::c_void,
        len,
    ) as *mut libc::c_char;
    let ref mut fresh4 = (*child).name;
    *fresh4 = new_name;
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
    'c_1049: loop {
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
                name.len = (name.len as libc::c_ulong).wrapping_sub(match_len) as size_t as size_t;
                continue 'c_1049;
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
                name.len = (name.len as libc::c_ulong).wrapping_sub(matching) as size_t as size_t;
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
        let ref mut fresh5 = (*cur).children;
        *fresh5 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            (*cur).children as *mut libc::c_void,
            (::std::mem::size_of::<tl_child>() as libc::c_ulong).wrapping_mul((*cur).sz_children),
        ) as *mut tl_child;
        if !((*cur).children).is_null() {
        } else {
            __assert_fail(
                b"cur->children\0" as *const u8 as *const libc::c_char,
                b"ns.c\0" as *const u8 as *const libc::c_char,
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
            b"ns.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"tl_name *tl_ns_resolve(tl_interp *, tl_ns *, tl_buffer)\0",
            ))
            .as_ptr(),
        );
    }
    let ref mut fresh6 = (*cur).num_children;
    *fresh6 = (*fresh6).wrapping_add(1);
    (*((*cur).children).offset(low as isize)).seg =
        tl_buf_slice(in_0, name, 0 as libc::c_int as size_t, name.len);
    let ref mut fresh7 = (*((*cur).children).offset(low as isize)).name;
    *fresh7 = ((*in_0).reallocf).expect("non-null function pointer")(
        in_0,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<tl_name>() as libc::c_ulong,
    ) as *mut tl_name_s;
    if !((*((*cur).children).offset(low as isize)).name).is_null() {
    } else {
        __assert_fail(
            b"cur->children[low].name\0" as *const u8 as *const libc::c_char,
            b"ns.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"tl_name *tl_ns_resolve(tl_interp *, tl_ns *, tl_buffer)\0",
            ))
            .as_ptr(),
        );
    }
    cur = (*((*cur).children).offset(low as isize)).name;
    (*cur).here = tl_buf_slice(in_0, whole_name, 0 as libc::c_int as size_t, whole_name.len);
    let ref mut fresh8 = (*cur).sz_children;
    *fresh8 = 0 as libc::c_int as size_t;
    (*cur).num_children = *fresh8;
    let ref mut fresh9 = (*cur).children;
    *fresh9 = NULL as *mut tl_child;
    return cur;
}
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn tl_ns_init(mut in_0: *mut tl_interp, mut ns: *mut tl_ns) {
    let ref mut fresh10 = (*ns).root;
    *fresh10 = ((*in_0).reallocf).expect("non-null function pointer")(
        in_0,
        0 as *mut libc::c_void,
        ::std::mem::size_of::<tl_name>() as libc::c_ulong,
    ) as *mut tl_name;
    let ref mut fresh11 = (*(*ns).root).here.data;
    *fresh11 = NULL as *mut libc::c_char;
    (*(*ns).root).here.len = 0 as libc::c_int as size_t;
    let ref mut fresh12 = (*(*ns).root).sz_children;
    *fresh12 = 0 as libc::c_int as size_t;
    (*(*ns).root).num_children = *fresh12;
    let ref mut fresh13 = (*(*ns).root).children;
    *fresh13 = NULL as *mut tl_child;
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
    let ref mut fresh14 = (*cur).chain;
    *fresh14 = NULL as *mut tl_name_s;
    while !cur.is_null() {
        index = 0 as libc::c_int as size_t;
        while index < (*cur).num_children {
            child = &mut *((*cur).children).offset(index as isize) as *mut tl_child;
            ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                (*child).seg.data as *mut libc::c_void,
                0 as libc::c_int as size_t,
            );
            let ref mut fresh15 = (*(*child).name).chain;
            *fresh15 = (*cur).chain;
            let ref mut fresh16 = (*cur).chain;
            *fresh16 = (*child).name;
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
unsafe extern "C" fn tl_ns_print_name(
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
unsafe extern "C" fn tl_ns_print_child(
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
    let ref mut fresh17 = (*cur).chain;
    *fresh17 = NULL as *mut tl_name_s;
    while !cur.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*cur).num_children {
            let ref mut fresh18 = (*(*((*cur).children).offset(i as isize)).name).chain;
            *fresh18 = (*cur).chain;
            let ref mut fresh19 = (*cur).chain;
            *fresh19 = (*((*cur).children).offset(i as isize)).name;
            i = i.wrapping_add(1);
        }
        cb.expect("non-null function pointer")(in_0, ns, cur, data);
        cur = (*cur).chain;
    }
}
#[c2rust::src_loc = "331:1"]
unsafe extern "C" fn _tl_add_symbol(
    mut in_0: *mut tl_interp,
    mut _ns: *mut tl_ns,
    mut name: *mut tl_name,
    mut data: *mut libc::c_void,
) {
    let mut cell = data as *mut tl_object;
    let ref mut fresh20 = (*cell).c2rust_unnamed.c2rust_unnamed.first;
    *fresh20 = tl_new_pair(
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
    let ref mut fresh21 = (*in_0).values;
    *fresh21 = tl_new_pair(
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
static mut init_tl_cf_all_symbols: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_all_symbols
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-all-symbols\0" as *const u8 as *const libc::c_char,
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
    let ref mut fresh22 = (*in_0).values;
    *fresh22 = tl_new_pair(
        in_0,
        tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
        (*in_0).values,
    );
}
#[link_section = "tl_init_ents"]
#[used]
#[c2rust::src_loc = "342:1"]
static mut init_tl_cf_print_ns: tl_init_ent = unsafe {
    tl_init_ent_s({
        let mut init = tl_init_ent_s_Inner {
            fn_0: Some(
                tl_cf_print_ns
                    as unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
            ),
            name: b"tl-print-ns\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as size_t,
        };
        init
    })
};
