use super::{TL_PAIR, TL_EMPTY_LIST, TL_SYM};
use super::object_c::tl_new_pair;
use bantaml_macros::tl_cf;
use crate::tl::{
 tl_interp, tl_object, TL_CFUNC, TL_CFUNC_BYVAL, TL_MACRO,
};


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

macro_rules! tl_is_permanent { ($obj:expr) => {  ((*$obj).gclink.next_alloc_i & crate::tl::TL_F_PERMANENT as _) } }

macro_rules! tl_next_alloc { ($obj:expr) => { todo!() }} //  ((tl_object *)((obj)->next_alloc_i & (~TL_FMASK))) } }

macro_rules! tl_make_next_alloc { ($ptr:expr) => {  todo!() } } // ((tl_object *)(((obj)->next_alloc_i & (~TL_FMASK)) | (((size_t)(orig)) & TL_FMASK))) } }

macro_rules! tl_is_int { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tl::TL_INT) } } }
macro_rules! tl_is_sym { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tl::TL_SYM) } } }

macro_rules! tl_is_pair { ($obj:expr) => {  { let obj =$obj;(obj.is_null() || (*obj).kind == crate::tl::TL_PAIR) } }}
macro_rules! tl_is_then { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tl::TL_THEN) } } }
macro_rules! tl_is_cfunc { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tl::TL_CFUNC) } }}
macro_rules! tl_is_cfunc_byval { ($obj:expr) => {  { let obj =$obj;(!obj.is_null() && (*obj).kind == crate::tl::TL_CFUNC_BYVAL) } }}
macro_rules! tl_is_macro { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tl::TL_MACRO) } }}
macro_rules! tl_is_func { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tl::TL_FUNC) } }}
macro_rules! tl_is_cont { ($obj:expr) => { { let obj =$obj; (!obj.is_null() && (*obj).kind == crate::tl::TL_CONT) } }}
macro_rules! tl_is_callable { ($obj:expr) => { { let obj =$obj; (tl_is_cfunc!(obj) || tl_is_cfunc_byval!(obj) || tl_is_then!(obj)|| tl_is_macro!(obj) || tl_is_func!(obj) || tl_is_cont!(obj)) } }}

macro_rules! tl_first { ($obj:expr) => { { let obj = $obj; (if (!obj.is_null() && tl_is_pair!(obj))  { (*obj).data.pair.first }else{std::ptr::null_mut()}) } } }
macro_rules! tl_next { ($obj:expr) => {  { let obj = $obj; (if (!obj.is_null() && tl_is_pair!(obj))  { (*obj).data.pair.next } else {std::ptr::null_mut()}) } } }

macro_rules! tl_list_iter { ($it:expr) => {  todo!() } }
macro_rules! tl_sym_eq { ($a:expr,$b:expr) => {  (tl_is_sym!($a) && tl_is_sym!($b) && (*$a).data.nm == (*$b).data.nm) } }
macro_rules! tl_sym_less { ($b:expr) => {  (tl_is_sym(a) && tl_is_sym(b) && ((a)->nm->here.len < (b)->nm->here.len && !((b)->nm->here.len < (a)->nm->here.len) || memcmp((a)->nm->here.data, (b)->nm->here.data, (a)->nm->here.len) < 0)) } }

macro_rules! tl_error_set { ($er:expr) => { if (*in).error.is_null() { (*in).error = $er } else { $er; } } }
macro_rules! tl_error_clear { ($in:expr) => {  ((*$in).error = std::ptr::null_mut()) } }
macro_rules! tl_has_error { ($in:expr) => {  ((in)->error) } }

macro_rules! tl_getc { ($in:expr) => {  if (*in).is_putback { (*$in).is_putback = 0; (*$in).putback } else { ((*$in).readf)($in) } } }
macro_rules! tl_putback { ($c:expr) => {  { (*$in).is_putback = 1; (*$in).putback = $c; } } }
macro_rules! tl_putc { ($in:expr,$c:expr) => { {let in0 = $in; ((*in0).writef.unwrap())(in0,$c as i8); }} }

macro_rules! tl_alloc_realloc { ($in:expr, $p:expr, $n:expr) => {  (((*$in).reallocf.expect("non-null function pointer"))($in, $p, $n)) } }

macro_rules! tl_alloc_malloc { ($in:expr, $n:expr) => {  tl_alloc_realloc!($in, std::ptr::null_mut(), n) } }
macro_rules! tl_alloc_free { ($in:expr, $ptr:expr) => {  tl_alloc_realloc!($in, $ptr, 0) } }

macro_rules! tl_values_push { ($in:expr, $v:expr) => {  (*$in).values = tl_new_pair(($in), tl_new_pair(($in), ($v), (*$in).false_), (*$in).values) } }

