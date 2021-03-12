#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use regex::Regex;

use crate::proc_macro::TokenStream;

//
// Angular values
//

#[proc_macro_derive(AngleMapper)]
pub fn angle_mapper_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse2(input.into()).unwrap();

    let name = &ast.ident;
    let count = parse_angle_format(&name.to_string())
        .filter(|value| *value != "arc")
        .count();

    match count {
        1 => (quote! {
            impl AngleMapper for #name {
                type Item = SimpleAngle;
            }
        }).into(),

        2 => (quote! {
            impl AngleMapper for #name {
                type Item = ShortAngle;
            }
        }).into(),

        3 => (quote! {
            impl AngleMapper for #name {
                type Item = LongAngle;
            }
        }).into(),

        _ => panic!("Illegal angle format name")
    }
}


fn parse_angle_format<'a>(name: &'a String) -> Box<dyn Iterator<Item=String> + 'a> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"([A-Z][a-z]+)"#).unwrap();
    }

    Box::new(
        RE.find_iter(name)
            .map(|mat| String::from(mat.as_str()).to_lowercase())
    )
}
