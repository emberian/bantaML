use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}

macro_rules! tl_first {
    ($e:expr) => ({ let obj = $e; if !obj.is_null() { (*obj).c2rust_unnamed.c2rust_unnamed.next } else { std::ptr::null_mut() }})
}

macro_rules! tl_is_pair {
    ($e:expr) => (todo!())
}
macro_rules! tl_mark { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i |= TL_F_MARK as _) } }

macro_rules! tl_unmark { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i &= !TL_FMASK as _) } }

macro_rules! tl_is_marked { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i & TL_F_MARK as _) } }

macro_rules! tl_make_permanent { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i |= TL_F_PERMANENT as _) } }

macro_rules! tl_make_transient { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i &= ~TL_F_PERMANENT as _) } }

macro_rules! tl_is_permanent { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i & crate::tinylisp::TL_F_PERMANENT as _) } }

macro_rules! tl_next_alloc { ($obj:expr) => { todo!() }} //  ((tl_object *)((obj)->next_alloc_i & (~TL_FMASK))) } }

macro_rules! tl_make_next_alloc { ($ptr:expr) => {  todo!() } } // ((tl_object *)(((obj)->next_alloc_i & (~TL_FMASK)) | (((size_t)(orig)) & TL_FMASK))) } }

macro_rules! tl_is_int { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tinylisp::TL_INT) } } }
macro_rules! tl_is_sym { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tinylisp::TL_SYM) } } }

macro_rules! tl_is_pair { ($obj:expr) => {  { let obj =$obj;(obj.is_null() || (*obj).kind == crate::tinylisp::TL_PAIR) } }}
macro_rules! tl_is_then { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tinylisp::TL_THEN) } } }
macro_rules! tl_is_cfunc { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tinylisp::TL_CFUNC) } }}
macro_rules! tl_is_cfunc_byval { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tinylisp::TL_CFUNC_BYVAL) } }}
macro_rules! tl_is_macro { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tinylisp::TL_MACRO) } }}
macro_rules! tl_is_func { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tinylisp::TL_FUNC) } }}
macro_rules! tl_is_cont { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tinylisp::TL_CONT) } }}
macro_rules! tl_is_callable { ($obj:expr) => { { let obj =$obj; (tl_is_cfunc!(obj) || tl_is_cfunc_byval!(obj) || tl_is_then!(obj)|| tl_is_macro!(obj) || tl_is_func!(obj) || tl_is_cont!(obj)) } }}

macro_rules! tl_first { ($obj:expr) => { { let obj = $obj; (if (!obj.is_null() && tl_is_pair!(obj))  { (*obj).data.pair.first }else{std::ptr::null_mut()}) } } }
macro_rules! tl_next { ($obj:expr) => {  { let obj = $obj; (if (!obj.is_null() && tl_is_pair!(obj))  { (*obj).data.pair.next } else {std::ptr::null_mut()}) } } }

macro_rules! tl_list_iter { ($it:expr) => {  todo!() } }
macro_rules! tl_sym_eq { ($b:expr) => {  (tl_is_sym(a) && tl_is_sym(b) && (a)->nm == (b)->nm) } }
macro_rules! tl_sym_less { ($b:expr) => {  (tl_is_sym(a) && tl_is_sym(b) && ((a)->nm->here.len < (b)->nm->here.len && !((b)->nm->here.len < (a)->nm->here.len) || memcmp((a)->nm->here.data, (b)->nm->here.data, (a)->nm->here.len) < 0)) } }

macro_rules! tl_error_set { ($er:expr) => {  ((in)->error ? (er) : ((in)->error = (er))) } }
macro_rules! tl_error_clear { ($in:expr) => {  ((in)->error = NULL) } }
macro_rules! tl_has_error { ($in:expr) => {  ((in)->error) } }

macro_rules! tl_getc { ($in:expr) => {  ((in)->is_putback ? ((in)->is_putback = 0, (in)->putback) : (in)->readf((in))) } }
macro_rules! tl_putback { ($c:expr) => {  ((in)->is_putback = 1, (in)->putback = (c)) } }
macro_rules! tl_putc { ($in:expr,$c:expr) => { {let in0 = $in; ((*in0).writef.unwrap())(in0,$c as i8); }} }

macro_rules! tl_alloc_realloc { ($n:expr) => {  ((in)->reallocf((in), (p), (n))) } }

macro_rules! tl_alloc_malloc { ($n:expr) => {  tl_alloc_realloc(in, NULL, n) } }
macro_rules! tl_alloc_free { ($ptr:expr) => {  tl_alloc_realloc(in, ptr, 0) } }

macro_rules! tl_values_push { ($v:expr) => {  (in)->values = tl_new_pair((in), tl_new_pair((in), (v), (in)->false_), (in)->values) } }

macro_rules! tl_values_push_syntactic { ($v:expr) => {  (in)->values = tl_new_pair((in), tl_new_pair((in), (v), (in)->true_), (in)->values) } }
macro_rules! tl_values_pop_into { ($var:expr) => {  todo!() } }
macro_rules! tl_rescue_push { ($cont:expr) => {  (in)->rescue = tl_new_pair((in), (cont), (in)->rescue); } }
macro_rules! tl_rescue_peek { ($in:expr) => {  tl_first((in)->rescue) } }
macro_rules! tl_rescue_drop { ($in:expr) => {  (in)->rescue = tl_next((in)->rescue) } }
macro_rules! tl_cfunc_return { ($v:expr) => {  do { tl_values_push((in), (v)); return; } while(0) } }

