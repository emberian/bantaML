extern "C" {
    fn tl_new_sym(_: *mut tl_interp, _: *const libc::c_char) -> *mut tl_object;
    fn tl_new_pair(_: *mut tl_interp, _: *mut tl_object, _: *mut tl_object) -> *mut tl_object;
    fn _tl_new_cfunc(
        _: *mut tl_interp,
        _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        _: *const libc::c_char,
    ) -> *mut tl_object;
    fn _tl_new_cfunc_byval(
        _: *mut tl_interp,
        _: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
        _: *const libc::c_char,
    ) -> *mut tl_object;
    fn tl_free(_: *mut tl_interp, _: *mut tl_object);
    fn tl_ns_free(_: *mut tl_interp, _: *mut tl_ns);
    fn tl_ns_init(_: *mut tl_interp, _: *mut tl_ns);
    static mut __start_tl_init_ents: tl_init_ent;
    static mut __stop_tl_init_ents: tl_init_ent;
    fn getchar() -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub type tl_init_ent = tl_init_ent_s;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
pub struct tl_init_ent_s(pub tl_init_ent_s_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tl_init_ent_s_Inner {
    pub fn_0: Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    pub name: *const libc::c_char,
    pub flags: size_t,
}
#[allow(dead_code, non_upper_case_globals)]
const tl_init_ent_s_PADDING: usize =
    ::std::mem::size_of::<tl_init_ent_s>() - ::std::mem::size_of::<tl_init_ent_s_Inner>();
unsafe extern "C" fn _readf(mut in_0: *mut tl_interp) -> libc::c_int {
    return getchar();
}
unsafe extern "C" fn _writef(mut in_0: *mut tl_interp, c: libc::c_char) {
    putchar(c as libc::c_int);
}
unsafe extern "C" fn _modloadf(
    mut in_0: *mut tl_interp,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn _reallocf(
    mut in_0: *mut tl_interp,
    mut ptr: *mut libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    if s == 0 {
        free(ptr);
        return 0 as *mut libc::c_void;
    }
    return realloc(ptr, s);
}
#[no_mangle]
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
    *fresh4 = 0 as *mut tl_object;
    let ref mut fresh5 = (*in_0).true_;
    *fresh5 = tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char);
    let ref mut fresh6 = (*in_0).false_;
    *fresh6 = tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char);
    let ref mut fresh7 = (*in_0).error;
    *fresh7 = 0 as *mut tl_object;
    let ref mut fresh8 = (*in_0).prefixes;
    *fresh8 = 0 as *mut tl_object;
    let ref mut fresh9 = (*in_0).current;
    *fresh9 = 0 as *mut tl_object;
    let ref mut fresh10 = (*in_0).conts;
    *fresh10 = 0 as *mut tl_object;
    let ref mut fresh11 = (*in_0).values;
    *fresh11 = 0 as *mut tl_object;
    let ref mut fresh12 = (*in_0).rescue;
    *fresh12 = 0 as *mut tl_object;
    (*in_0).gc_events = 65536 as libc::c_int as size_t;
    (*in_0).ctr_events = 0 as libc::c_int as size_t;
    (*in_0).putback = 0 as libc::c_int;
    (*in_0).is_putback = 0 as libc::c_int;
    let ref mut fresh13 = (*in_0).read_buffer;
    *fresh13 = 0 as *mut libc::c_char;
    (*in_0).disp_sep = '\t' as i32 as libc::c_char;
    let ref mut fresh14 = (*in_0).top_env;
    *fresh14 = 0 as *mut tl_object;
    let mut top_frm: *mut tl_object = 0 as *mut tl_object;
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
pub unsafe extern "C" fn tl_interp_cleanup(mut in_0: *mut tl_interp) {
    while !((*in_0).top_alloc).is_null() {
        tl_free(in_0, (*in_0).top_alloc);
    }
    tl_ns_free(in_0, &mut (*in_0).ns);
}
