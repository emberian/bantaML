use proc_macro::{TokenTree,TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::Ident;
#[proc_macro_attribute]
pub fn tl_cf(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut inp_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let fnname = inp_fn.sig.ident.to_string();
    let init_name = Ident::new(&format!("init_{}", fnname), Span::call_site());
    inp_fn.sig.abi = Some(syn::Abi { extern_token: Default::default(), name: Some(syn::LitStr::new("C", Span::call_site())) });
    let tl_name = format!("tl-{}\0", fnname.strip_prefix("tl_cf_").expect("tl_cf function name must start with tl_cf_"));
    
    let flags = if !attr.is_empty() { quote!{crate::tl::TL_EF_BYVAL} } else { quote!{0}} ;
    let fnid = inp_fn.sig.ident.clone();
    let us = inp_fn.sig.unsafety;
    let r = quote::quote! {
        #[link_section = "tl_init_ents"]
        #[used]
        static mut #init_name: (#us extern fn (*mut crate::tl::tl_interp, *mut tl_object, *mut crate::tl::tl_object) , *const u8, libc::size_t) = (#fnid, #tl_name.as_ptr(), #flags);

        #inp_fn
    }.into();
    r
}