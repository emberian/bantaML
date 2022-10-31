use libc::size_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "417:8"]
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
#[c2rust::src_loc = "83:1"]
pub type tl_object = tl_object_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "83:16"]
pub struct tl_object_s {
    pub kind: ObjectTag,
    pub data: ObjData,
    pub gclink: GcLink,
    pub prev_alloc: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "175:2"]
pub union GcLink {
    pub next_alloc: *mut tl_object_s,
    pub next_alloc_i: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "132:2"]
pub union ObjData {
    pub ival: libc::c_long,
    pub nm: *mut tl_name,
    pub pair: PairData,
    pub then: ThenData,
    pub func: FuncData,
    pub cont: ContData,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "161:3"]
pub struct ContData {
    pub ret_env: *mut tl_object_s,
    pub ret_conts: *mut tl_object_s,
    pub ret_values: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "151:3"]
pub struct FuncData {
    pub args: *mut tl_object_s,
    pub body: *mut tl_object_s,
    pub env: *mut tl_object_s,
    pub envn: *mut tl_object_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "143:3"]
pub struct ThenData {
    pub cfunc:
        Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object_s, *mut tl_object_s) -> ()>,
    pub state: *mut tl_object_s,
    pub name: *mut libc::c_char,
}
#[c2rust::src_loc = "72:1"]
pub type tl_interp = tl_interp_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "137:3"]
pub struct PairData {
    pub first: *mut tl_object_s,
    pub next: *mut tl_object_s,
}
#[c2rust::src_loc = "73:1"]
pub type tl_name = tl_name_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1043:8"]
pub struct tl_name_s {
    pub here: tl_buffer,
    pub num_children: size_t,
    pub sz_children: size_t,
    pub children: *mut tl_child,
    pub chain: *mut tl_name_s,
}
#[c2rust::src_loc = "1038:1"]
pub type tl_child = tl_child_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1038:16"]
pub struct tl_child_s {
    pub seg: tl_buffer,
    pub name: *mut tl_name_s,
}
#[c2rust::src_loc = "1033:1"]
pub type tl_buffer = tl_buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1033:16"]
pub struct tl_buffer_s {
    pub data: *mut libc::c_char,
    pub len: size_t,
}
#[c2rust::src_loc = "85:2"]
pub type ObjectTag = libc::c_uint;
#[c2rust::src_loc = "130:3"]
pub const TL_CONT: ObjectTag = 8;
#[c2rust::src_loc = "125:3"]
pub const TL_FUNC: ObjectTag = 7;
#[c2rust::src_loc = "120:3"]
pub const TL_MACRO: ObjectTag = 6;
#[c2rust::src_loc = "115:3"]
pub const TL_CFUNC_BYVAL: ObjectTag = 5;
#[c2rust::src_loc = "110:3"]
pub const TL_CFUNC: ObjectTag = 4;
#[c2rust::src_loc = "105:3"]
pub const TL_THEN: ObjectTag = 3;
#[c2rust::src_loc = "100:3"]
pub const TL_PAIR: ObjectTag = 2;
#[c2rust::src_loc = "95:3"]
pub const TL_SYM: ObjectTag = 1;
#[c2rust::src_loc = "90:3"]
pub const TL_INT: ObjectTag = 0;
#[c2rust::src_loc = "394:1"]
pub type tl_ns = tl_ns_s;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "394:16"]
pub struct tl_ns_s {
    pub root: *mut tl_name,
}
#[c2rust::src_loc = "992:1"]
pub type tl_init_ent = tl_init_ent_s;
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[c2rust::src_loc = "992:16"]
pub struct tl_init_ent_s(pub tl_init_ent_s_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "992:16"]
pub struct tl_init_ent_s_Inner {
    pub fn_0:
        Option<unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> ()>,
    pub name: *const libc::c_char,
    pub flags: size_t,
}
pub const TL_EF_BYVAL: libc::c_int = 1;

#[allow(dead_code, non_upper_case_globals)]
#[c2rust::src_loc = "992:16"]
pub const tl_init_ent_s_PADDING: usize =
    ::std::mem::size_of::<tl_init_ent_s>() - ::std::mem::size_of::<tl_init_ent_s_Inner>();
#[c2rust::src_loc = "27:9"]
pub const TL_DEFAULT_GC_EVENTS: libc::c_int = 65536 as libc::c_int;
#[c2rust::src_loc = "194:9"]
pub const TL_FMASK: libc::c_int = 0x3 as libc::c_int;
#[c2rust::src_loc = "201:9"]
pub const TL_F_MARK: libc::c_int = 0x1 as libc::c_int;
#[c2rust::src_loc = "217:9"]
pub const TL_F_PERMANENT: libc::c_int = 0x2 as libc::c_int;
#[c2rust::src_loc = "357:9"]
pub const TL_EMPTY_LIST: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "913:9"]
pub const TL_RESULT_DONE: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "922:9"]
pub const TL_RESULT_AGAIN: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "890:9"]
pub const TL_APPLY_DROP_EVAL: libc::c_int = -3;
#[c2rust::src_loc = "868:9"]
pub const TL_APPLY_PUSH_EVAL: libc::c_int = -(1 as libc::c_int);
#[c2rust::src_loc = "896:9"]
pub const TL_APPLY_DROP: libc::c_int = -(4 as libc::c_int);
#[c2rust::src_loc = "884:9"]
pub const TL_APPLY_INDIRECT: libc::c_int = -2;
#[c2rust::src_loc = "934:9"]
pub const TL_RESULT_GETCHAR: libc::c_int = 2 as libc::c_int;
#[c2rust::src_loc = "907:9"]
pub const TL_APPLY_GETCHAR: libc::c_int = -(6 as libc::c_int);
#[c2rust::src_loc = "902:9"]
pub const TL_APPLY_DROP_RESCUE: libc::c_int = -5;


mod remainder;
pub(crate) mod builtins;
pub use remainder::*;