pub use self::tinylisp_h::*;
/*

#define tl_mark(obj) ((obj)->next_alloc_i |= TL_F_MARK)

#define tl_unmark(obj) ((obj)->next_alloc_i &= ~TL_FMASK)

#define tl_is_marked(obj) ((obj)->next_alloc_i & TL_F_MARK)

#define tl_make_permanent(obj) ((obj)->next_alloc_i |= TL_F_PERMANENT)

#define tl_make_transient(obj) ((obj)->next_alloc_i &= ~TL_F_PERMANENT)

#define tl_is_permanent(obj) ((obj)->next_alloc_i & TL_F_PERMANENT)

#define tl_next_alloc(obj) ((tl_object *)((obj)->next_alloc_i & (~TL_FMASK)))

#define tl_make_next_alloc(orig, ptr) ((tl_object *)(((obj)->next_alloc_i & (~TL_FMASK)) | (((size_t)(orig)) & TL_FMASK)))

#define tl_is_int(obj) (!obj.is_null() && (obj)->kind == TL_INT)
#define tl_is_sym(obj) (!obj.is_null() && (obj)->kind == TL_SYM)

#define tl_is_pair(obj) (!(obj) || (obj)->kind == TL_PAIR)
#define tl_is_then(obj) (!obj.is_null() && (obj)->kind == TL_THEN)
#define tl_is_cfunc(obj) (!obj.is_null() && (obj)->kind == TL_CFUNC)
#define tl_is_cfunc_byval(obj) (!obj.is_null() && (obj)->kind == TL_CFUNC_BYVAL)
#define tl_is_macro(obj) (!obj.is_null() && (obj)->kind == TL_MACRO)
#define tl_is_func(obj) (!obj.is_null() && (obj)->kind == TL_FUNC)
#define tl_is_cont(obj) (!obj.is_null() && (obj)->kind == TL_CONT)
#define tl_is_callable(obj) (tl_is_cfunc(obj) || tl_is_cfunc_byval(obj) || tl_is_then(obj)|| tl_is_macro(obj) || tl_is_func(obj) || tl_is_cont(obj))

#define tl_first(obj) ((!obj.is_null() && tl_is_pair(obj)) ? (obj)->first : NULL)
#define tl_next(obj) ((!obj.is_null() && tl_is_pair(obj)) ? (obj)->next : NULL)

#define tl_list_iter(obj, it) tl_object *l_##it = obj, *it = tl_first(obj); l_##it; l_##it = tl_next(l_##it), it = tl_first(l_##it)
TL_EXTERN size_t tl_list_len(tl_object *);
TL_EXTERN tl_object *tl_list_rvs(tl_interp *, tl_object *);
TL_EXTERN tl_object *tl_list_rvs_improp(tl_interp *, tl_object *);

#define tl_sym_eq(a, b) (tl_is_sym(a) && tl_is_sym(b) && (a)->nm == (b)->nm)
#define tl_sym_less(a, b) (tl_is_sym(a) && tl_is_sym(b) && ((a)->nm->here.len < (b)->nm->here.len && !((b)->nm->here.len < (a)->nm->here.len) || memcmp((a)->nm->here.data, (b)->nm->here.data, (a)->nm->here.len) < 0))

#define tl_error_set(in, er) ((in)->error ? (er) : ((in)->error = (er)))
#define tl_error_clear(in) ((in)->error = NULL)
#define tl_has_error(in) ((in)->error)

#define tl_getc(in) ((in)->is_putback ? ((in)->is_putback = 0, (in)->putback) : (in)->readf((in)))
#define tl_putback(in, c) ((in)->is_putback = 1, (in)->putback = (c))
#define tl_putc(in, c) ((in)->writef((in), (c)))

#define tl_alloc_realloc(in, p, n) ((in)->reallocf((in), (p), (n)))

#define tl_alloc_malloc(in, n) tl_alloc_realloc(in, NULL, n)
#define tl_alloc_free(in, ptr) tl_alloc_realloc(in, ptr, 0)

#define tl_values_push(in, v) (in)->values = tl_new_pair((in), tl_new_pair((in), (v), (in)->false_), (in)->values)

#define tl_values_push_syntactic(in, v) (in)->values = tl_new_pair((in), tl_new_pair((in), (v), (in)->true_), (in)->values)
#define tl_values_pop_into(in, var) do { \
	var = tl_first(tl_first((in)->values)); \
	(in)->values = tl_next((in)->values); \
} while(0)
#define tl_rescue_push(in, cont) (in)->rescue = tl_new_pair((in), (cont), (in)->rescue);
#define tl_rescue_peek(in) tl_first((in)->rescue)
#define tl_rescue_drop(in) (in)->rescue = tl_next((in)->rescue)
#define tl_cfunc_return(in, v) do { tl_values_push((in), (v)); return; } while(0)
*/
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stddef.h:1"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "89:11"]
    pub const NULL_0: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/usr/lib/llvm-14/lib/clang/14.0.0/include/stdarg.h:1"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types.h:1"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_FILE.h:1"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::stddef_h::size_t;
    use super::types_h::{__off64_t, __off_t};
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h:1"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/ember/src/tinylisp/tinylisp.h:1"]
pub mod tinylisp_h {
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
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/home/ember/src/tinylisp/print.c:9"]
pub mod print_c {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:2"]
    pub union C2RustUnnamed_6 {
        pub s: *const libc::c_char,
        pub b: *mut tl_buffer,
    }
    #[c2rust::src_loc = "7:9"]
    pub const QUOTED_SYM_CHARS: [libc::c_char; 22] = unsafe {
        *::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
            b"0123456789.,'\"` \n\r\t\x08\x0B\0",
        )
    };
    #[no_mangle]
    #[c2rust::src_loc = "9:1"]
    pub unsafe extern "C" fn _print_pairs(mut in_0: *mut tl_interp, mut cur: *mut tl_object) {
        while !cur.is_null() {
            if tl_is_pair!(cur) {
                tl_printf(in_0, b". \0" as *const u8 as *const libc::c_char);
                tl_print(in_0, cur);
                cur = NULL as *mut tl_object;
            } else {
                tl_print(in_0, tl_first!(cur));
                if !tl_next!(cur).is_null() {
                    tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                }
                cur = tl_next!(cur);
            }
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "25:1"]
    pub unsafe extern "C" fn _mempbrk(
        mut m: *mut libc::c_char,
        mut s: *mut libc::c_char,
        mut sz: size_t,
    ) -> *mut libc::c_char {
        let mut i: size_t = 0;
        let mut t = 0 as *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < sz {
            t = s;
            while *t != 0 {
                if *m as libc::c_int == *t as libc::c_int {
                    return m;
                }
                t = t.offset(1);
            }
            i = i.wrapping_add(1);
            m = m.offset(1);
        }
        return NULL as *mut libc::c_char;
    }
    #[no_mangle]
    #[c2rust::src_loc = "36:1"]
    pub unsafe extern "C" fn tl_print(
        mut in_0: *mut tl_interp,
        mut obj: *mut tl_object,
    ) -> *mut tl_object {
        let mut cur = 0 as *mut tl_object;
        if obj.is_null() {
            tl_printf(in_0, b"()\0" as *const u8 as *const libc::c_char);
            return (*in_0).true_;
        }
        match (*obj).kind as libc::c_uint {
            0 => {
                tl_printf(
                    in_0,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*obj).data.ival,
                );
            }
            1 => {
                if (*(*obj).data.nm).here.len == 0 as libc::c_int as libc::c_ulong
                    || !(_mempbrk(
                        (*(*obj).data.nm).here.data,
                        QUOTED_SYM_CHARS.as_mut_ptr(),
                        (*(*obj).data.nm).here.len,
                    ))
                    .is_null()
                {
                    tl_putc!(in_0, '"');
                    tl_write(
                        in_0,
                        (*(*obj).data.nm).here.data,
                        (*(*obj).data.nm).here.len,
                    );
                    tl_putc!(in_0, '"');
                } else {
                    tl_write(
                        in_0,
                        (*(*obj).data.nm).here.data,
                        (*(*obj).data.nm).here.len,
                    );
                }
            }
            2 => {
                tl_printf(in_0, b"(\0" as *const u8 as *const libc::c_char);
                _print_pairs(in_0, obj);
                tl_printf(in_0, b")\0" as *const u8 as *const libc::c_char);
            }
            4 | 5 | 3 => {
                tl_printf(
                    in_0,
                    b"%s:%p\0" as *const u8 as *const libc::c_char,
                    if !((*obj).data.then.name).is_null() {
                        (*obj).data.then.name as *const libc::c_char
                    } else if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
                    {
                        b"<cfunc>\0" as *const u8 as *const libc::c_char
                    } else if (*obj).kind as libc::c_uint
                        == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                    {
                        b"<cfunc_byval>\0" as *const u8 as *const libc::c_char
                    } else {
                        b"<then>\0" as *const u8 as *const libc::c_char
                    },
                    (*obj).data.then.cfunc,
                );
            }
            6 | 7 => {
                tl_printf(
                    in_0,
                    b"(%s \0" as *const u8 as *const libc::c_char,
                    if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                        b"macro\0" as *const u8 as *const libc::c_char
                    } else {
                        b"lambda\0" as *const u8 as *const libc::c_char
                    },
                );
                tl_print(in_0, (*obj).data.func.args);
                tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                if tl_is_macro!(obj) {
                    tl_print(in_0, (*obj).data.func.envn);
                    tl_printf(in_0, b" \0" as *const u8 as *const libc::c_char);
                }
                _print_pairs(in_0, (*obj).data.func.body);
                tl_printf(in_0, b")\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                tl_printf(in_0, b"cont:%p\0" as *const u8 as *const libc::c_char, obj);
            }
            _ => {
                tl_printf(
                    in_0,
                    b"<unknown object kind %d>\0" as *const u8 as *const libc::c_char,
                    (*obj).kind as libc::c_uint,
                );
            }
        }
        return (*in_0).true_;
    }
    #[no_mangle]
    #[c2rust::src_loc = "98:1"]
    pub unsafe extern "C" fn tl_puts(mut in_0: *mut tl_interp, mut s: *const libc::c_char) {
        while *s as libc::c_int != 0 as libc::c_int {
            tl_putc!(in_0, {let v = * s ; s = s.offset(1); v}).expect("non-null function pointer")(
                tl_putc!(in_0, {let v = * s ; s = s.offset(1); v}),
                tl_putc!(in_0, {let v = * s ; s = s.offset(1); v}),
            );
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "102:1"]
    pub unsafe extern "C" fn tl_write(
        mut in_0: *mut tl_interp,
        mut data: *const libc::c_char,
        mut len: size_t,
    ) {
        let mut i = 0 as libc::c_int as size_t;
        loop {
            let fresh248 = i;
            i = i.wrapping_add(1);
            if !(fresh248 < len) {
                break;
            }
            tl_putc!(in_0, {let v = * data ; data = data.offset(1); v});
    }
    #[no_mangle]
    #[c2rust::src_loc = "107:1"]
    pub unsafe extern "C" fn tl_printf(
        mut in_0: *mut tl_interp,
        mut cur: *const libc::c_char,
        mut args: ...
    ) {
        let mut ap: ::std::ffi::VaListImpl;
        let mut temp = C2RustUnnamed_6 {
            s: 0 as *const libc::c_char,
        };
        let mut buf: [libc::c_char; 32] = [0; 32];
        ap = args.clone();
        while *cur as libc::c_int != 0 as libc::c_int {
            if *cur as libc::c_int == '%' as i32 {
                cur = cur.offset(1);
                match *cur as libc::c_int {
                    0 => {
                        tl_putc!(in_0, '%');
                    }
                    37 => {
                        tl_putc!(in_0, '%');
                        cur = cur.offset(1);
                    }
                    115 => {
                        temp.s = va_arg!(ap, const char *);
                        if (temp.s).is_null() {
                            tl_puts(in_0, b"<NULL>\0" as *const u8 as *const libc::c_char);
                        } else {
                            tl_puts(in_0, temp.s);
                        }
                        cur = cur.offset(1);
                    }
                    112 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%p\0" as *const u8 as *const libc::c_char,
                            va_arg!(ap, void *),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    108 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            va_arg!(ap, long),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(2 as libc::c_int as isize);
                    }
                    100 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            va_arg!(ap, int),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    78 => {
                        temp.b = va_arg!(ap, tl_buffer *);
                        snprintf(
                            buf.as_mut_ptr(),
                            32 as libc::c_int as libc::c_ulong,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (*temp.b).len,
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        tl_putc!(in_0, ':').expect("non-null function pointer")(
                            tl_putc!(in_0, ':'),
                            tl_putc!(in_0, ':'),
                        );
                        tl_write(in_0, (*temp.b).data, (*temp.b).len);
                        cur = cur.offset(1);
                    }
                    79 => {
                        tl_print(in_0, va_arg!(ap, tl_object *));
                        cur = cur.offset(1);
                    }
                    _ => {
                        tl_putc!(in_0, '%');
                        tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } );
                    }
                }
            } else {
                tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } ).expect("non-null function pointer")(
                    tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } ),
                    tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } ),
                );
            }
        }
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:10"]
pub mod ctype_h {
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_7 = 2048;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_7 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_7 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_7 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_7 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_7 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_7 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_7 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_7 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_7 = 4096;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_7 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_7 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_7 = 256;
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/string.h:1"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "141:14"]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[c2rust::src_loc = "64:12"]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:1"]
pub mod stdio_h {
    #[c2rust::src_loc = "104:9"]
    pub const EOF: libc::c_int = -1;
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "144:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "145:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "230:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "350:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
        #[c2rust::src_loc = "378:12"]
        pub fn snprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ...
        ) -> libc::c_int;
        #[c2rust::src_loc = "520:1"]
        pub fn getchar() -> libc::c_int;
        #[c2rust::src_loc = "549:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[c2rust::src_loc = "556:1"]
        pub fn putchar(__c: libc::c_int) -> libc::c_int;
        #[c2rust::src_loc = "681:15"]
        pub fn fwrite(
            _: *const libc::c_void,
            _: libc::c_ulong,
            _: libc::c_ulong,
            _: *mut FILE,
        ) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:1"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "551:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "555:13"]
        pub fn free(_: *mut libc::c_void);
        #[c2rust::src_loc = "624:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
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
#[c2rust::header_src = "/home/ember/src/tinylisp/builtin.c:1"]
pub mod builtin_c {
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "513:1"]
    pub static mut init_tl_cf_all_objects: tl_init_ent =
        unsafe { TL_CFBV!(all_objects, "all-objects") };
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
            cur = tl_next_alloc!(cur);
        }
        let ref mut fresh0 = (*in_0).values;
        *fresh0 = tl_new_pair(in_0, tl_new_pair(in_0, res, (*in_0).false_), (*in_0).values);
    }
    #[no_mangle]
    #[c2rust::src_loc = "46:1"]
    pub unsafe extern "C" fn tl_cf_macro(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut fargs = tl_first!(args);
        let mut envn = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let mut body = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .next
        } else {
            NULL_0 as *mut tl_object_s
        };
        if tl_is_sym!(envn) == 0 {
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
                let ref mut fresh1 = (*in_0).error;
                *fresh1 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"bad macro envname\0" as *const u8 as *const libc::c_char,
                    ),
                    envn,
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
        let ref mut fresh3 = (*in_0).values;
        *fresh3 = tl_new_pair(
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
        let mut fargs = tl_first!(args);
        let mut body = tl_next!(args);
        let ref mut fresh4 = (*in_0).values;
        *fresh4 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_macro(in_0, fargs, 0 as *mut tl_object, body, (*in_0).env),
                (*in_0).false_,
            ),
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
        let mut key = tl_first!(args);
        let mut val = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if arity_n!(in, args, 2, "define") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad define arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh5 = (*in_0).error;
                *fresh5 = tl_new_sym(
                    in_0,
                    b"bad define arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        if tl_is_sym!(key) == 0 {
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
                let ref mut fresh6 = (*in_0).error;
                *fresh6 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"define non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
                );
            };
            let ref mut fresh7 = (*in_0).values;
            *fresh7 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        tl_eval_and_then!(in, val, key, _tl_cf_define_k)(
            tl_eval_and_then!(in, val, key, _tl_cf_define_k),
            tl_eval_and_then!(in, val, key, _tl_cf_define_k),
            tl_eval_and_then!(in, val, key, _tl_cf_define_k),
            tl_eval_and_then!(in, val, key, _tl_cf_define_k),
            b"tl_eval_and_then:_tl_cf_define_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "63:1"]
    pub unsafe extern "C" fn _tl_cf_define_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut key: *mut tl_object,
    ) {
        let ref mut fresh8 = (*in_0).env;
        *fresh8 = tl_env_set_local(in_0, (*in_0).env, key, tl_first!(result));
        let ref mut fresh9 = (*in_0).values;
        *fresh9 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "500:1"]
    pub static mut init_tl_cf_rescue: tl_init_ent = unsafe { TL_CFBV!(rescue, "rescue") };
    #[no_mangle]
    #[c2rust::src_loc = "500:1"]
    pub unsafe extern "C" fn tl_cf_rescue(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut func = 0 as *mut tl_object;
        let mut cont = 0 as *mut tl_object;
        if arity_1!(in, args, "rescue").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad rescue arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh10 = (*in_0).error;
                *fresh10 = tl_new_sym(
                    in_0,
                    b"bad rescue arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        func = tl_first!(args);
        if !(!func.is_null()
            && (*func).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint
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
                let ref mut fresh11 = (*in_0).error;
                *fresh11 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"rescue on non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    func,
                );
            };
            let ref mut fresh12 = (*in_0).values;
            *fresh12 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        cont = tl_new_cont(in_0, (*in_0).env, (*in_0).conts, (*in_0).values);
        let ref mut fresh13 = tl_rescue_push!(in_0, cont);
        *fresh13 = tl_rescue_push!(in_0, cont);
        tl_push_apply(
            in_0,
            TL_APPLY_DROP_RESCUE as libc::c_long,
            TL_EMPTY_LIST as *mut tl_object,
            TL_EMPTY_LIST as *mut tl_object,
        );
        tl_push_apply(in_0, 0 as libc::c_int as libc::c_long, func, (*in_0).env);
    }
    #[no_mangle]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn tl_cf_prefix(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut prefix = tl_first!(args);
        let mut name = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let ref mut fresh14 = (*in_0).prefixes;
        *fresh14 = tl_new_pair(in_0, tl_new_pair(in_0, prefix, name), (*in_0).prefixes);
        let ref mut fresh15 = (*in_0).values;
        *fresh15 = tl_new_pair(
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
        let mut key = tl_first!(args);
        let mut val = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if arity_n!(in, args, 2, "set") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad set arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh16 = (*in_0).error;
                *fresh16 = tl_new_sym(in_0, b"bad set arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!key.is_null() && (*key).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
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
                let ref mut fresh17 = (*in_0).error;
                *fresh17 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"set! on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    key,
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
        tl_eval_and_then!(in, val, key, _tl_cf_set_k)(
            tl_eval_and_then!(in, val, key, _tl_cf_set_k),
            tl_eval_and_then!(in, val, key, _tl_cf_set_k),
            tl_eval_and_then!(in, val, key, _tl_cf_set_k),
            tl_eval_and_then!(in, val, key, _tl_cf_set_k),
            b"tl_eval_and_then:_tl_cf_set_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "167:1"]
    pub unsafe extern "C" fn _tl_cf_set_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut key: *mut tl_object,
    ) {
        let ref mut fresh19 = (*in_0).env;
        *fresh19 = tl_env_set_global(in_0, (*in_0).env, key, tl_first!(result));
        let ref mut fresh20 = (*in_0).values;
        *fresh20 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "161:1"]
    pub unsafe extern "C" fn tl_cf_if(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cond = tl_first!(args);
        if arity_n!(in, args, 3, "if") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad if arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh21 = (*in_0).error;
                *fresh21 = tl_new_sym(in_0, b"bad if arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        tl_eval_and_then!(in, cond, tl_next(args), _tl_cf_if_k)(
            tl_eval_and_then!(in, cond, tl_next(args), _tl_cf_if_k),
            tl_eval_and_then!(in, cond, tl_next(args), _tl_cf_if_k),
            tl_eval_and_then!(in, cond, tl_next(args), _tl_cf_if_k),
            tl_eval_and_then!(in, cond, tl_next(args), _tl_cf_if_k),
            b"tl_eval_and_then:_tl_cf_if_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn _tl_cf_if_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut branches: *mut tl_object,
    ) {
        let mut ift = tl_first!(branches);
        let mut iff = if !(if !branches.is_null()
            && (branches.is_null()
                || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*branches).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !branches.is_null()
                && (branches.is_null()
                    || (*branches).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*branches).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !branches.is_null()
                    && (branches.is_null()
                        || (*branches).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*branches).data.pair.next
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
                (*branches).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if _unboolify(in_0, tl_first!(result)) != 0 {
            tl_push_eval(in_0, ift, (*in_0).env);
        } else {
            tl_push_eval(in_0, iff, (*in_0).env);
        };
    }
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
        if tl_is_int!(obj) != 0 {
            return (*obj).data.ival as libc::c_int;
        }
        if tl_is_sym!(obj) != 0 {
            return ((*(*obj).data.nm).here.len > 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "479:1"]
    pub static mut init_tl_cf_load_mod: tl_init_ent = unsafe { TL_CFBV!(load_mod, "load-mod") };
    #[no_mangle]
    #[c2rust::src_loc = "479:1"]
    pub unsafe extern "C" fn tl_cf_load_mod(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut name = tl_first!(args);
        let mut name_cstr = 0 as *mut libc::c_char;
        let mut ret = 0 as *mut tl_object;
        if tl_is_sym!(name) == 0 {
            let ref mut fresh22 = (*in_0).values;
            *fresh22 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        name_cstr = tl_alloc_malloc!(in, name -> nm -> here.len + 1);
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
        let ref mut fresh23 = (*in_0).values;
        *fresh23 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "475:1"]
    pub static mut init_tl_cf_read: tl_init_ent = unsafe { TL_CFBV!(read, "read") };
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
    #[c2rust::src_loc = "470:1"]
    pub static mut init_tl_cf_gc: tl_init_ent = unsafe { TL_CFBV!(gc, "gc") };
    #[no_mangle]
    #[c2rust::src_loc = "470:1"]
    pub unsafe extern "C" fn tl_cf_gc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_gc(in_0);
        let ref mut fresh24 = (*in_0).values;
        *fresh24 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "457:1"]
    pub static mut init_tl_cf_apply: tl_init_ent = unsafe { TL_CFBV!(apply, "apply") };
    #[no_mangle]
    #[c2rust::src_loc = "457:1"]
    pub unsafe extern "C" fn tl_cf_apply(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut cur = 0 as *mut tl_object;
        let mut len: size_t = 0;
        if arity_n!(in, args, 2, "apply") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad apply arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh25 = (*in_0).error;
                *fresh25 = tl_new_sym(
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
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        len = 0 as libc::c_int as size_t;
        while !cur.is_null() {
            let ref mut fresh26 = (*in_0).values;
            *fresh26 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !cur.is_null()
                        && (cur.is_null()
                            || (*cur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*cur).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            cur = tl_next!(cur);
            len = len.wrapping_add(1);
        }
        let ref mut fresh27 = (*in_0).values;
        *fresh27 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
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
    #[c2rust::src_loc = "440:1"]
    pub static mut init_tl_cf_type: tl_init_ent = unsafe { TL_CFBV!(type, "type") };
    #[no_mangle]
    #[c2rust::src_loc = "440:1"]
    pub unsafe extern "C" fn tl_cf_type(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut obj = tl_first!(args);
        if !tl_has_error!(in).is_null() {
            let ref mut fresh28 = (*in_0).values;
            *fresh28 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if tl_is_int!(obj) != 0 {
            let ref mut fresh29 = (*in_0).values;
            *fresh29 = tl_new_pair(
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
        if tl_is_sym!(obj) != 0 {
            let ref mut fresh30 = (*in_0).values;
            *fresh30 = tl_new_pair(
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
        if tl_is_cfunc!(obj) != 0 {
            let ref mut fresh31 = (*in_0).values;
            *fresh31 = tl_new_pair(
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
        if tl_is_cfunc_byval!(obj) != 0 {
            let ref mut fresh32 = (*in_0).values;
            *fresh32 = tl_new_pair(
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
        if tl_is_func!(obj) != 0 {
            let ref mut fresh33 = (*in_0).values;
            *fresh33 = tl_new_pair(
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
        if tl_is_macro!(obj) != 0 {
            let ref mut fresh34 = (*in_0).values;
            *fresh34 = tl_new_pair(
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
        if tl_is_cont!(obj) != 0 {
            let ref mut fresh35 = (*in_0).values;
            *fresh35 = tl_new_pair(
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
        if tl_is_pair!(obj) != 0 {
            let ref mut fresh36 = (*in_0).values;
            *fresh36 = tl_new_pair(
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
        let ref mut fresh37 = (*in_0).values;
        *fresh37 = tl_new_pair(
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
    #[c2rust::src_loc = "435:1"]
    pub static mut init_tl_cf_nand: tl_init_ent = unsafe { TL_CFBV!(nand, "nand") };
    #[no_mangle]
    #[c2rust::src_loc = "435:1"]
    pub unsafe extern "C" fn tl_cf_nand(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = _unboolify(in_0, tl_first!(args));
        let mut b = _unboolify(
            in_0,
            if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .first
            } else {
                NULL_0 as *mut tl_object_s
            },
        );
        let ref mut fresh38 = (*in_0).values;
        *fresh38 = tl_new_pair(
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
    #[c2rust::src_loc = "423:1"]
    pub static mut init_tl_cf_less: tl_init_ent = unsafe { TL_CFBV!(less, "<") };
    #[no_mangle]
    #[c2rust::src_loc = "423:1"]
    pub unsafe extern "C" fn tl_cf_less(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = tl_first!(args);
        let mut b = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if tl_is_int!(a) != 0 && tl_is_int!(b) != 0 {
            let ref mut fresh39 = (*in_0).values;
            *fresh39 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if (*a).c2rust_unnamed.ival < (*b).data.ival {
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
        if tl_is_sym!(a) != 0 && tl_is_sym!(b) != 0 {
            let ref mut fresh40 = (*in_0).values;
            *fresh40 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !a.is_null()
                        && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
                        && (!b.is_null()
                            && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && ((*(*a).c2rust_unnamed.nm).here.len < (*(*b).data.nm).here.len
                            && !((*(*b).data.nm).here.len
                                < (*(*a).c2rust_unnamed.nm).here.len)
                            || memcmp(
                                (*(*a).c2rust_unnamed.nm).here.data as *const libc::c_void,
                                (*(*b).data.nm).here.data as *const libc::c_void,
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
            let ref mut fresh41 = (*in_0).error;
            *fresh41 = tl_new_pair(
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
        let ref mut fresh42 = (*in_0).values;
        *fresh42 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "412:1"]
    pub static mut init_tl_cf_eq: tl_init_ent = unsafe { TL_CFBV!(eq, "=") };
    #[no_mangle]
    #[c2rust::src_loc = "412:1"]
    pub unsafe extern "C" fn tl_cf_eq(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut a = tl_first!(args);
        let mut b = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if tl_is_int!(a) != 0 && tl_is_int!(b) != 0 {
            let ref mut fresh43 = (*in_0).values;
            *fresh43 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if (*a).c2rust_unnamed.ival == (*b).data.ival {
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
        if tl_is_sym!(a) != 0 && tl_is_sym!(b) != 0 {
            let ref mut fresh44 = (*in_0).values;
            *fresh44 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !a.is_null()
                        && (*a).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint
                        && (!b.is_null()
                            && (*b).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
                        && (*a).c2rust_unnamed.nm == (*b).data.nm
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
        let ref mut fresh45 = (*in_0).values;
        *fresh45 = tl_new_pair(
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
    #[c2rust::src_loc = "397:1"]
    pub static mut init_tl_cf_mod: tl_init_ent = unsafe { TL_CFBV!(mod, "%") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh46 = (*in_0).error;
                    *fresh46 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"% on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh47 = (*in_0).values;
                *fresh47 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res *= (*val).data.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res %= (*val).data.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        let ref mut fresh48 = (*in_0).values;
        *fresh48 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "382:1"]
    pub static mut init_tl_cf_div: tl_init_ent = unsafe { TL_CFBV!(div, "/") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh49 = (*in_0).error;
                    *fresh49 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"/ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh50 = (*in_0).values;
                *fresh50 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res *= (*val).data.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res /= (*val).data.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        let ref mut fresh51 = (*in_0).values;
        *fresh51 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "373:1"]
    pub static mut init_tl_cf_mul: tl_init_ent = unsafe { TL_CFBV!(mul, "*") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh52 = (*in_0).error;
                    *fresh52 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"* on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh53 = (*in_0).values;
                *fresh53 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            res *= (*val).data.ival;
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        let ref mut fresh54 = (*in_0).values;
        *fresh54 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "358:1"]
    pub static mut init_tl_cf_sub: tl_init_ent = unsafe { TL_CFBV!(sub, "-") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh55 = (*in_0).error;
                    *fresh55 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"- on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh56 = (*in_0).values;
                *fresh56 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            if phase == 0 {
                res += (*val).data.ival;
                phase = 1 as libc::c_int as libc::c_long;
            } else {
                res -= (*val).data.ival;
            }
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        let ref mut fresh57 = (*in_0).values;
        *fresh57 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
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
                let ref mut fresh58 = (*in_0).error;
                *fresh58 = if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                };
            };
            let ref mut fresh59 = (*in_0).values;
            *fresh59 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        } else {
            let ref mut fresh60 = (*in_0).values;
            *fresh60 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).error, (*in_0).false_),
                (*in_0).values,
            );
            return;
        };
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "37:1"]
    pub static mut init_tl_cf_error: tl_init_ent = unsafe { TL_CFBV!(error, "error") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "46:1"]
    pub static mut init_tl_cf_macro: tl_init_ent = unsafe { TL_CF!(macro, "macro") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "57:1"]
    pub static mut init_tl_cf_lambda: tl_init_ent = unsafe { TL_CF!(lambda, "lambda") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "68:1"]
    pub static mut init_tl_cf_define: tl_init_ent = unsafe { TL_CF!(define, "define") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_arg.is_null() {
            tl_print(in_0, arg);
            if !tl_next!(l_arg).is_null() {
                tl_putc!(in_0, in -> disp_sep).expect("non-null function pointer")(
                    tl_putc!(in_0, in -> disp_sep),
                    tl_putc!(in_0, in -> disp_sep),
                );
            }
            l_arg = (if !l_arg.is_null()
                && (l_arg.is_null()
                    || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_arg).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            arg = (if !l_arg.is_null()
                && (l_arg.is_null()
                    || (*l_arg).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_arg).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        tl_printf(in_0, b"\n\0" as *const u8 as *const libc::c_char);
        let ref mut fresh61 = (*in_0).values;
        *fresh61 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "78:1"]
    pub static mut init_tl_cf_display: tl_init_ent = unsafe { TL_CFBV!(display, "display") };
    #[no_mangle]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn tl_cf_display_sep(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut arg = 0 as *mut tl_object;
        if args.is_null() {
            let ref mut fresh62 = (*in_0).values;
            *fresh62 = tl_new_pair(
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
        arg = tl_first!(args);
        if tl_is_sym!(arg) == 0 {
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
                let ref mut fresh63 = (*in_0).error;
                *fresh63 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-display-sep with non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    arg,
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
        if (*(*arg).data.nm).here.len == 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh65 = (*in_0).error;
                *fresh65 = tl_new_sym(
                    in_0,
                    b"tl-display-sep with empty sym\0" as *const u8 as *const libc::c_char,
                );
            };
            let ref mut fresh66 = (*in_0).values;
            *fresh66 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        (*in_0).disp_sep =
            *((*(*arg).data.nm).here.data).offset(0 as libc::c_int as isize);
        let ref mut fresh67 = (*in_0).values;
        *fresh67 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "87:1"]
    pub static mut init_tl_cf_display_sep: tl_init_ent =
        unsafe { TL_CFBV!(display_sep, "display-sep") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "105:1"]
    pub static mut init_tl_cf_prefix: tl_init_ent = unsafe { TL_CF!(prefix, "prefix") };
    #[no_mangle]
    #[c2rust::src_loc = "112:1"]
    pub unsafe extern "C" fn tl_cf_evalin(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_n!(in, args, 3, "eval-in&") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad eval-in& arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh68 = (*in_0).error;
                *fresh68 = tl_new_sym(
                    in_0,
                    b"bad eval-in& arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let mut env = tl_first!(args);
        let mut expr = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let mut k = if !(if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            }))
            .data
            .pair
            .next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
                .next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
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
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .data
                    .pair
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, k, (*in_0).env);
        tl_push_eval(in_0, expr, env);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "112:1"]
    pub static mut init_tl_cf_evalin: tl_init_ent = unsafe { TL_CFBV!(evalin, "eval-in&") };
    #[no_mangle]
    #[c2rust::src_loc = "125:1"]
    pub unsafe extern "C" fn tl_cf_call_with_current_continuation(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "call-with-current-continuation").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad call-with-current-continuation arity\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                let ref mut fresh69 = (*in_0).error;
                *fresh69 = tl_new_sym(
                    in_0,
                    b"bad call-with-current-continuation arity\0" as *const u8
                        as *const libc::c_char,
                );
            };
            return;
        }
        let mut cont = tl_new_cont(in_0, (*in_0).env, (*in_0).conts, (*in_0).values);
        tl_push_apply(
            in_0,
            1 as libc::c_int as libc::c_long,
            tl_first!(args),
            (*in_0).env,
        );
        let ref mut fresh70 = tl_values_push!(in, cont);
        *fresh70 = tl_values_push!(in, cont);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "125:1"]
    pub static mut init_tl_cf_call_with_current_continuation: tl_init_ent = unsafe {
        TL_CFBV!(
            call_with_current_continuation,
            "call-with-current-continuation"
        )
    };
    #[no_mangle]
    #[c2rust::src_loc = "132:1"]
    pub unsafe extern "C" fn tl_cf_cons(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_n!(in, args, 2, "cons") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad cons arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh71 = (*in_0).error;
                *fresh71 = tl_new_sym(
                    in_0,
                    b"bad cons arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let ref mut fresh72 = (*in_0).values;
        *fresh72 = tl_new_pair(
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
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .data
                        .pair
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
    pub static mut init_tl_cf_cons: tl_init_ent = unsafe { TL_CFBV!(cons, "cons") };
    #[no_mangle]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn tl_cf_car(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "car").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad car arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh73 = (*in_0).error;
                *fresh73 = tl_new_sym(in_0, b"bad car arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let ref mut fresh74 = (*in_0).values;
        *fresh74 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
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
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .data
                    .pair
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
    pub static mut init_tl_cf_car: tl_init_ent = unsafe { TL_CFBV!(car, "car") };
    #[no_mangle]
    #[c2rust::src_loc = "142:1"]
    pub unsafe extern "C" fn tl_cf_cdr(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "cdr").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad cdr arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh75 = (*in_0).error;
                *fresh75 = tl_new_sym(in_0, b"bad cdr arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let ref mut fresh76 = (*in_0).values;
        *fresh76 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
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
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .data
                    .pair
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
    #[c2rust::src_loc = "142:1"]
    pub static mut init_tl_cf_cdr: tl_init_ent = unsafe { TL_CFBV!(cdr, "cdr") };
    #[no_mangle]
    #[c2rust::src_loc = "147:1"]
    pub unsafe extern "C" fn tl_cf_null(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "null?").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad null? arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh77 = (*in_0).error;
                *fresh77 = tl_new_sym(
                    in_0,
                    b"bad null? arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        let ref mut fresh78 = (*in_0).values;
        *fresh78 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
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
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "147:1"]
    pub static mut init_tl_cf_null: tl_init_ent = unsafe { TL_CFBV!(null, "null?") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "161:1"]
    pub static mut init_tl_cf_if: tl_init_ent = unsafe { TL_CF!(if, "if") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "172:1"]
    pub static mut init_tl_cf_set: tl_init_ent = unsafe { TL_CF!(set, "set!") };
    #[no_mangle]
    #[c2rust::src_loc = "179:1"]
    pub unsafe extern "C" fn tl_cf_env(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut f = tl_first!(args);
        if f.is_null() {
            let ref mut fresh79 = (*in_0).values;
            *fresh79 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).env, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !(tl_is_macro!(f) != 0 || tl_is_func!(f) != 0) {
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
                let ref mut fresh80 = (*in_0).error;
                *fresh80 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"env of non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    f,
                );
            };
            let ref mut fresh81 = (*in_0).values;
            *fresh81 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh82 = (*in_0).values;
        *fresh82 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                (*f).data.func.env,
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "179:1"]
    pub static mut init_tl_cf_env: tl_init_ent = unsafe { TL_CFBV!(env, "env") };
    #[no_mangle]
    #[c2rust::src_loc = "191:1"]
    pub unsafe extern "C" fn tl_cf_setenv(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut first = tl_first!(args);
        let mut next = if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        if next.is_null() {
            let ref mut fresh83 = (*in_0).env;
            *fresh83 = first;
            let ref mut fresh84 = (*in_0).values;
            *fresh84 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !(tl_is_macro!(first) != 0 || tl_is_func!(first) != 0) {
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
                let ref mut fresh85 = (*in_0).error;
                *fresh85 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"setenv on non-func or -macro\0" as *const u8 as *const libc::c_char,
                    ),
                    first,
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
        let ref mut fresh87 = (*first).data.func.env;
        *fresh87 = next;
        let ref mut fresh88 = (*in_0).values;
        *fresh88 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "191:1"]
    pub static mut init_tl_cf_setenv: tl_init_ent = unsafe { TL_CFBV!(setenv, "set-env!") };
    #[no_mangle]
    #[c2rust::src_loc = "205:1"]
    pub unsafe extern "C" fn tl_cf_topenv(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let ref mut fresh89 = (*in_0).values;
        *fresh89 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).top_env, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "205:1"]
    pub static mut init_tl_cf_topenv: tl_init_ent = unsafe { TL_CFBV!(topenv, "top-env") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if tl_is_int!(val) != 0 {
                let mut buf = 0 as *mut libc::c_char;
                let mut sz_0: libc::c_int = 0;
                let mut sm = 0 as *mut tl_object;
                sz_0 = snprintf(
                    NULL_0 as *mut libc::c_char,
                    0 as libc::c_int as libc::c_ulong,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*val).data.ival,
                );
                buf = tl_alloc_malloc!(in, sz + 1);
                snprintf(
                    buf,
                    (sz_0 + 1 as libc::c_int) as libc::c_ulong,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*val).data.ival,
                );
                val = tl_new_sym(in_0, buf);
                free(buf as *mut libc::c_void);
                let ref mut fresh90 = (*l_val).data.pair.first;
                *fresh90 = val;
            }
            if tl_is_sym!(val) != 0 {
                sz = (sz as libc::c_ulong).wrapping_add((*(*val).data.nm).here.len)
                    as size_t as size_t;
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
                    let ref mut fresh91 = (*in_0).error;
                    *fresh91 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"concat on non-sym or int\0" as *const u8 as *const libc::c_char,
                        ),
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
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        rsz = sz;
        buffer = tl_alloc_malloc!(in, sz);
        end = buffer;
        if buffer.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh93 = (*in_0).error;
                *fresh93 = tl_new_sym(in_0, b"out of memory\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        let mut l_val_0 = args;
        let mut val_0 = if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val_0.is_null() {
            src = (*(*val_0).data.nm).here.data;
            sz = (*(*val_0).data.nm).here.len;
            while sz > 0 as libc::c_int as libc::c_ulong {
                let fresh94 = src;
                src = src.offset(1);
                let fresh95 = end;
                end = end.offset(1);
                *fresh95 = *fresh94;
                sz = sz.wrapping_sub(1);
            }
            l_val_0 = (if !l_val_0.is_null()
                && (l_val_0.is_null()
                    || (*l_val_0).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val_0).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val_0 = (if !l_val_0.is_null()
                && (l_val_0.is_null()
                    || (*l_val_0).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val_0).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        ret = tl_new_sym_data(in_0, buffer, rsz);
        free(buffer as *mut libc::c_void);
        let ref mut fresh96 = (*in_0).values;
        *fresh96 = tl_new_pair(in_0, tl_new_pair(in_0, ret, (*in_0).false_), (*in_0).values);
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "209:1"]
    pub static mut init_tl_cf_concat: tl_init_ent = unsafe { TL_CFBV!(concat, "concat") };
    #[no_mangle]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn tl_cf_length(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "length").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad length arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh97 = (*in_0).error;
                *fresh97 = tl_new_sym(
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
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh98 = (*in_0).error;
                *fresh98 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"length on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh99 = (*in_0).values;
            *fresh99 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh100 = (*in_0).values;
        *fresh100 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_int(
                    in_0,
                    (*(*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .data
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
    #[c2rust::src_loc = "255:1"]
    pub static mut init_tl_cf_length: tl_init_ent = unsafe { TL_CFBV!(length, "length") };
    #[no_mangle]
    #[c2rust::src_loc = "261:1"]
    pub unsafe extern "C" fn tl_cf_ord(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_n!(in, args, 2, "ord") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad ord arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh101 = (*in_0).error;
                *fresh101 =
                    tl_new_sym(in_0, b"bad ord arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh102 = (*in_0).error;
                *fresh102 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh103 = (*in_0).values;
            *fresh103 = tl_new_pair(
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
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            }))
            .data
            .pair
            .first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .data
                        .pair
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh104 = (*in_0).error;
                *fresh104 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"ord on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .data
                        .pair
                        .first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh105 = (*in_0).values;
            *fresh105 = tl_new_pair(
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
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            }))
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        }))
        .data
        .ival as libc::c_ulong
            >= (*(*tl_first!(args)).c2rust_unnamed.nm).here.len
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.next
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
                                (*args).data.pair.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .pair
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
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .nm)
                                .here
                                .len as libc::c_long,
                        ),
                    ),
                );
            } else {
                let ref mut fresh106 = (*in_0).error;
                *fresh106 = tl_new_pair(
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.next
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
                                (*args).data.pair.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .pair
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
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .nm)
                                .here
                                .len as libc::c_long,
                        ),
                    ),
                );
            };
            return;
        }
        let ref mut fresh107 = (*in_0).values;
        *fresh107 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_int(
                    in_0,
                    *((*(*if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .data
                    .nm)
                        .here
                        .data)
                        .offset(
                            (*if !(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                && ((if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.next
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .is_null()
                                    || (*(if !args.is_null()
                                        && (args.is_null()
                                            || (*args).kind as libc::c_uint
                                                == TL_PAIR as libc::c_int as libc::c_uint)
                                    {
                                        (*args).data.pair.next
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
                                    (*args).data.pair.next
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .data
                                .pair
                                .first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
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
    #[c2rust::src_loc = "261:1"]
    pub static mut init_tl_cf_ord: tl_init_ent = unsafe { TL_CFBV!(ord, "ord") };
    #[no_mangle]
    #[c2rust::src_loc = "272:1"]
    pub unsafe extern "C" fn tl_cf_chr(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut s: [libc::c_char; 2] = [0, 0];
        if arity_1!(in, args, "chr").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"bad chr arity\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh108 = (*in_0).error;
                *fresh108 =
                    tl_new_sym(in_0, b"bad chr arity\0" as *const u8 as *const libc::c_char);
            };
            return;
        }
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh109 = (*in_0).error;
                *fresh109 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"chr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh110 = (*in_0).values;
            *fresh110 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        s[0 as libc::c_int as usize] = (*tl_first!(args)).c2rust_unnamed.ival as libc::c_char;
        let ref mut fresh111 = (*in_0).values;
        *fresh111 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_sym(in_0, s.as_mut_ptr()), (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "272:1"]
    pub static mut init_tl_cf_chr: tl_init_ent = unsafe { TL_CFBV!(chr, "chr") };
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
        if arity_n!(in, args, 2, "substr") != 0 {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad substr arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh112 = (*in_0).error;
                *fresh112 = tl_new_sym(
                    in_0,
                    b"bad substr arity\0" as *const u8 as *const libc::c_char,
                );
            };
            return;
        }
        sym = tl_first!(args);
        if !(!sym.is_null() && (*sym).kind as libc::c_uint == TL_SYM as libc::c_int as libc::c_uint)
        {
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
                let ref mut fresh113 = (*in_0).error;
                *fresh113 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-sym\0" as *const u8 as *const libc::c_char,
                    ),
                    sym,
                );
            };
            let ref mut fresh114 = (*in_0).values;
            *fresh114 = tl_new_pair(
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
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
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
                let ref mut fresh115 = (*in_0).error;
                *fresh115 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"substr on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    start,
                );
            };
            let ref mut fresh116 = (*in_0).values;
            *fresh116 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        sidx = (*start).data.ival;
        if !if !(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
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
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .next
        } else {
            NULL_0 as *mut tl_object_s
        }
        .is_null()
        {
            start = if !(if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
                .next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
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
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .data
                    .pair
                    .next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        && ((if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            || (*(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.next
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
                            (*args).data.pair.next
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .data
                        .pair
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
                    (*args).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.next
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
                        (*args).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    })
                    .data
                    .pair
                    .next
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .first
            } else {
                NULL_0 as *mut tl_object_s
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
                    let ref mut fresh117 = (*in_0).error;
                    *fresh117 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"substr on non-int\0" as *const u8 as *const libc::c_char,
                        ),
                        start,
                    );
                };
                let ref mut fresh118 = (*in_0).values;
                *fresh118 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            eidx = (*start).data.ival;
        } else {
            eidx = (*(*sym).data.nm).here.len as libc::c_long;
        }
        if sidx < 0 as libc::c_int as libc::c_long {
            sidx = (sidx as libc::c_ulong).wrapping_add((*(*sym).data.nm).here.len)
                as libc::c_long as libc::c_long;
            if sidx < 0 as libc::c_int as libc::c_long {
                sidx = 0 as libc::c_int as libc::c_long;
            }
        }
        if eidx < 0 as libc::c_int as libc::c_long {
            eidx = (eidx as libc::c_ulong).wrapping_add((*(*sym).data.nm).here.len)
                as libc::c_long as libc::c_long;
            if eidx < 0 as libc::c_int as libc::c_long {
                eidx = 0 as libc::c_int as libc::c_long;
            }
        }
        if sidx as libc::c_ulong >= (*(*sym).data.nm).here.len {
            sidx = ((*(*sym).data.nm).here.len)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_long;
        }
        if eidx as libc::c_ulong > (*(*sym).data.nm).here.len {
            eidx = (*(*sym).data.nm).here.len as libc::c_long;
        }
        if sidx >= eidx {
            sidx = eidx;
        }
        let ref mut fresh119 = (*in_0).values;
        *fresh119 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_sym_data(
                    in_0,
                    ((*(*sym).data.nm).here.data).offset(sidx as isize),
                    (eidx - sidx) as size_t,
                ),
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "280:1"]
    pub static mut init_tl_cf_substr: tl_init_ent = unsafe { TL_CFBV!(substr, "substr") };
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "349:1"]
    pub static mut init_tl_cf_add: tl_init_ent = unsafe { TL_CFBV!(add, "+") };
    #[c2rust::src_loc = "326:1"]
    pub unsafe extern "C" fn _tl_readc_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let ref mut fresh120 = (*in_0).values;
        *fresh120 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                },
                (*in_0).false_,
            ),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "331:1"]
    pub unsafe extern "C" fn tl_cf_readc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_readc_k)(
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_readc_k),
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_readc_k),
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_readc_k),
            b"tl_getc_and_then:_tl_readc_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "331:1"]
    pub static mut init_tl_cf_readc: tl_init_ent = unsafe { TL_CFBV!(readc, "readc") };
    #[no_mangle]
    #[c2rust::src_loc = "335:1"]
    pub unsafe extern "C" fn tl_cf_putbackc(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "putback").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad putback arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh121 = (*in_0).error;
                *fresh121 = tl_new_sym(
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
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh122 = (*in_0).error;
                *fresh122 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"putback on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh123 = (*in_0).values;
            *fresh123 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh124 = (*in_0).values;
        *fresh124 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "335:1"]
    pub static mut init_tl_cf_putbackc: tl_init_ent = unsafe { TL_CFBV!(putbackc, "putbackc") };
    #[no_mangle]
    #[c2rust::src_loc = "342:1"]
    pub unsafe extern "C" fn tl_cf_writec(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if arity_1!(in, args, "write").is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(
                    in_0,
                    b"bad write arity\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh125 = (*in_0).error;
                *fresh125 = tl_new_sym(
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
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh126 = (*in_0).error;
                *fresh126 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"write on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh127 = (*in_0).values;
            *fresh127 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        tl_putc!(in_0, tl_first(args) -> ival).expect("non-null function pointer")(
            tl_putc!(in_0, tl_first(args) -> ival),
            tl_putc!(in_0, tl_first(args) -> ival),
        );
        let ref mut fresh128 = (*in_0).values;
        *fresh128 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "342:1"]
    pub static mut init_tl_cf_writec: tl_init_ent = unsafe { TL_CFBV!(writec, "writec") };
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
            (*args).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_val.is_null() {
            if !(!val.is_null()
                && (*val).kind as libc::c_uint == TL_INT as libc::c_int as libc::c_uint)
            {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                } else {
                    let ref mut fresh129 = (*in_0).error;
                    *fresh129 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"+ on non-int\0" as *const u8 as *const libc::c_char),
                        val,
                    );
                };
                let ref mut fresh130 = (*in_0).values;
                *fresh130 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            res += (*val).data.ival;
            l_val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            val = (if !l_val.is_null()
                && (l_val.is_null()
                    || (*l_val).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_val).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        let ref mut fresh131 = (*in_0).values;
        *fresh131 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, res), (*in_0).false_),
            (*in_0).values,
        );
    }
    use super::env_c::{tl_env_set_global, tl_env_set_local};
    use super::eval_c::{tl_push_apply, tl_push_eval};
    use super::object_c::{
        tl_gc, tl_new_cont, tl_new_int, tl_new_macro, tl_new_pair, tl_new_sym, tl_new_sym_data,
    };
    use super::print_c::{tl_print, tl_printf};
    use super::read_c::tl_read;
    use super::stddef_h::{size_t, NULL_0};
    use super::stdio_h::snprintf;
    use super::stdlib_h::free;
    use super::string_h::{memcmp, memcpy};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_object, tl_object_s, ObjectTag, TL_APPLY_DROP_RESCUE,
        TL_APPLY_INDIRECT, TL_CFUNC, TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FUNC, TL_INT,
        TL_MACRO, TL_PAIR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/debug.c:2"]
pub mod debug_c {
    #[no_mangle]
    #[c2rust::src_loc = "6:1"]
    pub unsafe extern "C" fn _indent(mut lev: libc::c_int) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < lev {
            fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
            i += 1;
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "13:1"]
    pub unsafe extern "C" fn tl_dbg_print(mut obj: *mut tl_object, mut level: libc::c_int) {
        _indent(level);
        if obj.is_null() {
            fprintf(
                stderr,
                b"() (NULL object)\n\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        match (*obj).kind as libc::c_uint {
            0 => {
                fprintf(
                    stderr,
                    b"INT: %ld\n\0" as *const u8 as *const libc::c_char,
                    (*obj).data.ival,
                );
            }
            1 => {
                fprintf(
                    stderr,
                    b"SYM: len=%lu: \0" as *const u8 as *const libc::c_char,
                    (*(*obj).data.nm).here.len,
                );
                fwrite(
                    (*(*obj).data.nm).here.data as *const libc::c_void,
                    (*(*obj).data.nm).here.len,
                    1 as libc::c_int as libc::c_ulong,
                    stderr,
                );
                fputc('\n' as i32, stderr);
            }
            2 => {
                fprintf(stderr, b"PAIR:\n\0" as *const u8 as *const libc::c_char);
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"first:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).data.pair.first,
                    level + 2 as libc::c_int,
                );
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"next:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).data.pair.next,
                    level + 2 as libc::c_int,
                );
            }
            4 | 5 | 3 => {
                fprintf(
                    stderr,
                    b"%s: %p\n\0" as *const u8 as *const libc::c_char,
                    if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint {
                        b"CFUNC\0" as *const u8 as *const libc::c_char
                    } else if (*obj).kind as libc::c_uint
                        == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                    {
                        b"CFUNC_BYVAL\0" as *const u8 as *const libc::c_char
                    } else {
                        b"THEN\0" as *const u8 as *const libc::c_char
                    },
                    (*obj).data.then.cfunc,
                );
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"state:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).data.then.state,
                    level + 2 as libc::c_int,
                );
            }
            6 | 7 => {
                fprintf(
                    stderr,
                    b"%s:\n\0" as *const u8 as *const libc::c_char,
                    if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                        b"MACRO\0" as *const u8 as *const libc::c_char
                    } else {
                        b"FUNC\0" as *const u8 as *const libc::c_char
                    },
                );
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"args:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).data.func.args,
                    level + 2 as libc::c_int,
                );
                if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                    _indent(level + 1 as libc::c_int);
                    fprintf(stderr, b"envn:\n\0" as *const u8 as *const libc::c_char);
                    tl_dbg_print(
                        (*obj).data.func.envn,
                        level + 2 as libc::c_int,
                    );
                }
                _indent(level + 1 as libc::c_int);
                fprintf(stderr, b"body:\n\0" as *const u8 as *const libc::c_char);
                tl_dbg_print(
                    (*obj).data.func.body,
                    level + 2 as libc::c_int,
                );
            }
            8 => {
                fprintf(
                    stderr,
                    b"CONTINUATION:\n\0" as *const u8 as *const libc::c_char,
                );
                _indent(level + 1 as libc::c_int);
                fprintf(
                    stderr,
                    b"ret_conts:\n\0" as *const u8 as *const libc::c_char,
                );
                tl_dbg_print(
                    (*obj).data.cont.ret_conts,
                    level + 2 as libc::c_int,
                );
                _indent(level + 1 as libc::c_int);
                fprintf(
                    stderr,
                    b"ret_values:\n\0" as *const u8 as *const libc::c_char,
                );
                tl_dbg_print(
                    (*obj).data.cont.ret_values,
                    level + 2 as libc::c_int,
                );
            }
            _ => {
                fprintf(
                    stderr,
                    b"!!! UNKNOWN OBJECT KIND %d\n\0" as *const u8 as *const libc::c_char,
                    (*obj).kind as libc::c_uint,
                );
            }
        };
    }
    #[c2rust::src_loc = "91:1"]
    pub unsafe extern "C" fn _tl_cf_debug_print_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        fprintf(stderr, b"VALUE:\n\0" as *const u8 as *const libc::c_char);
        tl_dbg_print(tl_first!(result), 0 as libc::c_int);
        let ref mut fresh132 = (*in_0).values;
        *fresh132 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn tl_cf_debug_print(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        fprintf(stderr, b"EXPR:\n\0" as *const u8 as *const libc::c_char);
        tl_dbg_print(tl_first!(args), 0 as libc::c_int);
        tl_eval_and_then!(in, tl_first(args), NULL, _tl_cf_debug_print_k)(
            tl_eval_and_then!(in, tl_first(args), NULL, _tl_cf_debug_print_k),
            tl_eval_and_then!(in, tl_first(args), NULL, _tl_cf_debug_print_k),
            tl_eval_and_then!(in, tl_first(args), NULL, _tl_cf_debug_print_k),
            tl_eval_and_then!(in, tl_first(args), NULL, _tl_cf_debug_print_k),
            b"tl_eval_and_then:_tl_cf_debug_print_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "97:1"]
    pub static mut init_tl_cf_debug_print: tl_init_ent =
        unsafe { TL_CF!(debug_print, "debug-print") };
    use super::object_c::tl_new_pair;
    use super::stdio_h::{fprintf, fputc, fwrite, stderr};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_object, ObjectTag, TL_CFUNC, TL_CFUNC_BYVAL, TL_MACRO,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/env.c:3"]
pub mod env_c {
    #[no_mangle]
    #[c2rust::src_loc = "6:1"]
    pub unsafe extern "C" fn tl_env_get_kv(
        mut in_0: *mut tl_interp,
        mut env: *mut tl_object,
        mut nm: *mut tl_object,
    ) -> *mut tl_object {
        let mut l_frame = env;
        let mut frame = if !env.is_null()
            && (env.is_null()
                || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*env).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_frame.is_null() {
            let mut l_kv = frame;
            let mut kv = if !frame.is_null()
                && (frame.is_null()
                    || (*frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*frame).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            };
            while !l_kv.is_null() {
                let mut key = tl_first!(kv);
                let mut val = tl_next!(kv);
                if !key.is_null() && tl_is_sym!(key) != 0 && tl_sym_eq!(key, nm) != 0 {
                    return kv;
                }
                l_kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).data.pair.next
                } else {
                    NULL_0 as *mut tl_object_s
                });
                kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).data.pair.first
                } else {
                    NULL_0 as *mut tl_object_s
                });
            }
            l_frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        return NULL_0 as *mut tl_object;
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
        if !kv.is_null() && tl_is_pair!(kv) != 0 {
            let ref mut fresh133 = (*kv).data.pair.next;
            *fresh133 = val;
            return env;
        }
        if env.is_null() {
            env = tl_new_pair(in_0, TL_EMPTY_LIST as *mut tl_object, env);
        }
        let mut l_frame = env;
        let mut frame = if !env.is_null()
            && (env.is_null()
                || (*env).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*env).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_frame.is_null() {
            if tl_next!(l_frame).is_null() {
                let ref mut fresh134 = (*l_frame).data.pair.first;
                *fresh134 = tl_frm_set(in_0, frame, nm, val);
            }
            l_frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
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
        let ref mut fresh135 = (*env).data.pair.first;
        *fresh135 = tl_frm_set(in_0, tl_first!(env), nm, val);
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
            && (frm.is_null()
                || (*frm).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*frm).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_kv.is_null() {
            if !kv.is_null()
                && tl_is_pair!(kv) != 0
                && (!(if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && (*(if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_SYM as libc::c_int as libc::c_uint)
                && tl_sym_eq!(tl_first(kv), nm) != 0
            {
                let ref mut fresh136 = (*kv).data.pair.next;
                *fresh136 = val;
                return frm;
            }
            l_kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
        return tl_new_pair(in_0, tl_new_pair(in_0, nm, val), frm);
    }
    use super::object_c::tl_new_pair;
    use super::stddef_h::NULL_0;
    use super::tinylisp_h::{
        tl_interp, tl_object, tl_object_s, ObjectTag, TL_EMPTY_LIST, TL_PAIR, TL_SYM,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/eval.c:4"]
pub mod eval_c {
    #[no_mangle]
    #[c2rust::src_loc = "30:1"]
    pub unsafe extern "C" fn tl_push_eval(
        mut in_0: *mut tl_interp,
        mut expr: *mut tl_object,
        mut env: *mut tl_object,
    ) -> libc::c_int {
        if !tl_has_error!(in).is_null() {
            return TL_RESULT_DONE;
        }
        if expr.is_null() {
            if !((*in_0).error).is_null() {
                tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
            } else {
                let ref mut fresh137 = (*in_0).error;
                *fresh137 = tl_new_sym(in_0, b"evaluate ()\0" as *const u8 as *const libc::c_char);
            };
            return TL_RESULT_DONE;
        }
        if tl_is_int!(expr) != 0 || tl_is_callable!(expr) != 0 {
            let ref mut fresh138 = tl_values_push!(in, expr);
            *fresh138 = tl_values_push!(in, expr);
            return TL_RESULT_DONE;
        }
        if tl_is_sym!(expr) != 0 {
            let mut binding = tl_env_get_kv(in_0, env, expr);
            if binding.is_null() {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                        expr,
                    );
                } else {
                    let ref mut fresh139 = (*in_0).error;
                    *fresh139 = tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b"unknown var\0" as *const u8 as *const libc::c_char),
                        expr,
                    );
                };
                return TL_RESULT_DONE;
            }
            let ref mut fresh140 = (*in_0).values;
            *fresh140 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    if !binding.is_null()
                        && (binding.is_null()
                            || (*binding).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*binding).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    },
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return TL_RESULT_DONE;
        }
        if tl_is_pair!(expr) != 0 {
            let mut len = tl_list_len(expr);
            let mut l_subex = expr;
            let mut subex = if !expr.is_null()
                && (expr.is_null()
                    || (*expr).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*expr).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            };
            while !l_subex.is_null() {
                if subex == tl_first!(expr) {
                    tl_push_apply(
                        in_0,
                        len as libc::c_long - 1 as libc::c_int as libc::c_long,
                        subex,
                        env,
                    );
                } else {
                    let ref mut fresh141 = tl_values_push_syntactic!(in, subex);
                    *fresh141 = tl_values_push_syntactic!(in, subex);
                }
                l_subex = (if !l_subex.is_null()
                    && (l_subex.is_null()
                        || (*l_subex).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_subex).data.pair.next
                } else {
                    NULL_0 as *mut tl_object_s
                });
                subex = (if !l_subex.is_null()
                    && (l_subex.is_null()
                        || (*l_subex).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_subex).data.pair.first
                } else {
                    NULL_0 as *mut tl_object_s
                });
            }
            return TL_RESULT_AGAIN;
        }
        if !((*in_0).error).is_null() {
            tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
                expr,
            );
        } else {
            let ref mut fresh142 = (*in_0).error;
            *fresh142 = tl_new_pair(
                in_0,
                tl_new_sym(in_0, b"unevaluable\0" as *const u8 as *const libc::c_char),
                expr,
            );
        };
        return TL_RESULT_DONE;
    }
    #[no_mangle]
    #[c2rust::src_loc = "126:1"]
    pub unsafe extern "C" fn tl_push_apply(
        mut in_0: *mut tl_interp,
        mut len: libc::c_long,
        mut expr: *mut tl_object,
        mut env: *mut tl_object,
    ) {
        let ref mut fresh143 = (*in_0).conts;
        *fresh143 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, tl_new_int(in_0, len), tl_new_pair(in_0, expr, env)),
            (*in_0).conts,
        );
        let ref mut fresh144 = (*in_0).ctr_events;
        *fresh144 = (*fresh144).wrapping_add(1);
        if (*in_0).gc_events > 0 as libc::c_int as libc::c_ulong
            && (*in_0).ctr_events >= (*in_0).gc_events
        {
            tl_gc(in_0);
            (*in_0).ctr_events = 0 as libc::c_int as size_t;
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn _tl_apply_next_body_callable_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut cont: *mut tl_object,
    ) {
        let mut callex = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
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
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let mut env = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
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
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .next
        } else {
            NULL_0 as *mut tl_object_s
        };
        let mut frm = TL_EMPTY_LIST as *mut tl_object;
        if tl_is_cfunc!(callex) != 0 || tl_is_cfunc_byval!(callex) != 0 || tl_is_then!(callex) != 0
        {
            ((*callex).data.then.cfunc).expect("non-null function pointer")(
                in_0,
                args,
                (*callex).data.then.state,
            );
            return;
        }
        if tl_is_pair!(callex -> args) != 0 {
            let mut is_improp = 0 as libc::c_int as libc::c_char;
            let mut paramlen = 0 as libc::c_int as libc::c_long;
            let mut l_item = (*callex).data.func.args;
            let mut item = if !((*callex).data.func.args).is_null()
                && (((*callex).data.func.args).is_null()
                    || (*(*callex).data.func.args).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*callex).data.func.args)
                    .data
                    .pair
                    .first
            } else {
                NULL_0 as *mut tl_object_s
            };
            while !l_item.is_null() {
                paramlen += 1;
                if !(if !l_item.is_null()
                    && (l_item.is_null()
                        || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_item).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && (*(if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).data.pair.next
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
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).data.pair.next
                    } else {
                        NULL_0 as *mut tl_object_s
                    });
                    item = (if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).data.pair.first
                    } else {
                        NULL_0 as *mut tl_object_s
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
                    let ref mut fresh145 = (*in_0).error;
                    *fresh145 = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            tl_new_sym(in_0, b"bad arity\0" as *const u8 as *const libc::c_char),
                            tl_new_int(in_0, paramlen),
                        ),
                        args,
                    );
                };
                let ref mut fresh146 = (*in_0).values;
                *fresh146 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            let mut acur = (*callex).data.func.args;
            while !acur.is_null() && !args.is_null() {
                frm = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, tl_first!(acur), tl_first!(args)),
                    frm,
                );
                args = tl_next!(args);
                if !((if !acur.is_null()
                    && (acur.is_null()
                        || (*acur).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*acur).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !acur.is_null()
                        && (acur.is_null()
                            || (*acur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*acur).data.pair.next
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    frm = tl_new_pair(in_0, tl_new_pair(in_0, tl_next!(acur), args), frm);
                    break;
                } else {
                    acur = tl_next!(acur);
                }
            }
        } else if tl_is_sym!(callex -> args) != 0 {
            frm = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*callex).data.func.args, args),
                frm,
            );
        } else {
            if !((*in_0).error).is_null() {
                tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"bad arg kind\0" as *const u8 as *const libc::c_char),
                    (*callex).data.func.args,
                );
            } else {
                let ref mut fresh147 = (*in_0).error;
                *fresh147 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"bad arg kind\0" as *const u8 as *const libc::c_char),
                    (*callex).data.func.args,
                );
            };
            let ref mut fresh148 = (*in_0).values;
            *fresh148 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        if !((*callex).data.func.envn).is_null() {
            frm = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*callex).data.func.envn, env),
                frm,
            );
        }
        env = tl_new_pair(in_0, frm, (*callex).data.func.env);
        let mut body_rvs = tl_list_rvs(in_0, (*callex).data.func.body);
        let mut l_ex = body_rvs;
        let mut ex = if !body_rvs.is_null()
            && (body_rvs.is_null()
                || (*body_rvs).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*body_rvs).data.pair.first
        } else {
            NULL_0 as *mut tl_object_s
        };
        while !l_ex.is_null() {
            tl_push_apply(
                in_0,
                (if ex == tl_first!(body_rvs) {
                    TL_APPLY_PUSH_EVAL
                } else {
                    TL_APPLY_DROP_EVAL
                }) as libc::c_long,
                ex,
                env,
            );
            l_ex = (if !l_ex.is_null()
                && (l_ex.is_null()
                    || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_ex).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            });
            ex = (if !l_ex.is_null()
                && (l_ex.is_null()
                    || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_ex).data.pair.first
            } else {
                NULL_0 as *mut tl_object_s
            });
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn tl_apply_next(mut in_0: *mut tl_interp) -> libc::c_int {
        let mut cont = tl_first!(in -> conts);
        let mut len: libc::c_long = 0;
        let mut callex = 0 as *mut tl_object;
        let mut env = 0 as *mut tl_object;
        let mut args = TL_EMPTY_LIST as *mut tl_object;
        let mut res: libc::c_int = 0;
        if !tl_has_error!(in).is_null() {
            let mut rescue = tl_rescue_peek!(in);
            if rescue.is_null() {
                return TL_RESULT_DONE;
            }
            let ref mut fresh149 = tl_rescue_drop!(in);
            *fresh149 = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            };
            tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, rescue, (*in_0).env);
            let ref mut fresh150 = tl_values_push!(in, in -> error);
            *fresh150 = tl_values_push!(in, in -> error);
            let ref mut fresh151 = tl_error_clear!(in);
            *fresh151 = NULL_0 as *mut tl_object;
            return TL_RESULT_AGAIN;
        }
        let ref mut fresh152 = (*in_0).current;
        *fresh152 = cont;
        if cont.is_null() {
            return TL_RESULT_DONE;
        }
        let ref mut fresh153 = (*in_0).conts;
        *fresh153 = tl_next!(in -> conts);
        if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.first
            } else {
                0 as *mut tl_object_s
            }))
            .kind as libc::c_uint
                == TL_INT as libc::c_int as libc::c_uint
        {
        } else {
            __assert_fail(
                b"tl_is_int(tl_first(cont))\0" as *const u8 as *const libc::c_char,
                b"./eval.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"int tl_apply_next(tl_interp *)\0",
                ))
                .as_ptr(),
            );
        }
        len = (*tl_first!(cont)).c2rust_unnamed.ival;
        callex = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
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
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        env = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
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
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .next
        } else {
            NULL_0 as *mut tl_object_s
        };
        if len == TL_APPLY_DROP as libc::c_long {
            let ref mut fresh154 = (*in_0).values;
            *fresh154 = tl_next!(in -> values);
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_DROP_RESCUE as libc::c_long {
            let ref mut fresh155 = tl_rescue_drop!(in);
            *fresh155 = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).data.pair.next
            } else {
                NULL_0 as *mut tl_object_s
            };
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_GETCHAR as libc::c_long {
            if (*in_0).is_putback != 0 {
                let ref mut fresh156 = (*in_0).values;
                *fresh156 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        tl_new_int(in_0, (*in_0).putback as libc::c_long),
                        (*in_0).false_,
                    ),
                    (*in_0).values,
                );
                (*in_0).is_putback = 0 as libc::c_int;
                return TL_RESULT_AGAIN;
            } else {
                return TL_RESULT_GETCHAR;
            }
        }
        if len != TL_APPLY_INDIRECT as libc::c_long {
            res = tl_push_eval(in_0, callex, env);
            if res != 0 {
                if !(len == TL_APPLY_PUSH_EVAL as libc::c_long
                    || len == TL_APPLY_DROP_EVAL as libc::c_long)
                {
                    cont = tl_first!(in -> conts);
                    let ref mut fresh157 = (*in_0).conts;
                    *fresh157 = tl_next!(in -> conts);
                    tl_push_apply(
                        in_0,
                        TL_APPLY_INDIRECT as libc::c_long,
                        tl_new_int(in_0, len),
                        env,
                    );
                    let ref mut fresh158 = (*in_0).conts;
                    *fresh158 = tl_new_pair(in_0, cont, (*in_0).conts);
                } else if len == TL_APPLY_DROP_EVAL as libc::c_long {
                    cont = tl_first!(in -> conts);
                    let ref mut fresh159 = (*in_0).conts;
                    *fresh159 = tl_next!(in -> conts);
                    tl_push_apply(
                        in_0,
                        TL_APPLY_DROP as libc::c_long,
                        TL_EMPTY_LIST as *mut tl_object,
                        TL_EMPTY_LIST as *mut tl_object,
                    );
                    let ref mut fresh160 = (*in_0).conts;
                    *fresh160 = tl_new_pair(in_0, cont, (*in_0).conts);
                }
                return res;
            }
        } else {
            len = (*if !(if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !cont.is_null()
                        && (cont.is_null()
                            || (*cont).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*cont).data.pair.next
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
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .first
            } else {
                NULL_0 as *mut tl_object_s
            })
            .data
            .ival;
        }
        callex = if !(if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !((*in_0).values).is_null()
                    && (((*in_0).values).is_null()
                        || (*(*in_0).values).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).values).data.pair.first
                } else {
                    0 as *mut tl_object_s
                }))
                .kind as libc::c_uint
                    == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let ref mut fresh161 = tl_values_pop_into!(in, callex);
        *fresh161 = if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).data.pair.next
        } else {
            NULL_0 as *mut tl_object_s
        };
        if len == TL_APPLY_DROP_EVAL as libc::c_long {
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_PUSH_EVAL as libc::c_long {
            let ref mut fresh162 = tl_values_push!(in, callex);
            *fresh162 = tl_values_push!(in, callex);
            return TL_RESULT_AGAIN;
        }
        if tl_is_callable!(callex) == 0 {
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
                let ref mut fresh163 = (*in_0).error;
                *fresh163 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"call non-callable\0" as *const u8 as *const libc::c_char,
                    ),
                    callex,
                );
            };
            return TL_RESULT_AGAIN;
        }
        let mut i = 0 as libc::c_int;
        while (i as libc::c_long) < len {
            args = tl_new_pair(in_0, tl_first!(in -> values), args);
            let ref mut fresh164 = (*in_0).values;
            *fresh164 = tl_next!(in -> values);
            i += 1;
        }
        let ref mut fresh165 = (*in_0).env;
        *fresh165 = env;
        let mut new_args = TL_EMPTY_LIST as *mut tl_object;
        match (*callex).kind as libc::c_uint {
            7 | 5 => {
                tl_eval_all_args!(
                    in, args, tl_new_pair(in, tl_new_int(in, len), tl_new_pair(in,
                    callex, env)), _tl_apply_next_body_callable_k
                )(
                    tl_eval_all_args!(
                        in, args, tl_new_pair(in, tl_new_int(in, len), tl_new_pair(in,
                        callex, env)), _tl_apply_next_body_callable_k
                    ),
                    tl_eval_all_args!(
                        in, args, tl_new_pair(in, tl_new_int(in, len), tl_new_pair(in,
                        callex, env)), _tl_apply_next_body_callable_k
                    ),
                    tl_eval_all_args!(
                        in, args, tl_new_pair(in, tl_new_int(in, len), tl_new_pair(in,
                        callex, env)), _tl_apply_next_body_callable_k
                    ),
                    tl_eval_all_args!(
                        in, args, tl_new_pair(in, tl_new_int(in, len), tl_new_pair(in,
                        callex, env)), _tl_apply_next_body_callable_k
                    ),
                    b"tl_eval_all_args:_tl_apply_next_body_callable_k\0" as *const u8
                        as *const libc::c_char,
                );
            }
            6 | 4 | 3 => {
                let mut l_arg = args;
                let mut arg = if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    NULL_0 as *mut tl_object_s
                };
                while !l_arg.is_null() {
                    if (*callex).kind as libc::c_uint != TL_THEN as libc::c_int as libc::c_uint
                        && tl_next!(arg) != (*in_0).true_
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
                            let ref mut fresh166 = (*in_0).error;
                            *fresh166 = tl_new_pair(
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
                        return TL_RESULT_AGAIN;
                    }
                    new_args = tl_new_pair(in_0, tl_first!(arg), new_args);
                    l_arg = (if !l_arg.is_null()
                        && (l_arg.is_null()
                            || (*l_arg).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_arg).data.pair.next
                    } else {
                        NULL_0 as *mut tl_object_s
                    });
                    arg = (if !l_arg.is_null()
                        && (l_arg.is_null()
                            || (*l_arg).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_arg).data.pair.first
                    } else {
                        NULL_0 as *mut tl_object_s
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
                        let ref mut fresh167 = (*in_0).error;
                        *fresh167 = tl_new_pair(
                            in_0,
                            tl_new_sym(
                                in_0,
                                b"bad cont arity (1)\0" as *const u8 as *const libc::c_char,
                            ),
                            args,
                        );
                    };
                    return TL_RESULT_AGAIN;
                }
                let ref mut fresh168 = (*in_0).conts;
                *fresh168 = (*callex).data.cont.ret_conts;
                let ref mut fresh169 = (*in_0).values;
                *fresh169 = (*callex).data.cont.ret_values;
                let ref mut fresh170 = (*in_0).env;
                *fresh170 = (*callex).data.cont.ret_env;
                if (if !(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
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
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .data
                    .pair
                    .next
                } else {
                    NULL_0 as *mut tl_object_s
                }) == (*in_0).true_
                {
                    tl_push_eval(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
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
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .pair
                            .first
                        } else {
                            NULL_0 as *mut tl_object_s
                        },
                        env,
                    );
                } else {
                    let ref mut fresh171 = (*in_0).values;
                    *fresh171 = tl_new_pair(
                        in_0,
                        tl_new_pair(
                            in_0,
                            if !(if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                && ((if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .is_null()
                                    || (*(if !args.is_null()
                                        && (args.is_null()
                                            || (*args).kind as libc::c_uint
                                                == TL_PAIR as libc::c_int as libc::c_uint)
                                    {
                                        (*args).data.pair.first
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
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object_s
                                })
                                .data
                                .pair
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
            _ => {}
        }
        return TL_RESULT_AGAIN;
    }
    #[no_mangle]
    #[c2rust::src_loc = "411:1"]
    pub unsafe extern "C" fn _tl_eval_and_then(
        mut in_0: *mut tl_interp,
        mut expr: *mut tl_object,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
        tl_push_eval(in_0, expr, (*in_0).env);
    }
    #[no_mangle]
    #[c2rust::src_loc = "425:1"]
    pub unsafe extern "C" fn _tl_getc_and_then(
        mut in_0: *mut tl_interp,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, tobj, (*in_0).env);
        tl_push_apply(
            in_0,
            TL_APPLY_GETCHAR as libc::c_long,
            TL_EMPTY_LIST as *mut tl_object,
            TL_EMPTY_LIST as *mut tl_object,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "432:1"]
    pub unsafe extern "C" fn _tl_eval_all_args_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut args = if !(if !(if !state.is_null()
            && (state.is_null()
                || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*state).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
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
                (*state).data.pair.first
            } else {
                0 as *mut tl_object_s
            }))
            .data
            .pair
            .first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
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
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
                .first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    && ((if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    })
                    .is_null()
                        || (*(if !state.is_null()
                            && (state.is_null()
                                || (*state).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*state).data.pair.first
                        } else {
                            0 as *mut tl_object_s
                        }))
                        .kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    }))
                    .data
                    .pair
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
                (*state).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
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
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .first
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL_0 as *mut tl_object_s
        };
        let mut stack = tl_new_pair(
            in_0,
            tl_first!(result),
            if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
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
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .data
                .pair
                .next
            } else {
                NULL_0 as *mut tl_object_s
            },
        );
        let mut then = tl_next!(state);
        let mut new_state = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                tl_new_pair(in_0, tl_next!(args), TL_EMPTY_LIST as *mut tl_object),
                stack,
            ),
            then,
        );
        if !args.is_null() {
            if (if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
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
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
                .next
            } else {
                NULL_0 as *mut tl_object_s
            }) == (*in_0).true_
            {
                tl_eval_and_then!(in, tl_first(tl_first(args)), new_state, _tl_eval_all_args_k)(
                    tl_eval_and_then!(in, tl_first(tl_first(args)), new_state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), new_state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), new_state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), new_state, _tl_eval_all_args_k),
                    b"tl_eval_and_then:_tl_eval_all_args_k\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh172 = (*in_0).values;
                *fresh172 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
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
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .pair
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
            let mut l_elem = tl_list_rvs(in_0, stack);
            let mut elem = if !(tl_list_rvs(in_0, stack)).is_null()
                && ((tl_list_rvs(in_0, stack)).is_null()
                    || (*tl_list_rvs(in_0, stack)).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*tl_list_rvs(in_0, stack))
                    .data
                    .pair
                    .first
            } else {
                NULL_0 as *mut tl_object_s
            };
            while !l_elem.is_null() {
                let ref mut fresh173 = tl_values_push!(in, elem);
                *fresh173 = tl_values_push!(in, elem);
                l_elem = (if !l_elem.is_null()
                    && (l_elem.is_null()
                        || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_elem).data.pair.next
                } else {
                    NULL_0 as *mut tl_object_s
                });
                elem = (if !l_elem.is_null()
                    && (l_elem.is_null()
                        || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_elem).data.pair.first
                } else {
                    NULL_0 as *mut tl_object_s
                });
            }
            tl_push_apply(in_0, tl_list_len(stack) as libc::c_long, then, (*in_0).env);
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "468:1"]
    pub unsafe extern "C" fn _tl_eval_all_args(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
        mut then: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut name: *const libc::c_char,
    ) {
        let mut tobj = tl_new_then(in_0, then, state, name);
        if !args.is_null() {
            let mut state_0 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, tl_next!(args), TL_EMPTY_LIST as *mut tl_object),
                    TL_EMPTY_LIST as *mut tl_object,
                ),
                tobj,
            );
            if (if !(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
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
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object_s
                }))
                .data
                .pair
                .next
            } else {
                NULL_0 as *mut tl_object_s
            }) == (*in_0).true_
            {
                tl_eval_and_then!(in, tl_first(tl_first(args)), state, _tl_eval_all_args_k)(
                    tl_eval_and_then!(in, tl_first(tl_first(args)), state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), state, _tl_eval_all_args_k),
                    tl_eval_and_then!(in, tl_first(tl_first(args)), state, _tl_eval_all_args_k),
                    b"tl_eval_and_then:_tl_eval_all_args_k\0" as *const u8 as *const libc::c_char,
                );
            } else {
                let ref mut fresh174 = (*in_0).values;
                *fresh174 = tl_new_pair(
                    in_0,
                    tl_new_pair(
                        in_0,
                        if !(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
                        } else {
                            0 as *mut tl_object_s
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
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
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object_s
                            })
                            .data
                            .pair
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
    #[c2rust::src_loc = "495:1"]
    pub unsafe extern "C" fn tl_run_until_done(mut in_0: *mut tl_interp) {
        let mut res: libc::c_int = 0;
        loop {
            res = tl_apply_next(in_0);
            if !(res != 0) {
                break;
            }
            match res {
                TL_RESULT_GETCHAR => {
                    let ref mut fresh175 = (*in_0).values;
                    *fresh175 = tl_new_pair(
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
                TL_RESULT_AGAIN | _ => {}
            }
        }
    }
    use super::assert_h::__assert_fail;
    use super::env_c::tl_env_get_kv;
    use super::object_c::{
        tl_gc, tl_list_len, tl_list_rvs, tl_new_int, tl_new_pair, tl_new_sym, tl_new_then,
    };
    use super::stddef_h::{size_t, NULL_0};
    use super::tinylisp_h::{
        tl_interp, tl_object, tl_object_s, ObjectTag, TL_APPLY_DROP, TL_APPLY_DROP_EVAL,
        TL_APPLY_DROP_RESCUE, TL_APPLY_GETCHAR, TL_APPLY_INDIRECT, TL_APPLY_PUSH_EVAL,
        TL_EMPTY_LIST, TL_INT, TL_PAIR, TL_RESULT_AGAIN, TL_RESULT_DONE, TL_RESULT_GETCHAR, TL_SYM,
        TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/interp.c:5"]
pub mod interp_c {
    #[c2rust::src_loc = "9:1"]
    pub unsafe extern "C" fn _readf(mut in_0: *mut tl_interp) -> libc::c_int {
        return getchar();
    }
    #[c2rust::src_loc = "10:1"]
    pub unsafe extern "C" fn _writef(mut in_0: *mut tl_interp, c: libc::c_char) {
        putchar(c as libc::c_int);
    }
    #[c2rust::src_loc = "11:1"]
    pub unsafe extern "C" fn _modloadf(
        mut in_0: *mut tl_interp,
        mut fn_0: *const libc::c_char,
    ) -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[c2rust::src_loc = "12:1"]
    pub unsafe extern "C" fn _reallocf(
        mut in_0: *mut tl_interp,
        mut ptr: *mut libc::c_void,
        mut s: size_t,
    ) -> *mut libc::c_void {
        if s == 0 {
            free(ptr);
            return NULL_0 as *mut libc::c_void;
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
        let ref mut fresh176 = (*in_0).reallocf;
        *fresh176 = reallocf;
        let ref mut fresh177 = (*in_0).readf;
        *fresh177 = Some(_readf as unsafe extern "C" fn(*mut tl_interp) -> libc::c_int);
        let ref mut fresh178 = (*in_0).writef;
        *fresh178 = Some(_writef as unsafe extern "C" fn(*mut tl_interp, libc::c_char) -> ());
        let ref mut fresh179 = (*in_0).modloadf;
        *fresh179 = Some(
            _modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
        );
        tl_ns_init(in_0, &mut (*in_0).ns);
        let ref mut fresh180 = (*in_0).top_alloc;
        *fresh180 = NULL_0 as *mut tl_object;
        let ref mut fresh181 = (*in_0).true_;
        *fresh181 = tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char);
        let ref mut fresh182 = (*in_0).false_;
        *fresh182 = tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char);
        let ref mut fresh183 = (*in_0).error;
        *fresh183 = NULL_0 as *mut tl_object;
        let ref mut fresh184 = (*in_0).prefixes;
        *fresh184 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh185 = (*in_0).current;
        *fresh185 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh186 = (*in_0).conts;
        *fresh186 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh187 = (*in_0).values;
        *fresh187 = TL_EMPTY_LIST as *mut tl_object;
        let ref mut fresh188 = (*in_0).rescue;
        *fresh188 = TL_EMPTY_LIST as *mut tl_object;
        (*in_0).gc_events = TL_DEFAULT_GC_EVENTS as size_t;
        (*in_0).ctr_events = 0 as libc::c_int as size_t;
        (*in_0).putback = 0 as libc::c_int;
        (*in_0).is_putback = 0 as libc::c_int;
        let ref mut fresh189 = (*in_0).read_buffer;
        *fresh189 = NULL_0 as *mut libc::c_char;
        (*in_0).disp_sep = '\t' as i32 as libc::c_char;
        let ref mut fresh190 = (*in_0).top_env;
        *fresh190 = TL_EMPTY_LIST as *mut tl_object;
        let mut top_frm = TL_EMPTY_LIST as *mut tl_object;
        let mut current: *mut tl_init_ent = &mut __start_tl_init_ents;
        top_frm = _tl_frm_set!("tl-#t", in -> true_, top_frm);
        top_frm = _tl_frm_set!("tl-#f", in -> false_, top_frm);
        while current != &mut __stop_tl_init_ents as *mut tl_init_ent {
            top_frm = _tl_frm_set!(
                current -> name, current -> flags & TL_EF_BYVAL ? _tl_new_cfunc_byval(in,
                current -> fn, current -> name) : _tl_new_cfunc(in, current -> fn,
                current -> name), top_frm
            );
            current = current.offset(1);
        }
        let ref mut fresh191 = (*in_0).top_env;
        *fresh191 = tl_new_pair(in_0, top_frm, (*in_0).top_env);
        let ref mut fresh192 = (*in_0).env;
        *fresh192 = (*in_0).top_env;
    }
    #[no_mangle]
    #[c2rust::src_loc = "110:1"]
    pub unsafe extern "C" fn tl_interp_cleanup(mut in_0: *mut tl_interp) {
        while !((*in_0).top_alloc).is_null() {
            tl_free(in_0, (*in_0).top_alloc);
        }
        tl_ns_free(in_0, &mut (*in_0).ns);
    }
    use super::ns_c::{tl_ns_free, tl_ns_init};
    use super::object_c::{tl_free, tl_new_pair, tl_new_sym};
    use super::stddef_h::{size_t, NULL_0};
    use super::stdio_h::{getchar, putchar};
    use super::stdlib_h::{free, realloc};
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_interp_s, tl_ns, tl_object, TL_DEFAULT_GC_EVENTS, TL_EMPTY_LIST,
    };
    extern "C" {
        #[c2rust::src_loc = "3:20"]
        pub static mut __start_tl_init_ents: tl_init_ent;
        #[c2rust::src_loc = "3:42"]
        pub static mut __stop_tl_init_ents: tl_init_ent;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:6"]
pub mod unistd_h {
    #[c2rust::src_loc = "210:9"]
    pub const STDIN_FILENO: libc::c_int = 0 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "809:1"]
        pub fn isatty(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/ember/src/tinylisp/main.c:6"]
pub mod main_c {
    #[no_mangle]
    #[c2rust::src_loc = "53:5"]
    pub static mut running: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "49:9"]
    pub const QUIET_NO_VALUE: libc::c_int = 3 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const QUIET_NO_TRUE: libc::c_int = 2 as libc::c_int;
    #[c2rust::src_loc = "46:9"]
    pub const QUIET_OFF: libc::c_int = 0 as libc::c_int;
    #[no_mangle]
    #[c2rust::src_loc = "50:5"]
    pub static mut quiet: libc::c_int = QUIET_OFF;
    #[no_mangle]
    #[c2rust::src_loc = "31:1"]
    pub unsafe extern "C" fn my_modloadf(
        mut in_0: *mut tl_interp,
        mut fname: *const libc::c_char,
    ) -> libc::c_int {
        let mut hdl = dlopen(fname, RTLD_NOW | RTLD_GLOBAL);
        if hdl.is_null() {
            tl_printf(
                in_0,
                b"Module load error: %s\n\0" as *const u8 as *const libc::c_char,
                dlerror(),
            );
            return 0 as libc::c_int;
        }
        let mut ini = dlsym(hdl, b"tl_init\0" as *const u8 as *const libc::c_char);
        if ini.is_null() {
            tl_printf(
                in_0,
                b"Module init error: %s\n\0" as *const u8 as *const libc::c_char,
                dlerror(),
            );
            return 0 as libc::c_int;
        }
        return (*(&mut ini as *mut *mut libc::c_void
            as *mut Option<
                unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
            >))
            .expect("non-null function pointer")(in_0, fname);
    }
    #[no_mangle]
    #[c2rust::src_loc = "27:12"]
    pub static mut _global_in: *mut tl_interp = 0 as *const tl_interp as *mut tl_interp;
    #[no_mangle]
    #[c2rust::src_loc = "55:1"]
    pub unsafe extern "C" fn _main_k(
        mut in_0: *mut tl_interp,
        mut result: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if tl_prompt!("Value: ") == QUIET_OFF {
            tl_prompt!("Value: ")(stderr, b"Value: \0" as *const u8 as *const libc::c_char);
        }
        if quiet != QUIET_NO_VALUE && (quiet != QUIET_NO_TRUE || tl_first!(result) != (*in_0).true_)
        {
            tl_print(in_0, tl_first!(result));
            tl_printf(in_0, b"\n\0" as *const u8 as *const libc::c_char);
        }
        fflush(stdout);
        if !((*in_0).values).is_null() {
            if tl_prompt!("(Rest of stack: ") == QUIET_OFF {
                tl_prompt!("(Rest of stack: ")(
                    stderr,
                    b"(Rest of stack: \0" as *const u8 as *const libc::c_char,
                );
            }
            tl_print(in_0, (*in_0).values);
            fflush(stdout);
            if tl_prompt!(")\n") == QUIET_OFF {
                tl_prompt!(")\n")(stderr, b")\n\0" as *const u8 as *const libc::c_char);
            }
        }
        let ref mut fresh193 = (*in_0).values;
        *fresh193 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[no_mangle]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn _main_read_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        let mut expr = tl_first!(args);
        if expr.is_null() {
            if tl_prompt!("Done.\n") == QUIET_OFF {
                tl_prompt!("Done.\n")(stderr, b"Done.\n\0" as *const u8 as *const libc::c_char);
            }
            tl_interp_cleanup(in_0);
            running = 0 as libc::c_int;
            return;
        }
        if quiet == QUIET_OFF {
            if tl_prompt!("Read: ") == QUIET_OFF {
                tl_prompt!("Read: ")(stderr, b"Read: \0" as *const u8 as *const libc::c_char);
            }
            tl_print(in_0, expr);
            fflush(stdout);
            if tl_prompt!("\n") == QUIET_OFF {
                tl_prompt!("\n")(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        let ref mut fresh194 = (*in_0).current;
        *fresh194 = TL_EMPTY_LIST as *mut tl_object;
        tl_eval_and_then!(in, expr, NULL, _main_k)(
            tl_eval_and_then!(in, expr, NULL, _main_k),
            tl_eval_and_then!(in, expr, NULL, _main_k),
            tl_eval_and_then!(in, expr, NULL, _main_k),
            tl_eval_and_then!(in, expr, NULL, _main_k),
            b"tl_eval_and_then:_main_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "89:1"]
    pub static mut init_tl_cf_quiet: tl_init_ent = unsafe { TL_CFBV!(quiet, "quiet") };
    #[no_mangle]
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn tl_cf_quiet(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if !args.is_null() {
            let mut arg = tl_first!(args);
            if tl_is_int!(arg) != 0 {
                quiet = (*arg).c2rust_unnamed.ival as libc::c_int;
                let ref mut fresh195 = (*in_0).values;
                *fresh195 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            } else {
                if !((*in_0).error).is_null() {
                    tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"tl-quiet on non-int\0" as *const u8 as *const libc::c_char,
                        ),
                        arg,
                    );
                } else {
                    let ref mut fresh196 = (*in_0).error;
                    *fresh196 = tl_new_pair(
                        in_0,
                        tl_new_sym(
                            in_0,
                            b"tl-quiet on non-int\0" as *const u8 as *const libc::c_char,
                        ),
                        arg,
                    );
                };
            }
        } else {
            let ref mut fresh197 = (*in_0).values;
            *fresh197 = tl_new_pair(
                in_0,
                tl_new_pair(
                    in_0,
                    tl_new_int(in_0, quiet as libc::c_long),
                    (*in_0).false_,
                ),
                (*in_0).values,
            );
            return;
        };
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "103:1"]
    pub static mut init_tl_cf_exit: tl_init_ent = unsafe { TL_CFBV!(exit, "exit") };
    #[no_mangle]
    #[c2rust::src_loc = "103:1"]
    pub unsafe extern "C" fn tl_cf_exit(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        if args.is_null()
            || !(!(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                && (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
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
                        b"tl-exit on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    args,
                );
            } else {
                let ref mut fresh198 = (*in_0).error;
                *fresh198 = tl_new_pair(
                    in_0,
                    tl_new_sym(
                        in_0,
                        b"tl-exit on non-int\0" as *const u8 as *const libc::c_char,
                    ),
                    args,
                );
            };
            let ref mut fresh199 = (*in_0).values;
            *fresh199 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        exit((*tl_first!(args)).c2rust_unnamed.ival as libc::c_int);
    }
    #[no_mangle]
    #[c2rust::src_loc = "113:1"]
    pub unsafe extern "C" fn _print_cont(
        mut in_0: *mut tl_interp,
        mut cont: *mut tl_object,
        mut level: libc::c_int,
    ) {
        let mut len = 0 as *mut tl_object;
        let mut callex = 0 as *mut tl_object;
        fprintf(stderr, b"Len \0" as *const u8 as *const libc::c_char);
        len = tl_first!(cont);
        tl_print(in_0, len);
        fflush(stdout);
        if tl_is_int!(len) != 0 && (*len).data.ival < 0 as libc::c_int as libc::c_long {
            match (*len).data.ival {
                -1 => {
                    fprintf(
                        stderr,
                        b" (TL_APPLY_PUSH_EVAL)\0" as *const u8 as *const libc::c_char,
                    );
                }
                -2 => {
                    fprintf(
                        stderr,
                        b" (TL_APPLY_INDIRECT)\0" as *const u8 as *const libc::c_char,
                    );
                }
                -3 => {
                    fprintf(
                        stderr,
                        b" (TL_APPLY_DROP_EVAL)\0" as *const u8 as *const libc::c_char,
                    );
                }
                -4 => {
                    fprintf(
                        stderr,
                        b" (TL_APPLY_DROP)\0" as *const u8 as *const libc::c_char,
                    );
                }
                -5 => {
                    fprintf(
                        stderr,
                        b" (TL_APPLY_DROP_RESCUE)\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {}
            }
        }
        fprintf(stderr, b" Callex \0" as *const u8 as *const libc::c_char);
        callex = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
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
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object_s
            })
            .data
            .pair
            .first
        } else {
            NULL as *mut tl_object_s
        };
        tl_print(in_0, callex);
        fflush(stdout);
        if tl_is_then!(callex) != 0 && !((*callex).data.then.state).is_null()
        {
            fprintf(
                stderr,
                b" Returns to \0" as *const u8 as *const libc::c_char,
            );
            _print_cont(
                in_0,
                (*callex).data.then.state,
                level + 1 as libc::c_int,
            );
        }
        if tl_is_cont!(callex) != 0 && tl_is_marked!(callex) == 0 {
            let ref mut fresh200 = tl_mark!(callex);
            *fresh200 |= TL_F_MARK as libc::c_ulong;
            fprintf(stderr, b":\0" as *const u8 as *const libc::c_char);
            _print_cont_stack(
                in_0,
                (*callex).data.cont.ret_conts,
                level + 1 as libc::c_int,
            );
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "144:1"]
    pub unsafe extern "C" fn _print_cont_stack(
        mut in_0: *mut tl_interp,
        mut stack: *mut tl_object,
        mut level: libc::c_int,
    ) {
        let mut i: libc::c_int = 0;
        let mut l_cont = (*in_0).conts;
        let mut cont = if !((*in_0).conts).is_null()
            && (((*in_0).conts).is_null()
                || (*(*in_0).conts).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).conts).data.pair.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_cont.is_null() {
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int;
            while i < level {
                fprintf(stderr, b"  \0" as *const u8 as *const libc::c_char);
                i += 1;
            }
            fprintf(stderr, b"Stack\0" as *const u8 as *const libc::c_char);
            if l_cont == (*in_0).conts {
                fprintf(stderr, b"(Top)\0" as *const u8 as *const libc::c_char);
            }
            if tl_next!(l_cont).is_null() {
                fprintf(stderr, b"(Bottom)\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b": \0" as *const u8 as *const libc::c_char);
            _print_cont(in_0, cont, level);
            l_cont = (if !l_cont.is_null()
                && (l_cont.is_null()
                    || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_cont).data.pair.next
            } else {
                NULL as *mut tl_object_s
            });
            cont = (if !l_cont.is_null()
                && (l_cont.is_null()
                    || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_cont).data.pair.first
            } else {
                NULL as *mut tl_object_s
            });
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "161:1"]
    pub unsafe extern "C" fn print_cont_stack(mut in_0: *mut tl_interp, mut stack: *mut tl_object) {
        let mut obj = (*in_0).top_alloc;
        while !obj.is_null() {
            let ref mut fresh201 = tl_unmark!(obj);
            *fresh201 &= !TL_FMASK as libc::c_ulong;
            obj = tl_next_alloc!(obj);
        }
        fprintf(stderr, b"\nCurrent: \0" as *const u8 as *const libc::c_char);
        _print_cont(in_0, (*in_0).current, 0 as libc::c_int);
        _print_cont_stack(in_0, stack, 0 as libc::c_int);
    }
    #[c2rust::src_loc = "183:1"]
    unsafe fn main_0() -> libc::c_int {
        let mut in_0 = tl_interp {
            ns: tl_ns {
                root: 0 as *mut tl_name,
            },
            top_env: 0 as *mut tl_object,
            env: 0 as *mut tl_object,
            true_: 0 as *mut tl_object,
            false_: 0 as *mut tl_object,
            error: 0 as *mut tl_object,
            prefixes: 0 as *mut tl_object,
            top_alloc: 0 as *mut tl_object,
            current: 0 as *mut tl_object,
            conts: 0 as *mut tl_object,
            values: 0 as *mut tl_object,
            rescue: 0 as *mut tl_object,
            gc_events: 0,
            ctr_events: 0,
            putback: 0,
            is_putback: 0,
            read_buffer: 0 as *mut libc::c_char,
            read_ptr: 0,
            read_sz: 0,
            disp_sep: 0,
            udata: 0 as *mut libc::c_void,
            readf: None,
            writef: None,
            reallocf: None,
            modloadf: None,
        };
        let mut expr = 0 as *mut tl_object;
        let mut val = 0 as *mut tl_object;
        _global_in = &mut in_0;
        if isatty(STDIN_FILENO) == 0 {
            quiet = QUIET_NO_TRUE;
        }
        tl_interp_init(&mut in_0);
        in_0.modloadf = Some(
            my_modloadf as unsafe extern "C" fn(*mut tl_interp, *const libc::c_char) -> libc::c_int,
        );
        if quiet == QUIET_OFF {
            if tl_prompt!("Top Env: ") == QUIET_OFF {
                tl_prompt!("Top Env: ")(stderr, b"Top Env: \0" as *const u8 as *const libc::c_char);
            }
            tl_print(&mut in_0, in_0.top_env);
            fflush(stdout);
            if tl_prompt!("\n") == QUIET_OFF {
                tl_prompt!("\n")(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        while running != 0 {
            if tl_prompt!("> ") == QUIET_OFF {
                tl_prompt!("> ")(stderr, b"> \0" as *const u8 as *const libc::c_char);
            }
            tl_read_and_then!(
                & in, _main_read_k, TL_EMPTY_LIST
            )(
                tl_read_and_then!(& in, _main_read_k, TL_EMPTY_LIST),
                tl_read_and_then!(& in, _main_read_k, TL_EMPTY_LIST),
                tl_read_and_then!(& in, _main_read_k, TL_EMPTY_LIST),
                tl_read_and_then!(& in, _main_read_k, TL_EMPTY_LIST),
            );
            tl_read_and_then!(& in, _main_read_k, TL_EMPTY_LIST)(&mut in_0);
            tl_run_until_done(&mut in_0);
            if running == 0 {
                break;
            }
            if !(in_0.error).is_null() {
                fprintf(stderr, b"Error: \0" as *const u8 as *const libc::c_char);
                tl_print(&mut in_0, in_0.error);
                fflush(stdout);
                print_cont_stack(&mut in_0, in_0.conts);
                fprintf(stderr, b"\nValues: \0" as *const u8 as *const libc::c_char);
                tl_print(&mut in_0, in_0.values);
                fflush(stdout);
                let mut l_frm = in_0.env;
                let mut frm = if !(in_0.env).is_null()
                    && ((in_0.env).is_null()
                        || (*in_0.env).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*in_0.env).data.pair.first
                } else {
                    NULL as *mut tl_object_s
                };
                while !l_frm.is_null() {
                    fprintf(stderr, b"\nFrame\0" as *const u8 as *const libc::c_char);
                    if tl_next!(l_frm).is_null() {
                        fprintf(stderr, b"(Outer)\0" as *const u8 as *const libc::c_char);
                    }
                    if l_frm == in_0.env {
                        fprintf(stderr, b"(Inner)\0" as *const u8 as *const libc::c_char);
                    }
                    fprintf(stderr, b": \0" as *const u8 as *const libc::c_char);
                    tl_print(&mut in_0, frm);
                    fflush(stdout);
                    l_frm = (if !l_frm.is_null()
                        && (l_frm.is_null()
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).data.pair.next
                    } else {
                        NULL as *mut tl_object_s
                    });
                    frm = (if !l_frm.is_null()
                        && (l_frm.is_null()
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).data.pair.first
                    } else {
                        NULL as *mut tl_object_s
                    });
                }
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                let ref mut fresh202 = tl_error_clear!(& in);
                *fresh202 = NULL as *mut tl_object;
            }
            in_0.conts = TL_EMPTY_LIST as *mut tl_object;
            in_0.values = TL_EMPTY_LIST as *mut tl_object;
            tl_gc(&mut in_0);
        }
        return 0;
    }
    use super::bits_dlfcn_h::{RTLD_GLOBAL, RTLD_NOW};
    use super::dlfcn_h::{dlerror, dlopen, dlsym};
    use super::eval_c::tl_run_until_done;
    use super::interp_c::{tl_interp_cleanup, tl_interp_init};
    use super::object_c::{tl_gc, tl_new_int, tl_new_pair, tl_new_sym};
    use super::print_c::{tl_print, tl_printf};
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::{fflush, fprintf, stderr, stdout};
    use super::stdlib_h::exit;
    use super::tinylisp_h::{
        tl_init_ent, tl_interp, tl_interp_s, tl_name, tl_ns, tl_object, tl_object_s,
        ObjectTag, TL_EMPTY_LIST, TL_FMASK, TL_F_MARK, TL_INT, TL_PAIR,
    };
    use super::unistd_h::{isatty, STDIN_FILENO};
}
#[c2rust::header_src = "/usr/include/x86_64-linux-gnu/bits/dlfcn.h:6"]
pub mod bits_dlfcn_h {
    #[c2rust::src_loc = "25:9"]
    pub const RTLD_NOW: libc::c_int = 0x2 as libc::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const RTLD_GLOBAL: libc::c_int = 0x100 as libc::c_int;
}
#[c2rust::header_src = "/usr/include/dlfcn.h:6"]
pub mod dlfcn_h {
    extern "C" {
        #[c2rust::src_loc = "84:1"]
        pub fn dlerror() -> *mut libc::c_char;
        #[c2rust::src_loc = "66:1"]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
            -> *mut libc::c_void;
        #[c2rust::src_loc = "58:1"]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/ember/src/tinylisp/ns.c:7"]
pub mod ns_c {
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
        ret.len = end.wrapping_sub(start);
        ret.data = tl_alloc_malloc!(in, ret.len);
        memcpy(
            ret.data as *mut libc::c_void,
            (orig.data).offset(start as isize) as *const libc::c_void,
            ret.len,
        );
        return ret;
    }
    #[c2rust::src_loc = "20:1"]
    pub unsafe extern "C" fn tl_ns_split(
        mut in_0: *mut tl_interp,
        mut child: *mut tl_child,
        mut len: size_t,
    ) -> *mut tl_name {
        let mut new_name = 0 as *mut tl_name;
        new_name = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name;
        let ref mut fresh203 = (*new_name).sz_children;
        *fresh203 = 1 as libc::c_int as size_t;
        (*new_name).num_children = *fresh203;
        let ref mut fresh204 = (*new_name).children;
        *fresh204 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_child>() as libc::c_ulong,
        ) as *mut tl_child;
        let ref mut fresh205 = (*(*new_name).children).name;
        *fresh205 = (*child).name;
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
        let ref mut fresh206 = (*child).seg.data;
        *fresh206 = tl_alloc_realloc!(in, child -> seg.data, len);
        let ref mut fresh207 = (*child).name;
        *fresh207 = new_name;
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
        'c_2714: loop {
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
                    tl_min!(name.len, children[index].seg.len),
                );
                if sign == 0 {
                    let mut match_len = tl_min!(name.len, children[index].seg.len);
                    if name.len < (*children.offset(index as isize)).seg.len {
                        cur = tl_ns_split(in_0, children.offset(index as isize), name.len);
                    } else {
                        cur = (*children.offset(index as isize)).name;
                    }
                    name.data = (name.data).offset(match_len as isize);
                    name.len =
                        (name.len as libc::c_ulong).wrapping_sub(match_len) as size_t as size_t;
                    continue 'c_2714;
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
                    name.len =
                        (name.len as libc::c_ulong).wrapping_sub(matching) as size_t as size_t;
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
            let ref mut fresh208 = (*cur).children;
            *fresh208 = tl_alloc_realloc!(
                in, cur -> children, sizeof(tl_child) * cur -> sz_children
            );
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
        let ref mut fresh209 = (*cur).num_children;
        *fresh209 = (*fresh209).wrapping_add(1);
        (*((*cur).children).offset(low as isize)).seg =
            tl_buf_slice(in_0, name, 0 as libc::c_int as size_t, name.len);
        let ref mut fresh210 = (*((*cur).children).offset(low as isize)).name;
        *fresh210 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name_s;
        cur = (*((*cur).children).offset(low as isize)).name;
        (*cur).here = tl_buf_slice(in_0, whole_name, 0 as libc::c_int as size_t, whole_name.len);
        let ref mut fresh211 = (*cur).sz_children;
        *fresh211 = 0 as libc::c_int as size_t;
        (*cur).num_children = *fresh211;
        let ref mut fresh212 = (*cur).children;
        *fresh212 = NULL as *mut tl_child;
        return cur;
    }
    #[no_mangle]
    #[c2rust::src_loc = "244:1"]
    pub unsafe extern "C" fn tl_ns_init(mut in_0: *mut tl_interp, mut ns: *mut tl_ns) {
        let ref mut fresh213 = (*ns).root;
        *fresh213 = ((*in_0).reallocf).expect("non-null function pointer")(
            in_0,
            0 as *mut libc::c_void,
            ::std::mem::size_of::<tl_name>() as libc::c_ulong,
        ) as *mut tl_name;
        let ref mut fresh214 = (*(*ns).root).here.data;
        *fresh214 = NULL as *mut libc::c_char;
        (*(*ns).root).here.len = 0 as libc::c_int as size_t;
        let ref mut fresh215 = (*(*ns).root).sz_children;
        *fresh215 = 0 as libc::c_int as size_t;
        (*(*ns).root).num_children = *fresh215;
        let ref mut fresh216 = (*(*ns).root).children;
        *fresh216 = NULL as *mut tl_child;
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
        let ref mut fresh217 = (*cur).chain;
        *fresh217 = NULL as *mut tl_name_s;
        while !cur.is_null() {
            index = 0 as libc::c_int as size_t;
            while index < (*cur).num_children {
                child = &mut *((*cur).children).offset(index as isize) as *mut tl_child;
                tl_alloc_free!(in, child -> seg.data).expect("non-null function pointer")(
                    tl_alloc_free!(in, child -> seg.data),
                    tl_alloc_free!(in, child -> seg.data),
                    tl_alloc_free!(in, child -> seg.data),
                );
                let ref mut fresh218 = (*(*child).name).chain;
                *fresh218 = (*cur).chain;
                let ref mut fresh219 = (*cur).chain;
                *fresh219 = (*child).name;
                index = index.wrapping_add(1);
            }
            tl_alloc_free!(in, cur -> children).expect("non-null function pointer")(
                tl_alloc_free!(in, cur -> children),
                tl_alloc_free!(in, cur -> children),
                tl_alloc_free!(in, cur -> children),
            );
            tl_alloc_free!(in, cur -> here.data).expect("non-null function pointer")(
                tl_alloc_free!(in, cur -> here.data),
                tl_alloc_free!(in, cur -> here.data),
                tl_alloc_free!(in, cur -> here.data),
            );
            temp = cur;
            cur = (*cur).chain;
            tl_alloc_free!(in, temp).expect("non-null function pointer")(
                tl_alloc_free!(in, temp),
                tl_alloc_free!(in, temp),
                tl_alloc_free!(in, temp),
            );
        }
    }
    #[c2rust::src_loc = "278:1"]
    pub unsafe extern "C" fn tl_ns_print_name(
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
    pub unsafe extern "C" fn tl_ns_print_child(
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
        let ref mut fresh220 = (*cur).chain;
        *fresh220 = NULL as *mut tl_name_s;
        while !cur.is_null() {
            i = 0 as libc::c_int as size_t;
            while i < (*cur).num_children {
                let ref mut fresh221 = (*(*((*cur).children).offset(i as isize)).name).chain;
                *fresh221 = (*cur).chain;
                let ref mut fresh222 = (*cur).chain;
                *fresh222 = (*((*cur).children).offset(i as isize)).name;
                i = i.wrapping_add(1);
            }
            cb.expect("non-null function pointer")(in_0, ns, cur, data);
            cur = (*cur).chain;
        }
    }
    #[c2rust::src_loc = "331:1"]
    pub unsafe extern "C" fn _tl_add_symbol(
        mut in_0: *mut tl_interp,
        mut _ns: *mut tl_ns,
        mut name: *mut tl_name,
        mut data: *mut libc::c_void,
    ) {
        let mut cell = data as *mut tl_object;
        let ref mut fresh223 = (*cell).data.pair.first;
        *fresh223 = tl_new_pair(
            in_0,
            tl_new_sym_name(in_0, name),
            (*cell).data.pair.first,
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
        let ref mut fresh224 = (*in_0).values;
        *fresh224 = tl_new_pair(
            in_0,
            tl_new_pair(
                in_0,
                if !cell.is_null()
                    && (cell.is_null()
                        || (*cell).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cell).data.pair.first
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
    pub static mut init_tl_cf_all_symbols: tl_init_ent =
        unsafe { TL_CF!(all_symbols, "all-symbols") };
    #[no_mangle]
    #[c2rust::src_loc = "342:1"]
    pub unsafe extern "C" fn tl_cf_print_ns(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        tl_ns_print(in_0, &mut (*in_0).ns);
        let ref mut fresh225 = (*in_0).values;
        *fresh225 = tl_new_pair(
            in_0,
            tl_new_pair(in_0, (*in_0).true_, (*in_0).false_),
            (*in_0).values,
        );
    }
    #[link_section = "tl_init_ents"]
    #[used]
    #[c2rust::src_loc = "342:1"]
    pub static mut init_tl_cf_print_ns: tl_init_ent = unsafe { TL_CF!(print_ns, "print-ns") };
    use super::object_c::{tl_new_pair, tl_new_sym_name};
    use super::print_c::{tl_printf, tl_puts};
    use super::stddef_h::{size_t, NULL};
    use super::string_h::{memcmp, memcpy, memmove};
    use super::tinylisp_h::{
        tl_buffer, tl_child, tl_init_ent, tl_interp, tl_name, tl_name_s, tl_ns, tl_object,
        tl_object_s, ObjectTag, TL_EMPTY_LIST, TL_PAIR,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/object.c:8"]
pub mod object_c {
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
        }
        let ref mut fresh226 = (*obj).gclink.next_alloc;
        *fresh226 = (*in_0).top_alloc;
        let ref mut fresh227 = (*obj).prev_alloc;
        *fresh227 = NULL as *mut tl_object_s;
        if !((*in_0).top_alloc).is_null() {
            let ref mut fresh228 = (*(*in_0).top_alloc).prev_alloc;
            *fresh228 = obj;
        }
        let ref mut fresh229 = (*in_0).top_alloc;
        *fresh229 = obj;
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
        (*obj).data.ival = ival;
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
        let ref mut fresh230 = (*obj).data.nm;
        *fresh230 = name;
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
        let ref mut fresh231 = (*obj).data.pair.first;
        *fresh231 = first;
        let ref mut fresh232 = (*obj).data.pair.next;
        *fresh232 = next;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn tl_new_then(
        mut in_0: *mut tl_interp,
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
        mut state: *mut tl_object,
        mut name: *const libc::c_char,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_THEN;
        let ref mut fresh233 = (*obj).data.then.cfunc;
        *fresh233 = cfunc;
        let ref mut fresh234 = (*obj).data.then.state;
        *fresh234 = state;
        let ref mut fresh235 = (*obj).data.then.name;
        *fresh235 = if !name.is_null() {
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
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
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
        mut cfunc: Option<
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        >,
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
        }) as ObjectTag;
        let ref mut fresh236 = (*obj).data.func.args;
        *fresh236 = args;
        let ref mut fresh237 = (*obj).data.func.body;
        *fresh237 = body;
        let ref mut fresh238 = (*obj).data.func.env;
        *fresh238 = env;
        let ref mut fresh239 = (*obj).data.func.envn;
        *fresh239 = envn;
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
        let ref mut fresh240 = (*obj).data.cont.ret_env;
        *fresh240 = env;
        let ref mut fresh241 = (*obj).data.cont.ret_conts;
        *fresh241 = conts;
        let ref mut fresh242 = (*obj).data.cont.ret_values;
        *fresh242 = values;
        return obj;
    }
    #[no_mangle]
    #[c2rust::src_loc = "170:1"]
    pub unsafe extern "C" fn tl_free(mut in_0: *mut tl_interp, mut obj: *mut tl_object) {
        if obj.is_null() {
            return;
        }
        if !((*obj).prev_alloc).is_null() {
            let ref mut fresh243 = (*(*obj).prev_alloc).gclink.next_alloc;
            *fresh243 = tl_make_next_alloc!(
                obj -> prev_alloc -> next_alloc, tl_next_alloc(obj)
            );
        } else {
            let ref mut fresh244 = (*in_0).top_alloc;
            *fresh244 = tl_make_next_alloc!(in -> top_alloc, tl_next_alloc(obj));
        }
        if !tl_next_alloc!(obj).is_null() {
            let ref mut fresh245 = (*tl_next_alloc!(obj)).prev_alloc;
            *fresh245 = (*obj).prev_alloc;
        }
        match (*obj).kind as libc::c_uint {
            4 | 5 | 3 => {
                tl_alloc_free!(in, obj -> name).expect("non-null function pointer")(
                    tl_alloc_free!(in, obj -> name),
                    tl_alloc_free!(in, obj -> name),
                    tl_alloc_free!(in, obj -> name),
                );
            }
            _ => {}
        }
        tl_alloc_free!(in, obj).expect("non-null function pointer")(
            tl_alloc_free!(in, obj),
            tl_alloc_free!(in, obj),
            tl_alloc_free!(in, obj),
        );
    }
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn _tl_mark_pass(mut obj: *mut tl_object) {
        if obj.is_null() {
            return;
        }
        if tl_is_marked!(obj) != 0 {
            return;
        }
        let ref mut fresh246 = tl_mark!(obj);
        *fresh246 |= TL_F_MARK as libc::c_ulong;
        match (*obj).kind as libc::c_uint {
            4 | 5 | 3 => {
                _tl_mark_pass((*obj).data.then.state);
            }
            7 | 6 => {
                _tl_mark_pass((*obj).data.func.args);
                _tl_mark_pass((*obj).data.func.body);
                _tl_mark_pass((*obj).data.func.env);
                _tl_mark_pass((*obj).data.func.envn);
            }
            2 => {
                _tl_mark_pass((*obj).data.pair.first);
                _tl_mark_pass((*obj).data.pair.next);
            }
            8 => {
                _tl_mark_pass((*obj).data.cont.ret_env);
                _tl_mark_pass((*obj).data.cont.ret_conts);
                _tl_mark_pass((*obj).data.cont.ret_values);
            }
            0 | 1 | _ => {}
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "251:1"]
    pub unsafe extern "C" fn tl_gc(mut in_0: *mut tl_interp) {
        let mut obj = (*in_0).top_alloc;
        let mut tmp = 0 as *mut tl_object;
        while !obj.is_null() {
            let ref mut fresh247 = tl_unmark!(obj);
            *fresh247 &= !TL_FMASK as libc::c_ulong;
            obj = tl_next_alloc!(obj);
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
            if tl_is_permanent!(obj) != 0 {
                _tl_mark_pass(obj);
            }
            obj = tl_next_alloc!(obj);
        }
        obj = (*in_0).top_alloc;
        while !obj.is_null() {
            tmp = obj;
            obj = tl_next_alloc!(obj);
            if tl_is_marked!(tmp) == 0 {
                tl_free(in_0, tmp);
            }
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "304:1"]
    pub unsafe extern "C" fn tl_list_len(mut l: *mut tl_object) -> size_t {
        let mut cnt = 0 as libc::c_int as size_t;
        if l.is_null() || tl_is_pair!(l) == 0 {
            return cnt;
        }
        let mut l_item = l;
        let mut item = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).data.pair.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            cnt = cnt.wrapping_add(1);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
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
            (*l).data.pair.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
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
        let mut res = tl_first!(l);
        l = tl_next!(l);
        let mut l_item = l;
        let mut item = if !l.is_null()
            && (l.is_null() || (*l).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*l).data.pair.first
        } else {
            NULL as *mut tl_object_s
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                NULL as *mut tl_object_s
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
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
        buf = tl_alloc_malloc!(in, s);
        if buf.is_null() {
            tl_gc(in_0);
            buf = tl_alloc_malloc!(in, s);
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
        let mut region = tl_alloc_malloc!(in, n * s);
        if region.is_null() {
            return NULL as *mut libc::c_void;
        }
        return memset(region, 0 as libc::c_int, n.wrapping_mul(s));
    }
    use super::ns_c::tl_ns_resolve;
    use super::stddef_h::{size_t, NULL};
    use super::string_h::{memset, strcpy, strlen};
    use super::tinylisp_h::{
        tl_buffer, tl_interp, tl_name, tl_ns, tl_object, tl_object_s, ObjectTag, TL_CFUNC,
        TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FMASK, TL_FUNC, TL_F_MARK, TL_INT, TL_MACRO,
        TL_PAIR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tinylisp/read.c:10"]
pub mod read_c {
    #[c2rust::src_loc = "13:9"]
    pub const DEFAULT_SYM_LEN: libc::c_int = 64 as libc::c_int;
    #[c2rust::src_loc = "85:1"]
    pub unsafe extern "C" fn _tl_read_top_prefix_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let ref mut fresh249 = (*in_0).values;
        *fresh249 = tl_new_pair(
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
                            (*args).data.pair.first
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
    #[c2rust::src_loc = "135:1"]
    pub unsafe extern "C" fn _tl_read_comment_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh250 = (*in_0).error;
                *fresh250 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh251 = (*in_0).values;
            *fresh251 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh252 = reader_prologue!(in, args);
        *fresh252 = reader_prologue!(in, args);
        match ch {
            EOF => {
                let ref mut fresh253 = (*in_0).values;
                *fresh253 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, 0 as *mut tl_object, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            10 => {
                tl_getc_and_then!(in, state, _tl_read_top_k)(
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                tl_getc_and_then!(in, state, _tl_read_comment_k)(
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    b"tl_getc_and_then:_tl_read_comment_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "152:1"]
    pub unsafe extern "C" fn _tl_read_string_k(
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
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh254 = (*in_0).error;
                *fresh254 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh255 = (*in_0).values;
            *fresh255 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh256 = reader_prologue!(in, args);
        *fresh256 = reader_prologue!(in, args);
        if make_read_buffer!(in).is_null() {
            let ref mut fresh257 = make_read_buffer!(in);
            *fresh257 = make_read_buffer!(in);
            let ref mut fresh258 = make_read_buffer!(in);
            *fresh258 = DEFAULT_SYM_LEN as size_t;
            let ref mut fresh259 = make_read_buffer!(in);
            *fresh259 = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                (*in_0).read_sz,
            ) as *mut libc::c_char;
            if !((*in_0).read_buffer).is_null() {
            } else {
                __assert_fail(
                    b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                    b"./read.c\0" as *const u8 as *const libc::c_char,
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
                    let ref mut fresh260 = (*in_0).error;
                    *fresh260 =
                        tl_new_sym(in_0, b"EOF in string\0" as *const u8 as *const libc::c_char);
                };
                let ref mut fresh261 = (*in_0).values;
                *fresh261 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            34 => {
                sym = tl_new_sym_data(in_0, (*in_0).read_buffer, (*in_0).read_ptr);
                tl_alloc_free!(in, in -> read_buffer).expect("non-null function pointer")(
                    tl_alloc_free!(in, in -> read_buffer),
                    tl_alloc_free!(in, in -> read_buffer),
                    tl_alloc_free!(in, in -> read_buffer),
                );
                let ref mut fresh262 = (*in_0).read_buffer;
                *fresh262 = NULL as *mut libc::c_char;
                let ref mut fresh263 = (*in_0).values;
                *fresh263 =
                    tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
                return;
            }
            _ => {
                let ref mut fresh264 = (*in_0).read_ptr;
                let fresh265 = *fresh264;
                *fresh264 = (*fresh264).wrapping_add(1);
                *((*in_0).read_buffer).offset(fresh265 as isize) = ch as libc::c_char;
                if (*in_0).read_ptr >= (*in_0).read_sz {
                    (*in_0).read_sz <<= add_to_cstr!(
                        in, in -> read_buffer, in -> read_sz, in -> read_ptr, ch
                    );
                    let ref mut fresh266 = (*in_0).read_buffer;
                    *fresh266 = ((*in_0).reallocf).expect("non-null function pointer")(
                        in_0,
                        (*in_0).read_buffer as *mut libc::c_void,
                        (*in_0).read_sz,
                    ) as *mut libc::c_char;
                    if !((*in_0).read_buffer).is_null() {
                    } else {
                        __assert_fail(
                            b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                            b"./read.c\0" as *const u8 as *const libc::c_char,
                            170 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                                b"void _tl_read_string_k(tl_interp *, tl_object *, tl_object *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                }
                tl_getc_and_then!(in, state, _tl_read_string_k)(
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    b"tl_getc_and_then:_tl_read_string_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "177:1"]
    pub unsafe extern "C" fn _tl_read_pair_cons_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        state = tl_new_pair(in_0, tl_first!(args), state);
        tl_getc_and_then!(in, state, _tl_read_list_k)(
            tl_getc_and_then!(in, state, _tl_read_list_k),
            tl_getc_and_then!(in, state, _tl_read_list_k),
            tl_getc_and_then!(in, state, _tl_read_list_k),
            b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "182:1"]
    pub unsafe extern "C" fn _tl_read_pair_improp_check_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh267 = (*in_0).error;
                *fresh267 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh268 = (*in_0).values;
            *fresh268 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh269 = reader_prologue!(in, args);
        *fresh269 = reader_prologue!(in, args);
        match ch {
            32 | 10 | 9 | 11 | 13 | 8 => {
                tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k)(
                    tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
                    tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
                    tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
                    b"tl_getc_and_then:_tl_read_pair_improp_check_k\0" as *const u8
                        as *const libc::c_char,
                );
            }
            41 => {
                let ref mut fresh270 = (*in_0).values;
                *fresh270 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, tl_list_rvs_improp(in_0, state), (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            _ => {
                state = tl_new_pair(
                    in_0,
                    tl_first!(state),
                    tl_new_pair(
                        in_0,
                        tl_new_sym(in_0, b".\0" as *const u8 as *const libc::c_char),
                        tl_next!(state),
                    ),
                );
                tl_getc_and_then!(in, state, _tl_read_list_k)(
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "200:1"]
    pub unsafe extern "C" fn _tl_read_pair_improp_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        state = tl_new_pair(in_0, tl_first!(args), state);
        tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k)(
            tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
            tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
            tl_getc_and_then!(in, state, _tl_read_pair_improp_check_k),
            b"tl_getc_and_then:_tl_read_pair_improp_check_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "205:1"]
    pub unsafe extern "C" fn _tl_read_list_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh271 = (*in_0).error;
                *fresh271 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh272 = (*in_0).values;
            *fresh272 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh273 = reader_prologue!(in, args);
        *fresh273 = reader_prologue!(in, args);
        match ch {
            32 | 10 | 9 | 11 | 13 | 8 => {
                tl_getc_and_then!(in, state, _tl_read_list_k)(
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    tl_getc_and_then!(in, state, _tl_read_list_k),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
            41 => {
                let ref mut fresh274 = (*in_0).values;
                *fresh274 = tl_new_pair(
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
                tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k)(
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
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
                tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k)(
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[c2rust::src_loc = "236:1"]
    pub unsafe extern "C" fn _tl_read_int_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh275 = (*in_0).error;
                *fresh275 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh276 = (*in_0).values;
            *fresh276 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh277 = reader_prologue!(in, args);
        *fresh277 = reader_prologue!(in, args);
        if isdigit!(ch) != 0 {
            state = tl_new_int(
                in_0,
                (*state).data.ival * 10 as libc::c_int as libc::c_long
                    + (ch - '0' as i32) as libc::c_long,
            );
            tl_getc_and_then!(in, state, _tl_read_int_k)(
                tl_getc_and_then!(in, state, _tl_read_int_k),
                tl_getc_and_then!(in, state, _tl_read_int_k),
                tl_getc_and_then!(in, state, _tl_read_int_k),
                b"tl_getc_and_then:_tl_read_int_k\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let ref mut fresh278 = (*in_0).values;
            *fresh278 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, state, (*in_0).false_),
                (*in_0).values,
            );
            return;
        };
    }
    #[c2rust::src_loc = "247:1"]
    pub unsafe extern "C" fn _tl_read_sym_k(
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
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh279 = (*in_0).error;
                *fresh279 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh280 = (*in_0).values;
            *fresh280 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh281 = reader_prologue!(in, args);
        *fresh281 = reader_prologue!(in, args);
        if make_read_buffer!(in).is_null() {
            let ref mut fresh282 = make_read_buffer!(in);
            *fresh282 = make_read_buffer!(in);
            let ref mut fresh283 = make_read_buffer!(in);
            *fresh283 = DEFAULT_SYM_LEN as size_t;
            let ref mut fresh284 = make_read_buffer!(in);
            *fresh284 = ((*in_0).reallocf).expect("non-null function pointer")(
                in_0,
                0 as *mut libc::c_void,
                (*in_0).read_sz,
            ) as *mut libc::c_char;
            if !((*in_0).read_buffer).is_null() {
            } else {
                __assert_fail(
                    b"(in)->read_buffer\0" as *const u8 as *const libc::c_char,
                    b"./read.c\0" as *const u8 as *const libc::c_char,
                    250 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                        b"void _tl_read_sym_k(tl_interp *, tl_object *, tl_object *)\0",
                    ))
                    .as_ptr(),
                );
            }
        }
        match ch {
            40 | 41 | 32 | 10 | 9 | 11 | 13 | 8 | EOF => {
                sym = tl_new_sym_data(in_0, (*in_0).read_buffer, (*in_0).read_ptr);
                tl_alloc_free!(in, in -> read_buffer).expect("non-null function pointer")(
                    tl_alloc_free!(in, in -> read_buffer),
                    tl_alloc_free!(in, in -> read_buffer),
                    tl_alloc_free!(in, in -> read_buffer),
                );
                let ref mut fresh285 = (*in_0).read_buffer;
                *fresh285 = NULL as *mut libc::c_char;
                let ref mut fresh286 = (*in_0).values;
                *fresh286 =
                    tl_new_pair(in_0, tl_new_pair(in_0, sym, (*in_0).false_), (*in_0).values);
                return;
            }
            _ => {
                let ref mut fresh287 = (*in_0).read_ptr;
                let fresh288 = *fresh287;
                *fresh287 = (*fresh287).wrapping_add(1);
                *((*in_0).read_buffer).offset(fresh288 as isize) = ch as libc::c_char;
                if (*in_0).read_ptr >= (*in_0).read_sz {
                    (*in_0).read_sz <<= add_to_cstr!(
                        in, in -> read_buffer, in -> read_sz, in -> read_ptr, ch
                    );
                    let ref mut fresh289 = (*in_0).read_buffer;
                    *fresh289 = ((*in_0).reallocf).expect("non-null function pointer")(
                        in_0,
                        (*in_0).read_buffer as *mut libc::c_void,
                        (*in_0).read_sz,
                    ) as *mut libc::c_char;
                    if !((*in_0).read_buffer).is_null() {
                    } else {
                        __assert_fail(
                            b"in->read_buffer\0" as *const u8 as *const libc::c_char,
                            b"./read.c\0" as *const u8 as *const libc::c_char,
                            263 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                                b"void _tl_read_sym_k(tl_interp *, tl_object *, tl_object *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                }
                tl_getc_and_then!(in, state, _tl_read_sym_k)(
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    b"tl_getc_and_then:_tl_read_sym_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    #[no_mangle]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn tl_read(mut in_0: *mut tl_interp) {
        tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k)(
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
            b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
        );
    }
    #[c2rust::src_loc = "89:1"]
    pub unsafe extern "C" fn _tl_read_top_k(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut state: *mut tl_object,
    ) {
        let mut ch: libc::c_int = 0;
        if !(!(if !args.is_null()
            && (args.is_null()
                || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*args).data.pair.first
        } else {
            0 as *mut tl_object_s
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
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
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            } else {
                let ref mut fresh290 = (*in_0).error;
                *fresh290 = tl_new_pair(
                    in_0,
                    tl_new_sym(in_0, b"not a char\0" as *const u8 as *const libc::c_char),
                    if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object_s
                    },
                );
            };
            let ref mut fresh291 = (*in_0).values;
            *fresh291 = tl_new_pair(
                in_0,
                tl_new_pair(in_0, (*in_0).false_, (*in_0).false_),
                (*in_0).values,
            );
            return;
        }
        let ref mut fresh292 = reader_prologue!(in, args);
        *fresh292 = reader_prologue!(in, args);
        match ch {
            EOF => {
                let ref mut fresh293 = (*in_0).values;
                *fresh293 = tl_new_pair(
                    in_0,
                    tl_new_pair(in_0, 0 as *mut tl_object, (*in_0).false_),
                    (*in_0).values,
                );
                return;
            }
            32 | 10 | 9 | 11 | 13 | 8 => {
                tl_getc_and_then!(in, state, _tl_read_top_k)(
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    tl_getc_and_then!(in, state, _tl_read_top_k),
                    b"tl_getc_and_then:_tl_read_top_k\0" as *const u8 as *const libc::c_char,
                );
            }
            59 => {
                tl_getc_and_then!(in, state, _tl_read_comment_k)(
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    tl_getc_and_then!(in, state, _tl_read_comment_k),
                    b"tl_getc_and_then:_tl_read_comment_k\0" as *const u8 as *const libc::c_char,
                );
            }
            34 => {
                tl_getc_and_then!(in, state, _tl_read_string_k)(
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    tl_getc_and_then!(in, state, _tl_read_string_k),
                    b"tl_getc_and_then:_tl_read_string_k\0" as *const u8 as *const libc::c_char,
                );
            }
            40 => {
                tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_list_k)(
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_list_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_list_k),
                    tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_list_k),
                    b"tl_getc_and_then:_tl_read_list_k\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                if isdigit!(ch) != 0 {
                    tl_getc_and_then!(
                        in, tl_new_int(in, ch - '0'), _tl_read_int_k
                    )(
                        tl_getc_and_then!(in, tl_new_int(in, ch - '0'), _tl_read_int_k),
                        tl_getc_and_then!(in, tl_new_int(in, ch - '0'), _tl_read_int_k),
                        tl_getc_and_then!(in, tl_new_int(in, ch - '0'), _tl_read_int_k),
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
                    (*(*in_0).prefixes).data.pair.first
                } else {
                    NULL as *mut tl_object_s
                };
                while !l_kv.is_null() {
                    let mut k = tl_first!(kv);
                    let mut v = tl_next!(kv);
                    if !k.is_null()
                        && !v.is_null()
                        && tl_is_sym!(k) != 0
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
                        tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k)(
                            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                            tl_getc_and_then!(in, TL_EMPTY_LIST, _tl_read_top_k),
                            b"tl_getc_and_then:_tl_read_top_k\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    }
                    l_kv = (if !l_kv.is_null()
                        && (l_kv.is_null()
                            || (*l_kv).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_kv).data.pair.next
                    } else {
                        NULL as *mut tl_object_s
                    });
                    kv = (if !l_kv.is_null()
                        && (l_kv.is_null()
                            || (*l_kv).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_kv).data.pair.first
                    } else {
                        NULL as *mut tl_object_s
                    });
                }
                tl_getc_and_then!(in, state, _tl_read_sym_k)(
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    tl_getc_and_then!(in, state, _tl_read_sym_k),
                    b"tl_getc_and_then:_tl_read_sym_k\0" as *const u8 as *const libc::c_char,
                );
            }
        };
    }
    use super::assert_h::__assert_fail;
    use super::eval_c::tl_push_apply;
    use super::object_c::{
        tl_list_rvs, tl_list_rvs_improp, tl_new_int, tl_new_pair, tl_new_sym, tl_new_sym_data,
        tl_new_then,
    };
    use super::stddef_h::{size_t, NULL};
    use super::stdio_h::EOF;
    use super::tinylisp_h::{tl_interp, tl_object, tl_object_s, ObjectTag, TL_INT, TL_PAIR};
}
use self::assert_h::__assert_fail;
pub use self::bits_dlfcn_h::{RTLD_GLOBAL, RTLD_NOW};
pub use self::builtin_c::{
    _tl_cf_define_k, _tl_cf_if_k, _tl_cf_set_k, _tl_readc_k, _unboolify, init_tl_cf_add,
    init_tl_cf_all_objects, init_tl_cf_apply, init_tl_cf_call_with_current_continuation,
    init_tl_cf_car, init_tl_cf_cdr, init_tl_cf_chr, init_tl_cf_concat, init_tl_cf_cons,
    init_tl_cf_define, init_tl_cf_display, init_tl_cf_display_sep, init_tl_cf_div, init_tl_cf_env,
    init_tl_cf_eq, init_tl_cf_error, init_tl_cf_evalin, init_tl_cf_gc, init_tl_cf_if,
    init_tl_cf_lambda, init_tl_cf_length, init_tl_cf_less, init_tl_cf_load_mod, init_tl_cf_macro,
    init_tl_cf_mod, init_tl_cf_mul, init_tl_cf_nand, init_tl_cf_null, init_tl_cf_ord,
    init_tl_cf_prefix, init_tl_cf_putbackc, init_tl_cf_read, init_tl_cf_readc, init_tl_cf_rescue,
    init_tl_cf_set, init_tl_cf_setenv, init_tl_cf_sub, init_tl_cf_substr, init_tl_cf_topenv,
    init_tl_cf_type, init_tl_cf_writec, tl_cf_add, tl_cf_all_objects, tl_cf_apply,
    tl_cf_call_with_current_continuation, tl_cf_car, tl_cf_cdr, tl_cf_chr, tl_cf_concat,
    tl_cf_cons, tl_cf_define, tl_cf_display, tl_cf_display_sep, tl_cf_div, tl_cf_env, tl_cf_eq,
    tl_cf_error, tl_cf_evalin, tl_cf_gc, tl_cf_if, tl_cf_lambda, tl_cf_length, tl_cf_less,
    tl_cf_load_mod, tl_cf_macro, tl_cf_mod, tl_cf_mul, tl_cf_nand, tl_cf_null, tl_cf_ord,
    tl_cf_prefix, tl_cf_putbackc, tl_cf_read, tl_cf_readc, tl_cf_rescue, tl_cf_set, tl_cf_setenv,
    tl_cf_sub, tl_cf_substr, tl_cf_topenv, tl_cf_type, tl_cf_writec,
};
pub use self::ctype_h::{
    C2RustUnnamed_7, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower,
    _ISprint, _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc,
};
pub use self::debug_c::{
    _indent, _tl_cf_debug_print_k, init_tl_cf_debug_print, tl_cf_debug_print, tl_dbg_print,
};
use self::dlfcn_h::{dlerror, dlopen, dlsym};
pub use self::env_c::{tl_env_get_kv, tl_env_set_global, tl_env_set_local, tl_frm_set};
pub use self::eval_c::{
    _tl_apply_next_body_callable_k, _tl_eval_all_args, _tl_eval_all_args_k, _tl_eval_and_then,
    _tl_getc_and_then, tl_apply_next, tl_push_apply, tl_push_eval, tl_run_until_done,
};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::interp_c::{
    __start_tl_init_ents, __stop_tl_init_ents, _modloadf, _readf, _reallocf, _writef,
    tl_interp_cleanup, tl_interp_init, tl_interp_init_alloc,
};
use self::main_c::{
    _global_in, _main_k, _main_read_k, _print_cont, _print_cont_stack, init_tl_cf_exit,
    init_tl_cf_quiet, main_0, my_modloadf, print_cont_stack, quiet, running, tl_cf_exit,
    tl_cf_quiet, QUIET_NO_TRUE, QUIET_NO_VALUE, QUIET_OFF,
};
pub use self::ns_c::{
    _tl_add_symbol, init_tl_cf_all_symbols, init_tl_cf_print_ns, tl_buf_slice, tl_cf_all_symbols,
    tl_cf_print_ns, tl_ns_for_each, tl_ns_free, tl_ns_init, tl_ns_print, tl_ns_print_child,
    tl_ns_print_name, tl_ns_resolve, tl_ns_split,
};
pub use self::object_c::{
    _tl_mark_pass, _tl_new_cfunc, _tl_new_cfunc_byval, tl_calloc, tl_free, tl_gc, tl_list_len,
    tl_list_rvs, tl_list_rvs_improp, tl_new, tl_new_cont, tl_new_int, tl_new_macro, tl_new_pair,
    tl_new_sym, tl_new_sym_data, tl_new_sym_name, tl_new_then, tl_strdup,
};
pub use self::print_c::{
    C2RustUnnamed_6, _mempbrk, _print_pairs, tl_print, tl_printf, tl_puts, tl_write,
    QUOTED_SYM_CHARS,
};
pub use self::read_c::{
    _tl_read_comment_k, _tl_read_int_k, _tl_read_list_k, _tl_read_pair_cons_k,
    _tl_read_pair_improp_check_k, _tl_read_pair_improp_k, _tl_read_string_k, _tl_read_sym_k,
    _tl_read_top_k, _tl_read_top_prefix_k, tl_read, DEFAULT_SYM_LEN,
};
pub use self::stdarg_h::va_list;
pub use self::stddef_h::{size_t, NULL, NULL_0};
pub use self::stdio_h::{
    fflush, fprintf, fputc, fwrite, getchar, putchar, snprintf, stderr, stdout, EOF,
};
use self::stdlib_h::{exit, free, realloc};
use self::string_h::{memcmp, memcpy, memmove, memset, strcpy, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::tinylisp_h::{
    tl_buffer, tl_buffer_s, tl_child, tl_child_s, tl_init_ent, tl_init_ent_s, tl_init_ent_s_Inner,
    tl_init_ent_s_PADDING, tl_interp, tl_interp_s, tl_name, tl_name_s, tl_ns, tl_ns_s, tl_object,
    tl_object_s, GcLink, ObjData, ContData, FuncData, ThenData,
    PairData, ObjectTag, TL_APPLY_DROP, TL_APPLY_DROP_EVAL, TL_APPLY_DROP_RESCUE,
    TL_APPLY_GETCHAR, TL_APPLY_INDIRECT, TL_APPLY_PUSH_EVAL, TL_CFUNC, TL_CFUNC_BYVAL, TL_CONT,
    TL_DEFAULT_GC_EVENTS, TL_EMPTY_LIST, TL_FMASK, TL_FUNC, TL_F_MARK, TL_F_PERMANENT, TL_INT,
    TL_MACRO, TL_PAIR, TL_RESULT_AGAIN, TL_RESULT_DONE, TL_RESULT_GETCHAR, TL_SYM, TL_THEN,
};
pub use self::types_h::{__off64_t, __off_t};
pub use self::unistd_h::{isatty, STDIN_FILENO};
pub use self::FILE_h::FILE;
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