macro_rules! tl_values_push_syntactic { ($v:expr) => {  (in)->values = tl_new_pair((in), tl_new_pair((in), (v), (in)->true_), (in)->values) } }
macro_rules! tl_values_pop_into { ($var:expr) => {  todo!() } }
macro_rules! tl_rescue_push { ($in:expr, $cont:expr) => {  { *($in).rescue = tl_new_pair($in, (cont), (*$in).rescue); } } }
macro_rules! tl_rescue_peek { ($in:expr) => {  tl_first((*$in).rescue) } }
macro_rules! tl_rescue_drop { ($in:expr) => {  (*$in).rescue = tl_next!((*$in).rescue) } }
macro_rules! tl_cfunc_return { ($in:expr, $v:expr) => {  tl_values_push(($in), ($v)); return; } }
macro_rules! tl_eval_and_then {
    ($in:expr,$ex:expr,$st:expr,$cb:expr) => {
        crate::tl::_tl_eval_and_then($in,$ex,$st,$cb,concat!("tl_eval_and_then", stringify!($cb)).as_ptr() as *const i8)
    }
}
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

#define tl_first(obj) ((!obj.is_null() && tl_is_pair(obj)) ? (obj)->first : std::ptr::null_mut())
#define tl_next(obj) ((!obj.is_null() && tl_is_pair(obj)) ? (obj)->next : std::ptr::null_mut())

#define tl_list_iter(obj, it) tl_object *l_##it = obj, *it = tl_first(obj); l_##it; l_##it = tl_next(l_##it), it = tl_first(l_##it)
TL_EXTERN size_t tl_list_len(tl_object *);
TL_EXTERN tl_object *tl_list_rvs(tl_interp *, tl_object *);
TL_EXTERN tl_object *tl_list_rvs_improp(tl_interp *, tl_object *);

#define tl_sym_eq(a, b) (tl_is_sym(a) && tl_is_sym(b) && (a)->nm == (b)->nm)
#define tl_sym_less(a, b) (tl_is_sym(a) && tl_is_sym(b) && ((a)->nm->here.len < (b)->nm->here.len && !((b)->nm->here.len < (a)->nm->here.len) || memcmp((a)->nm->here.data, (b)->nm->here.data, (a)->nm->here.len) < 0))

#define tl_error_set(in, er) ((in)->error ? (er) : ((in)->error = (er)))
#define tl_error_clear(in) ((in)->error = std::ptr::null_mut())
#define tl_has_error(in) ((in)->error)

#define tl_getc(in) ((in)->is_putback ? ((in)->is_putback = 0, (in)->putback) : (in)->readf((in)))
#define tl_putback(in, c) ((in)->is_putback = 1, (in)->putback = (c))
#define tl_putc(in, c) ((in)->writef((in), (c)))

#define tl_alloc_realloc(in, p, n) ((in)->reallocf((in), (p), (n)))

