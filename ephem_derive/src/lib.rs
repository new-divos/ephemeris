#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate syn;

use regex::Regex;

use crate::proc_macro::TokenStream;

//
// Angular values
//

struct AngleSignature {
    name: syn::Ident
}

impl syn::parse::Parse for AngleSignature {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            panic!("Write full angle item signature.");
        }

        Ok(AngleSignature {
            name: input.parse().unwrap()
        })
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

        _ => panic!("Illegal angle item name.")
    }
}


#[proc_macro]
pub fn angle_serialize(input: TokenStream) -> TokenStream {
    let signature = parse_macro_input!(input as AngleSignature);

    let name = &signature.name;
    let units: Vec<String> = parse_angle_format(&name.to_string())
        .collect();

    let arc_pos = units.iter()
        .position(|value| *value == "arc")
        .unwrap_or(usize::max_value());
    let keys: Vec<String> = units.iter()
        .enumerate()
        .map(|(idx, value)| {
            if idx <= arc_pos {
                value.to_owned()
            } else {
                let mut key = String::from("arc_");
                key.push_str(value.as_str());

                key
            }
        })
        .filter(|value| *value != "arc")
        .collect();

    match keys.len() {
        1 => {
            let key = keys[0].to_owned();

            (quote! {
                impl Serialize for Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: Serializer,
                    {
                        let mut state = serializer.serialize_struct("Angle", 1)?;
                        state.serialize_field(#key, &self.0.0)?;
                        state.end()
                    }
                }
            }).into()
        },

        2 => {
            let key1 = keys[0].to_owned();
            let key2 = keys[1].to_owned();

            (quote! {
                impl Serialize for Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: Serializer,
                    {
                        let mut state = serializer.serialize_struct("Angle", 2)?;
                        state.serialize_field(#key1, &self.0.0)?;
                        state.serialize_field(#key2, &self.0.1)?;
                        state.end()
                    }
                }
            }).into()
        },

        3 => {
            let key1 = keys[0].to_owned();
            let key2 = keys[1].to_owned();
            let key3 = keys[2].to_owned();

            (quote! {
                impl Serialize for Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: Serializer,
                    {
                        let mut state = serializer.serialize_struct("Angle", 3)?;
                        state.serialize_field(#key1, &self.0.0)?;
                        state.serialize_field(#key2, &(self.0.1 as i32))?;
                        state.serialize_field(#key3, &self.0.2)?;
                        state.end()
                    }
                }
            }).into()
        },

        _ => panic!("Illegal angle item name.")
    }
}

