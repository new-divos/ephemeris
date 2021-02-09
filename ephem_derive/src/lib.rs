#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use crate::proc_macro::TokenStream;

//
// Angular values
//

#[proc_macro_derive(AngleOrd)]
pub fn angle_ord_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    (quote! {
        impl ::std::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    }).into()
}


#[proc_macro_derive(AngleValue)]
pub fn angle_value_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    (quote! {
        impl ::std::convert::Into<f64> for #name {
            fn into(self) -> f64 {
                self.value()
            }
        }
    }).into()
}


#[proc_macro_derive(UnpackAngleValue)]
pub fn unpack_angle_value_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    (quote! {
        impl ::std::convert::Into<(Sign, f64)> for #name {
            fn into(self) -> (Sign, f64) {
                (self.sign(), self.0.abs())
            }
        }
    }).into()
}


macro_rules! impl_angular_conv {
    ($ast:expr, $t:ty) => {{
        let name = &$ast.ident;

        (quote! {
            impl ::std::convert::Into<$t> for #name {
                fn into(self) -> $t {
                    self.0.into()
                }
            }
        }).into()
    }};
}


#[proc_macro_derive(UnpackShortAngle)]
pub fn unpack_short_angle_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_angular_conv!(ast, (Sign, i32, f64))
}


#[proc_macro_derive(RawShortAngle)]
pub fn raw_short_angle_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_angular_conv!(ast, (i32, f64))
}


#[proc_macro_derive(UnpackLongAngle)]
pub fn unpack_long_angle_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_angular_conv!(ast, (Sign, i32, i32, f64))
}


#[proc_macro_derive(RawLongAngle)]
pub fn raw_long_angle_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_angular_conv!(ast, (i32, i32, f64))
}