#define tl_alloc_malloc(in, n) tl_alloc_realloc(in, std::ptr::null_mut(), n)
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
use crate::tl::tl_buffer;

    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:2"]
    pub union C2RustUnnamed_6 {
        pub s: *const libc::c_char,
        pub b: core::mem::ManuallyDrop<*mut tl_buffer>,
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
                cur = std::ptr::null_mut() as *mut tl_object;
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
        return std::ptr::null_mut() as *mut libc::c_char;
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
                if (*(*obj).data.nm).here.len == 0
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
            tl_putc!(in_0, {let v = * s ; s = s.offset(1); v});
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
                        temp.s = ap.arg();
                        if (temp.s).is_null() {
                            tl_puts(in_0, b"<std::ptr::null_mut()>\0" as *const u8 as *const libc::c_char);
                        } else {
                            tl_puts(in_0, temp.s);
                        }
                        cur = cur.offset(1);
                    }
                    112 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32,
                            b"%p\0" as *const u8 as *const libc::c_char,
                            ap.arg::<*const ()>(),
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    108 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            ap.arg::<libc::c_long>()
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(2 as libc::c_int as isize);
                    }
                    100 => {
                        snprintf(
                            buf.as_mut_ptr(),
                            32,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            ap.arg::<libc::c_int>()
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        cur = cur.offset(1);
                    }
                    78 => {
                        temp.b = core::mem::ManuallyDrop::new(ap.arg());
                        snprintf(
                            buf.as_mut_ptr(),
                            32,
                            b"%ld\0" as *const u8 as *const libc::c_char,
                            (**temp.b).len,
                        );
                        tl_puts(in_0, buf.as_mut_ptr());
                        tl_putc!(in_0, ':');
                        tl_write(in_0, (**temp.b).data, (**temp.b).len);
                        cur = cur.offset(1);
                    }
                    79 => {
                        tl_print(in_0, ap.arg());
                        cur = cur.offset(1);
                    }
                    _ => {
                        tl_putc!(in_0, '%');
                        tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } );
                    }
                }
            } else {
                tl_putc!(in_0, {let _v = *cur; cur = cur.offset(1); _v } );
            }
        }
    }


    #[no_mangle]
    #[c2rust::src_loc = "6:1"]
    pub unsafe extern "C" fn _indent(mut lev: libc::c_int) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < lev {
            eprint!("  ");
            i += 1;
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "13:1"]
    pub unsafe extern "C" fn tl_dbg_print(mut obj: *mut tl_object, mut level: libc::c_int) {
        _indent(level);
        if obj.is_null() {
            eprintln!("() (NULL object)\n");
            return;
        }
        match (*obj).kind as libc::c_uint {
            0 => {
                eprintln!(
                    "INT: {}" ,
                    (*obj).data.ival,
                );
            }
            1 => {
                eprint!(
                    "SYM: len={}: ",
                    (*(*obj).data.nm).here.len,
                );
                let d = core::slice::from_raw_parts((*(*obj).data.nm).here.data as *const u8,
                (*(*obj).data.nm).here.len);
                std::io::Write::write_all(&mut std::io::stderr(), d);
                eprintln!();
            }
            2 => {
                eprintln!("PAIR:");
                _indent(level + 1 as libc::c_int);
                eprintln!( "first:" );
                tl_dbg_print(
                    (*obj).data.pair.first,
                    level + 2 as libc::c_int,
                );
                _indent(level + 1 as libc::c_int);
                eprintln!("next:"); 
                tl_dbg_print(
                    (*obj).data.pair.next,
                    level + 2 as libc::c_int,
                );
            }
            4 | 5 | 3 => {
                eprintln!(
                    "{}: {:p}",
                    if (*obj).kind as libc::c_uint == TL_CFUNC as libc::c_int as libc::c_uint {
                        "CFUNC" 
                    } else if (*obj).kind as libc::c_uint
                        == TL_CFUNC_BYVAL as libc::c_int as libc::c_uint
                    {
                        "CFUNC_BYVAL" 
                    } else {
                        "THEN" 
                    },
                    (*obj).data.then.cfunc.expect("non-null function pointer"),
                );
                _indent(level + 1 as libc::c_int);
                eprintln!("state:");
                tl_dbg_print(
                    (*obj).data.then.state,
                    level + 2 as libc::c_int,
                );
            }
            6 | 7 => {
                eprintln!(
                    
                 "{}:" ,
                    if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                        "MACRO" 
                    } else {
                        "FUNC" 
                    },
                );
                _indent(level + 1 as libc::c_int);
                eprintln!("args:");
                tl_dbg_print(
                    (*obj).data.func.args,
                    level + 2 as libc::c_int,
                );
                if (*obj).kind as libc::c_uint == TL_MACRO as libc::c_int as libc::c_uint {
                    _indent(level + 1 as libc::c_int);
                    eprintln!("envn:"); 
                    tl_dbg_print(
                        (*obj).data.func.envn,
                        level + 2 as libc::c_int,
                    );
                }
                _indent(level + 1 as libc::c_int);
                eprintln!("body:" );
                tl_dbg_print(
                    (*obj).data.func.body,
                    level + 2 as libc::c_int,
                );
            }
            8 => {
                eprintln!(
                    
                    "CONTINUATION:"
                );
                _indent(level + 1 as libc::c_int);
                eprintln!(
                    
                    "ret_conts:" 
                );
                tl_dbg_print(
                    (*obj).data.cont.ret_conts,
                    level + 2 as libc::c_int,
                );
                _indent(level + 1 as libc::c_int);
                eprintln!(
                    
                    "ret_values:" 
                );
                tl_dbg_print(
                    (*obj).data.cont.ret_values,
                    level + 2 as libc::c_int,
                );
            }
            _ => {
                eprintln!(
                    
                    "!!! UNKNOWN OBJECT KIND {}" ,
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
        eprintln!("VALUE:");
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
    #[tl_cf(name = "debug-print")]
    pub unsafe extern "C" fn tl_cf_debug_print(
        mut in_0: *mut tl_interp,
        mut args: *mut tl_object,
        mut _unused: *mut tl_object,
    ) {
        eprintln!("EXPR:");
        tl_dbg_print(tl_first!(args), 0 as libc::c_int);
        tl_eval_and_then!(in_0, tl_first!(args), std::ptr::null_mut(), _tl_cf_debug_print_k);
    }

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
            core::ptr::null_mut() as *mut tl_object
        };
        while !l_frame.is_null() {
            let mut l_kv = frame;
            let mut kv = if !frame.is_null()
                && (frame.is_null()
                    || (*frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*frame).data.pair.first
            } else {
                core::ptr::null_mut() as *mut tl_object
            };
            while !l_kv.is_null() {
                let mut key = tl_first!(kv);
                let mut val = tl_next!(kv);
                if !key.is_null() && tl_is_sym!(key) && tl_sym_eq!(key, nm) {
                    return kv;
                }
                l_kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).data.pair.next
                } else {
                    core::ptr::null_mut() as *mut tl_object
                });
                kv = (if !l_kv.is_null()
                    && (l_kv.is_null()
                        || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_kv).data.pair.first
                } else {
                    core::ptr::null_mut() as *mut tl_object
                });
            }
            l_frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.next
            } else {
                core::ptr::null_mut() as *mut tl_object
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.first
            } else {
                core::ptr::null_mut() as *mut tl_object
            });
        }
        return core::ptr::null_mut() as *mut tl_object;
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
        if !kv.is_null() && tl_is_pair!(kv) {
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
            core::ptr::null_mut() as *mut tl_object
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
                core::ptr::null_mut() as *mut tl_object
            });
            frame = (if !l_frame.is_null()
                && (l_frame.is_null()
                    || (*l_frame).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_frame).data.pair.first
            } else {
                core::ptr::null_mut() as *mut tl_object
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
            core::ptr::null_mut() as *mut tl_object
        };
        while !l_kv.is_null() {
            if !kv.is_null()
                && tl_is_pair!(kv)
                && (!(if !kv.is_null()
                    && (kv.is_null()
                        || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*kv).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    && (*(if !kv.is_null()
                        && (kv.is_null()
                            || (*kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*kv).data.pair.first
                    } else {
                        0 as *mut tl_object
                    }))
                    .kind as libc::c_uint
                        == TL_SYM as libc::c_int as libc::c_uint)
                && tl_sym_eq!(tl_first!(kv), nm)
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
                core::ptr::null_mut() as *mut tl_object
            });
            kv = (if !l_kv.is_null()
                && (l_kv.is_null()
                    || (*l_kv).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_kv).data.pair.first
            } else {
                core::ptr::null_mut() as *mut tl_object
            });
        }
        return tl_new_pair(in_0, tl_new_pair(in_0, nm, val), frm);
    }


