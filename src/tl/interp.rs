use ::libc;
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
use crate::tl::tinylisp_h::*;
#[c2rust::header_src = "/usr/include/stdio.h:7"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "520:1"]
        pub fn getchar() -> libc::c_int;
        #[c2rust::src_loc = "556:1"]
        pub fn putchar(__c: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:8"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "551:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "555:13"]
        pub fn free(_: *mut libc::c_void);
    }
}
pub use self::stddef_h::{size_t, NULL};
use self::stdio_h::{getchar, putchar};
use self::stdlib_h::{free, realloc};
extern "C" {
    #[c2rust::src_loc = "3:20"]
    pub static mut __start_tl_init_ents: tl_init_ent;
    #[c2rust::src_loc = "3:42"]
    pub static mut __stop_tl_init_ents: tl_init_ent;
}
#[c2rust::src_loc = "9:1"]
unsafe extern "C" fn _readf(mut in_0: *mut tl_interp) -> libc::c_int {
    return getchar();
}
#[c2rust::src_loc = "10:1"]
unsafe extern "C" fn _writef(mut in_0: *mut tl_interp, c: libc::c_char) {
    putchar(c as libc::c_int);
}
#[c2rust::src_loc = "11:1"]
unsafe extern "C" fn _modloadf(
    mut in_0: *mut tl_interp,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "12:1"]
unsafe extern "C" fn _reallocf(
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
    let ref mut fresh0 = (*in_0).reallocf;
    *fresh0 = reallocf;
    let ref mut fresh1 = (*in_0).readf;
    *fresh1 = Some(_readf as unsafe extern "C" fn(*mut tl_interp) -> libc::c_int);
    let ref mut fresh2 = (*in_0).writef;
    *fresh2 = Some(_writef as unsafe extern "C" fn(*mut tl_interp, libc::c_char) -> ());
    let ref mut fresh3 = (*in_0).modloadf;
    *fresh3 =
        Some(_modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int);
    tl_ns_init(in_0, &mut (*in_0).ns);
    let ref mut fresh4 = (*in_0).top_alloc;
    *fresh4 = NULL as *mut tl_object;
    let ref mut fresh5 = (*in_0).true_;
    *fresh5 = tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char);
    let ref mut fresh6 = (*in_0).false_;
    *fresh6 = tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char);
    let ref mut fresh7 = (*in_0).error;
    *fresh7 = NULL as *mut tl_object;
    let ref mut fresh8 = (*in_0).prefixes;
    *fresh8 = TL_EMPTY_LIST as *mut tl_object;
    let ref mut fresh9 = (*in_0).current;
    *fresh9 = TL_EMPTY_LIST as *mut tl_object;
    let ref mut fresh10 = (*in_0).conts;
    *fresh10 = TL_EMPTY_LIST as *mut tl_object;
    let ref mut fresh11 = (*in_0).values;
    *fresh11 = TL_EMPTY_LIST as *mut tl_object;
    let ref mut fresh12 = (*in_0).rescue;
    *fresh12 = TL_EMPTY_LIST as *mut tl_object;
    (*in_0).gc_events = TL_DEFAULT_GC_EVENTS as size_t;
    (*in_0).ctr_events = 0 as libc::c_int as size_t;
    (*in_0).putback = 0 as libc::c_int;
    (*in_0).is_putback = 0 as libc::c_int;
    let ref mut fresh13 = (*in_0).read_buffer;
    *fresh13 = NULL as *mut libc::c_char;
    (*in_0).disp_sep = '\t' as i32 as libc::c_char;
    let ref mut fresh14 = (*in_0).top_env;
    *fresh14 = TL_EMPTY_LIST as *mut tl_object;
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
    let ref mut fresh15 = (*in_0).top_env;
    *fresh15 = tl_new_pair(in_0, top_frm, (*in_0).top_env);
    let ref mut fresh16 = (*in_0).env;
    *fresh16 = (*in_0).top_env;
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn tl_interp_cleanup(mut in_0: *mut tl_interp) {
    while !((*in_0).top_alloc).is_null() {
        tl_free(in_0, (*in_0).top_alloc);
    }
    tl_ns_free(in_0, &mut (*in_0).ns);
}