#[c2rust::header_src = "/home/ember/src/tl/eval.c:4"]
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
                        0 as *mut tl_object
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
                core::ptr::null_mut() as *mut tl_object
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
                    core::ptr::null_mut() as *mut tl_object
                });
                subex = (if !l_subex.is_null()
                    && (l_subex.is_null()
                        || (*l_subex).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_subex).data.pair.first
                } else {
                    core::ptr::null_mut() as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .first
        } else {
            core::ptr::null_mut() as *mut tl_object
        };
        let mut env = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .next
        } else {
            core::ptr::null_mut() as *mut tl_object
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
                core::ptr::null_mut() as *mut tl_object
            };
            while !l_item.is_null() {
                paramlen += 1;
                if !(if !l_item.is_null()
                    && (l_item.is_null()
                        || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_item).data.pair.next
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    && (*(if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).data.pair.next
                    } else {
                        0 as *mut tl_object
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
                        core::ptr::null_mut() as *mut tl_object
                    });
                    item = (if !l_item.is_null()
                        && (l_item.is_null()
                            || (*l_item).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_item).data.pair.first
                    } else {
                        core::ptr::null_mut() as *mut tl_object
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
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !acur.is_null()
                        && (acur.is_null()
                            || (*acur).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*acur).data.pair.next
                    } else {
                        0 as *mut tl_object
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
            core::ptr::null_mut() as *mut tl_object
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
                core::ptr::null_mut() as *mut tl_object
            });
            ex = (if !l_ex.is_null()
                && (l_ex.is_null()
                    || (*l_ex).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_ex).data.pair.first
            } else {
                core::ptr::null_mut() as *mut tl_object
            });
        }
    }
    #[no_mangle]
    #[c2rust::src_loc = "255:1"]
    pub unsafe extern "C" fn tl_apply_next(mut in_0: *mut tl_interp) -> libc::c_int {
        let mut cont = tl_first!(in_0 -> conts);
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
                core::ptr::null_mut() as *mut tl_object
            };
            tl_push_apply(in_0, 1 as libc::c_int as libc::c_long, rescue, (*in_0).env);
            let ref mut fresh150 = tl_values_push!(in, in -> error);
            *fresh150 = tl_values_push!(in, in -> error);
            let ref mut fresh151 = tl_error_clear!(in);
            *fresh151 = core::ptr::null_mut() as *mut tl_object;
            return TL_RESULT_AGAIN;
        }
        let ref mut fresh152 = (*in_0).current;
        *fresh152 = cont;
        if cont.is_null() {
            return TL_RESULT_DONE;
        }
        let ref mut fresh153 = (*in_0).conts;
        *fresh153 = tl_next!(in_0 -> conts);
        if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.first
        } else {
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.first
            } else {
                0 as *mut tl_object
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
        len = (*tl_first!(cont)).data.ival;
        callex = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .first
        } else {
            core::ptr::null_mut() as *mut tl_object
        };
        env = if !(if !cont.is_null()
            && (cont.is_null()
                || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*cont).data.pair.next
        } else {
            0 as *mut tl_object
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .next
        } else {
            core::ptr::null_mut() as *mut tl_object
        };
        if len == TL_APPLY_DROP as libc::c_long {
            let ref mut fresh154 = (*in_0).values;
            *fresh154 = tl_next!((*in_0).values);
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_DROP_RESCUE as libc::c_long {
            let ref mut fresh155 = tl_rescue_drop!(in_0);
            *fresh155 = if !((*in_0).rescue).is_null()
                && (((*in_0).rescue).is_null()
                    || (*(*in_0).rescue).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).rescue).data.pair.next
            } else {
                core::ptr::null_mut() as *mut tl_object
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
                    cont = tl_first!((*in_0).conts);
                    let ref mut fresh157 = (*in_0).conts;
                    *fresh157 = tl_next!((*in_0).conts);
                    tl_push_apply(
                        in_0,
                        TL_APPLY_INDIRECT as libc::c_long,
                        tl_new_int(in_0, len),
                        env,
                    );
                    let ref mut fresh158 = (*in_0).conts;
                    *fresh158 = tl_new_pair(in_0, cont, (*in_0).conts);
                } else if len == TL_APPLY_DROP_EVAL as libc::c_long {
                    cont = tl_first!((*in_0).conts);
                    let ref mut fresh159 = (*in_0).conts;
                    *fresh159 = tl_next!((*in_0).conts);
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
                0 as *mut tl_object
            })
            .is_null()
                && ((if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !cont.is_null()
                        && (cont.is_null()
                            || (*cont).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*cont).data.pair.next
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                })
                .data
                .pair
                .first
            } else {
                core::ptr::null_mut() as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && ((if !((*in_0).values).is_null()
                && (((*in_0).values).is_null()
                    || (*(*in_0).values).kind as libc::c_uint
                        == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*(*in_0).values).data.pair.first
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !((*in_0).values).is_null()
                    && (((*in_0).values).is_null()
                        || (*(*in_0).values).kind as libc::c_uint
                            == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*(*in_0).values).data.pair.first
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .first
        } else {
            core::ptr::null_mut() as *mut tl_object
        };
        let ref mut fresh161 = tl_values_pop_into!(in, callex);
        *fresh161 = if !((*in_0).values).is_null()
            && (((*in_0).values).is_null()
                || (*(*in_0).values).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
        {
            (*(*in_0).values).data.pair.next
        } else {
            core::ptr::null_mut() as *mut tl_object
        };
        if len == TL_APPLY_DROP_EVAL as libc::c_long {
            return TL_RESULT_AGAIN;
        }
        if len == TL_APPLY_PUSH_EVAL as libc::c_long {
            let ref mut fresh162 = tl_values_push!(in_0, callex);
            *fresh162 = tl_values_push!(in_0, callex);
            return TL_RESULT_AGAIN;
        }
        if tl_is_callable!(callex) {
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
            args = tl_new_pair(in_0, tl_first!(in_0 -> values), args);
            let ref mut fresh164 = (*in_0).values;
            *fresh164 = tl_next!(in_0 -> values);
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
                    core::ptr::null_mut() as *mut tl_object
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
                        core::ptr::null_mut() as *mut tl_object
                    });
                    arg = (if !l_arg.is_null()
                        && (l_arg.is_null()
                            || (*l_arg).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_arg).data.pair.first
                    } else {
                        core::ptr::null_mut() as *mut tl_object
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
                    0 as *mut tl_object
                })
                .is_null()
                    && ((if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object
                    })
                    .is_null()
                        || (*(if !args.is_null()
                            && (args.is_null()
                                || (*args).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*args).data.pair.first
                        } else {
                            0 as *mut tl_object
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
                        0 as *mut tl_object
                    }))
                    .data
                    .pair
                    .next
                } else {
                    core::ptr::null_mut() as *mut tl_object
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
                            0 as *mut tl_object
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object
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
                                0 as *mut tl_object
                            })
                            .data
                            .pair
                            .first
                        } else {
                            core::ptr::null_mut() as *mut tl_object
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
                                0 as *mut tl_object
                            })
                            .is_null()
                                && ((if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object
                                })
                                .is_null()
                                    || (*(if !args.is_null()
                                        && (args.is_null()
                                            || (*args).kind as libc::c_uint
                                                == TL_PAIR as libc::c_int as libc::c_uint)
                                    {
                                        (*args).data.pair.first
                                    } else {
                                        0 as *mut tl_object
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
                                    0 as *mut tl_object
                                })
                                .data
                                .pair
                                .first
                            } else {
                                0 as *mut tl_object
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
        mut then:
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        
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
        mut then: 
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        
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
            0 as *mut tl_object
        })
        .is_null()
            && ((if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).data.pair.first
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            }))
            .data
            .pair
            .first
        } else {
            0 as *mut tl_object
        })
        .is_null()
            && ((if !(if !state.is_null()
                && (state.is_null()
                    || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*state).data.pair.first
            } else {
                0 as *mut tl_object
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                }))
                .data
                .pair
                .first
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !(if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    && ((if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object
                    })
                    .is_null()
                        || (*(if !state.is_null()
                            && (state.is_null()
                                || (*state).kind as libc::c_uint
                                    == TL_PAIR as libc::c_int as libc::c_uint)
                        {
                            (*state).data.pair.first
                        } else {
                            0 as *mut tl_object
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
                        0 as *mut tl_object
                    }))
                    .data
                    .pair
                    .first
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                })
                .data
                .pair
                .first
            } else {
                0 as *mut tl_object
            })
            .data
            .pair
            .first
        } else {
            core::ptr::null_mut() as *mut tl_object
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
                0 as *mut tl_object
            })
            .is_null()
                && ((if !state.is_null()
                    && (state.is_null()
                        || (*state).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*state).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !state.is_null()
                        && (state.is_null()
                            || (*state).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*state).data.pair.first
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                })
                .data
                .pair
                .next
            } else {
                core::ptr::null_mut() as *mut tl_object
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
                0 as *mut tl_object
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                }))
                .data
                .pair
                .next
            } else {
                core::ptr::null_mut() as *mut tl_object
            }) == (*in_0).true_
            {
                tl_eval_and_then!(in_0, tl_first!(tl_first!(args)), new_state, _tl_eval_all_args_k);
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
                            0 as *mut tl_object
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object
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
                                0 as *mut tl_object
                            })
                            .data
                            .pair
                            .first
                        } else {
                            0 as *mut tl_object
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
                core::ptr::null_mut() as *mut tl_object
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
                    core::ptr::null_mut() as *mut tl_object
                });
                elem = (if !l_elem.is_null()
                    && (l_elem.is_null()
                        || (*l_elem).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*l_elem).data.pair.first
                } else {
                    core::ptr::null_mut() as *mut tl_object
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
                0 as *mut tl_object
            })
            .is_null()
                && ((if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object
                })
                .is_null()
                    || (*(if !args.is_null()
                        && (args.is_null()
                            || (*args).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*args).data.pair.first
                    } else {
                        0 as *mut tl_object
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
                    0 as *mut tl_object
                }))
                .data
                .pair
                .next
            } else {
                core::ptr::null_mut() as *mut tl_object
            }) == (*in_0).true_
            {
                tl_eval_and_then!(in_0, tl_first!(tl_first!(args)), state, _tl_eval_all_args_k);
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
                            0 as *mut tl_object
                        })
                        .is_null()
                            && ((if !args.is_null()
                                && (args.is_null()
                                    || (*args).kind as libc::c_uint
                                        == TL_PAIR as libc::c_int as libc::c_uint)
                            {
                                (*args).data.pair.first
                            } else {
                                0 as *mut tl_object
                            })
                            .is_null()
                                || (*(if !args.is_null()
                                    && (args.is_null()
                                        || (*args).kind as libc::c_uint
                                            == TL_PAIR as libc::c_int as libc::c_uint)
                                {
                                    (*args).data.pair.first
                                } else {
                                    0 as *mut tl_object
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
                                0 as *mut tl_object
                            })
                            .data
                            .pair
                            .first
                        } else {
                            0 as *mut tl_object
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
    use super::env_c::tl_env_get_kv;
    use super::object_c::{
        tl_gc, tl_list_len, tl_list_rvs, tl_new_int, tl_new_pair, tl_new_sym, tl_new_then,
    };
    use libc::{size_t};
    use crate::tl::{
        tl_interp, tl_object, tl_object, ObjectTag, TL_APPLY_DROP, TL_APPLY_DROP_EVAL,
        TL_APPLY_DROP_RESCUE, TL_APPLY_GETCHAR, TL_APPLY_INDIRECT, TL_APPLY_PUSH_EVAL,
        TL_EMPTY_LIST, TL_INT, TL_PAIR, TL_RESULT_AGAIN, TL_RESULT_DONE, TL_RESULT_GETCHAR, TL_SYM,
        TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tl/interp.c:5"]
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
            return core::ptr::null_mut() as *mut libc::c_void;
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
        *fresh180 = core::ptr::null_mut() as *mut tl_object;
        let ref mut fresh181 = (*in_0).true_;
        *fresh181 = tl_new_sym(in_0, b"tl-#t\0" as *const u8 as *const libc::c_char);
        let ref mut fresh182 = (*in_0).false_;
        *fresh182 = tl_new_sym(in_0, b"tl-#f\0" as *const u8 as *const libc::c_char);
        let ref mut fresh183 = (*in_0).error;
        *fresh183 = core::ptr::null_mut() as *mut tl_object;
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
        *fresh189 = core::ptr::null_mut() as *mut libc::c_char;
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
    use libc::{size_t};
    use libc::{getchar, putchar};
    use libc::{free, realloc};
    use crate::tl::{
        tl_init_ent, tl_interp, tl_interp_s, tl_ns, tl_object, TL_DEFAULT_GC_EVENTS, TL_EMPTY_LIST,
    };
    extern "C" {
        #[c2rust::src_loc = "3:20"]
        pub static mut __start_tl_init_ents: tl_init_ent;
        #[c2rust::src_loc = "3:42"]
        pub static mut __stop_tl_init_ents: tl_init_ent;
    }
}

#[c2rust::header_src = "/home/ember/src/tl/main.c:6"]
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
        tl_eval_and_then!(in_0, expr, std::ptr::null_mut(), _main_k);
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
                0 as *mut tl_object
            })
            .is_null()
                && (*(if !args.is_null()
                    && (args.is_null()
                        || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*args).data.pair.first
                } else {
                    0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && ((if !cont.is_null()
                && (cont.is_null()
                    || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*cont).data.pair.next
            } else {
                0 as *mut tl_object
            })
            .is_null()
                || (*(if !cont.is_null()
                    && (cont.is_null()
                        || (*cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
                {
                    (*cont).data.pair.next
                } else {
                    0 as *mut tl_object
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
                0 as *mut tl_object
            })
            .data
            .pair
            .first
        } else {
            std::ptr::null_mut() as *mut tl_object
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
            std::ptr::null_mut() as *mut tl_object
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
                std::ptr::null_mut() as *mut tl_object
            });
            cont = (if !l_cont.is_null()
                && (l_cont.is_null()
                    || (*l_cont).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_cont).data.pair.first
            } else {
                std::ptr::null_mut() as *mut tl_object
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
                    std::ptr::null_mut() as *mut tl_object
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
                        std::ptr::null_mut() as *mut tl_object
                    });
                    frm = (if !l_frm.is_null()
                        && (l_frm.is_null()
                            || (*l_frm).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_frm).data.pair.first
                    } else {
                        std::ptr::null_mut() as *mut tl_object
                    });
                }
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                let ref mut fresh202 = tl_error_clear!(& in_0);
                *fresh202 = std::ptr::null_mut() as *mut tl_object;
            }
            in_0.conts = TL_EMPTY_LIST as *mut tl_object;
            in_0.values = TL_EMPTY_LIST as *mut tl_object;
            tl_gc(&mut in_0);
        }
        return 0;
    }
    use libc::{RTLD_GLOBAL, RTLD_NOW};
    use libc::{dlerror, dlopen, dlsym};
    use super::eval_c::tl_run_until_done;
    use super::interp_c::{tl_interp_cleanup, tl_interp_init};
    use super::object_c::{tl_gc, tl_new_int, tl_new_pair, tl_new_sym};
    use super::print_c::{tl_print, tl_printf};
    use libc::{size_t};
    use libc::{fflush, fprintf};
    use libc::exit;
    use crate::tl::{
        tl_init_ent, tl_interp, tl_interp_s, tl_name, tl_ns, tl_object, tl_object,
        ObjectTag, TL_EMPTY_LIST, TL_FMASK, TL_F_MARK, TL_INT, TL_PAIR,
    };
    use libc::{isatty, STDIN_FILENO};
}

#[c2rust::header_src = "/home/ember/src/tl/ns.c:7"]
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
        *fresh212 = std::ptr::null_mut() as *mut tl_child;
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
        *fresh214 = std::ptr::null_mut() as *mut libc::c_char;
        (*(*ns).root).here.len = 0 as libc::c_int as size_t;
        let ref mut fresh215 = (*(*ns).root).sz_children;
        *fresh215 = 0 as libc::c_int as size_t;
        (*(*ns).root).num_children = *fresh215;
        let ref mut fresh216 = (*(*ns).root).children;
        *fresh216 = std::ptr::null_mut() as *mut tl_child;
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
        let ref mut fresh217 = (*cur).chain_0;
        *fresh217 = std::ptr::null_mut() as *mut tl_name_s;
        while !cur.is_null() {
            index = 0 as libc::c_in_0t as size_t;
            while index < (*cur).num_children {
                child = &mut *((*cur).children).offset(index as isize) as *mut tl_child;
                tl_alloc_free!(in_0, child -> seg.data).expect("non-null function poin_0ter")(
                    tl_alloc_free!(in_0, child -> seg.data),
                    tl_alloc_free!(in_0, child -> seg.data),
                    tl_alloc_free!(in_0, child -> seg.data),
                );
                let ref mut fresh218 = (*(*child).name).chain_0;
                *fresh218 = (*cur).chain_0;
                let ref mut fresh219 = (*cur).chain_0;
                *fresh219 = (*child).name;
                index = index.wrappin_0g_add(1);
            }
            tl_alloc_free!(in_0, cur -> children).expect("non-null function poin_0ter")(
                tl_alloc_free!(in_0, cur -> children),
                tl_alloc_free!(in_0, cur -> children),
                tl_alloc_free!(in_0, cur -> children),
            );
            tl_alloc_free!(in_0, cur -> here.data).expect("non-null function poin_0ter")(
                tl_alloc_free!(in_0, cur -> here.data),
                tl_alloc_free!(in_0, cur -> here.data),
                tl_alloc_free!(in_0, cur -> here.data),
            );
            temp = cur;
            cur = (*cur).chain_0;
            tl_alloc_free!(in_0, temp).expect("non-null function poin_0ter")(
                tl_alloc_free!(in_0, temp),
                tl_alloc_free!(in_0, temp),
                tl_alloc_free!(in_0, temp),
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
            tl_printf(in_0, b"[std::ptr::null_mut()]\n\0" as *const u8 as *const libc::c_char);
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
            tl_printf(in_0, b" <std::ptr::null_mut()>\n\0" as *const u8 as *const libc::c_char);
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
        *fresh220 = std::ptr::null_mut() as *mut tl_name_s;
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
                    0 as *mut tl_object
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
    use libc::{size_t};
    use libc::{memcmp, memcpy, memmove};
    use crate::tl::{
        tl_buffer, tl_child, tl_init_ent, tl_interp, tl_name, tl_name_s, tl_ns, tl_object,
        tl_object, ObjectTag, TL_EMPTY_LIST, TL_PAIR,
    };
}
#[c2rust::header_src = "/home/ember/src/tl/object.c:8"]
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
        *fresh227 = std::ptr::null_mut() as *mut tl_object;
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
                std::ptr::null_mut() as *const libc::c_char,
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
        mut cfunc: 
            unsafe extern "C" fn(*mut tl_interp, *mut tl_object, *mut tl_object) -> (),
        
        mut state: *mut tl_object,
        mut name: *const libc::c_char,
    ) -> *mut tl_object {
        let mut obj = tl_new(in_0);
        (*obj).kind = TL_THEN;
        let ref mut fresh233 = (*obj).data.then.cfunc;
        *fresh233 = Some(cfunc);
        let ref mut fresh234 = (*obj).data.then.state;
        *fresh234 = state;
        let ref mut fresh235 = (*obj).data.then.name;
        *fresh235 = if !name.is_null() {
            tl_strdup(in_0, name)
        } else {
            std::ptr::null_mut() as *mut libc::c_char
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
            std::ptr::null_mut() as *mut tl_object
        };
        while !l_item.is_null() {
            cnt = cnt.wrapping_add(1);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                std::ptr::null_mut() as *mut tl_object
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
            } else {
                std::ptr::null_mut() as *mut tl_object
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
            std::ptr::null_mut() as *mut tl_object
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                std::ptr::null_mut() as *mut tl_object
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
            } else {
                std::ptr::null_mut() as *mut tl_object
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
            std::ptr::null_mut() as *mut tl_object
        };
        while !l_item.is_null() {
            res = tl_new_pair(in_0, item, res);
            l_item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.next
            } else {
                std::ptr::null_mut() as *mut tl_object
            });
            item = (if !l_item.is_null()
                && (l_item.is_null()
                    || (*l_item).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*l_item).data.pair.first
            } else {
                std::ptr::null_mut() as *mut tl_object
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
            return std::ptr::null_mut() as *mut libc::c_char;
        }
        s = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        if s == 0 {
            return std::ptr::null_mut() as *mut libc::c_char;
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
            return std::ptr::null_mut() as *mut libc::c_void;
        }
        return memset(region, 0 as libc::c_int, n.wrapping_mul(s));
    }
    use super::ns_c::tl_ns_resolve;
    use libc::{size_t};
    use libc::{memset, strcpy, strlen};
    use crate::tl::{
        tl_buffer, tl_interp, tl_name, tl_ns, tl_object, tl_object, ObjectTag, TL_CFUNC,
        TL_CFUNC_BYVAL, TL_CONT, TL_EMPTY_LIST, TL_FMASK, TL_FUNC, TL_F_MARK, TL_INT, TL_MACRO,
        TL_PAIR, TL_SYM, TL_THEN,
    };
}
#[c2rust::header_src = "/home/ember/src/tl/read.c:10"]
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
                            0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
                *fresh262 = std::ptr::null_mut() as *mut libc::c_char;
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
                *fresh285 = std::ptr::null_mut() as *mut libc::c_char;
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
            0 as *mut tl_object
        })
        .is_null()
            && (*(if !args.is_null()
                && (args.is_null()
                    || (*args).kind as libc::c_uint == TL_PAIR as libc::c_int as libc::c_uint)
            {
                (*args).data.pair.first
            } else {
                0 as *mut tl_object
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
                        0 as *mut tl_object
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
                        0 as *mut tl_object
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
                    std::ptr::null_mut() as *mut tl_object
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
                        std::ptr::null_mut() as *mut tl_object
                    });
                    kv = (if !l_kv.is_null()
                        && (l_kv.is_null()
                            || (*l_kv).kind as libc::c_uint
                                == TL_PAIR as libc::c_int as libc::c_uint)
                    {
                        (*l_kv).data.pair.first
                    } else {
                        std::ptr::null_mut() as *mut tl_object
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
    use super::eval_c::tl_push_apply;
    use super::object_c::{
        tl_list_rvs, tl_list_rvs_improp, tl_new_int, tl_new_pair, tl_new_sym, tl_new_sym_data,
        tl_new_then,
    };
    use libc::{size_t};
    use libc::EOF;
    use crate::tl::{tl_interp, tl_object, tl_object, ObjectTag, TL_INT, TL_PAIR};
}

pub use super::builtins::{
    _tl_cf_define_k, _tl_cf_if_k, _tl_cf_set_k, _tl_readc_k,
    tl_cf_add, tl_cf_all_objects, tl_cf_apply,
    tl_cf_call_with_current_continuation, tl_cf_car, tl_cf_cdr, tl_cf_chr, tl_cf_concat,
    tl_cf_cons, tl_cf_define, tl_cf_display, tl_cf_display_sep, tl_cf_div, tl_cf_env, tl_cf_eq,
    tl_cf_error, tl_cf_evalin, tl_cf_gc, tl_cf_if, tl_cf_lambda, tl_cf_length, tl_cf_less,
    tl_cf_load_mod, tl_cf_macro, tl_cf_mod, tl_cf_mul, tl_cf_nand, tl_cf_null, tl_cf_ord,
    tl_cf_prefix, tl_cf_putbackc, tl_cf_read, tl_cf_readc, tl_cf_rescue, tl_cf_set, tl_cf_setenv,
    tl_cf_sub, tl_cf_substr, tl_cf_topenv, tl_cf_type, tl_cf_writec,
};
pub use self::debug_c::{
    _indent, _tl_cf_debug_print_k, init_tl_cf_debug_print, tl_cf_debug_print, tl_dbg_print,
};
pub use self::env_c::{tl_env_get_kv, tl_env_set_global, tl_env_set_local, tl_frm_set};
pub use self::eval_c::{
    _tl_apply_next_body_callable_k, _tl_eval_all_args, _tl_eval_all_args_k, _tl_eval_and_then,
    _tl_getc_and_then, tl_apply_next, tl_push_apply, tl_push_eval, tl_run_until_done,
};
pub use self::interp_c::{
    __start_tl_init_ents, __stop_tl_init_ents, _modloadf, _readf, _reallocf, _writef,
    tl_interp_cleanup, tl_interp_init, tl_interp_init_alloc,
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
 use libc::{size_t, snprintf};

 pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}