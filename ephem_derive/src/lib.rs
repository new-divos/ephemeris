#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate syn;

use regex::Regex;

use crate::proc_macro::TokenStream;
use proc_macro2::Span;

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
            impl crate::base::angle::AngleMapper for #name {
                type Item = SimpleAngle;
            }
        }).into(),

        2 => (quote! {
            impl crate::base::angle::AngleMapper for #name {
                type Item = ShortAngle;
            }
        }).into(),

        3 => (quote! {
            impl crate::base::angle::AngleMapper for #name {
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
    let type_name = name.to_string();
    let units: Vec<String> = parse_angle_format(&type_name)
        .collect();
    let struct_name = format!("Angle{}", type_name);

    let arc_pos = units.iter()
        .position(|value| *value == "arc")
        .unwrap_or(usize::max_value());
    let keys: Vec<String> = units.iter()
        .enumerate()
        .map(|(idx, value)| {
            if idx <= arc_pos {
                value.to_owned()
            } else {
                format!("arc_{}", value)
            }
        })
        .filter(|value| *value != "arc")
        .collect();

    match keys.len() {
        1 => {
            let key = keys[0].to_owned();

            (quote! {
                impl serde::ser::Serialize for crate::base::angle::Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: serde::ser::Serializer,
                    {
                        use serde::ser::SerializeStruct;

                        let mut state = serializer.serialize_struct(#struct_name, 1)?;
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
                impl serde::ser::Serialize for crate::base::angle::Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: serde::ser::Serializer,
                    {
                        use serde::ser::SerializeStruct;

                        let mut state = serializer.serialize_struct(#struct_name, 2)?;
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
                impl serde::ser::Serialize for crate::base::angle::Angle<#name> {
                    fn serialize<S>(&self, serializer: S) ->
                            std::result::Result<S::Ok, S::Error>
                        where
                            S: serde::ser::Serializer,
                    {
                        use serde::ser::SerializeStruct;

                        let mut state = serializer.serialize_struct(#struct_name, 3)?;
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


#[proc_macro]
pub fn angle_deserialize(input: TokenStream) -> TokenStream {
    let signature = parse_macro_input!(input as AngleSignature);

    let name = &signature.name;
    let type_name = name.to_string();
    let units: Vec<String> = parse_angle_format(&name.to_string())
        .collect();
    let visitor_name = format!("Visitor{}", type_name);
    let struct_name = format!("Angle{}", type_name);

    let visitor = syn::Ident::new(
        visitor_name.as_str(),
        Span::mixed_site()
    );

    let arc_pos = units.iter()
        .position(|value| *value == "arc")
        .unwrap_or(usize::max_value());
    let keys: Vec<(String, String)> = units.iter()
        .enumerate()
        .map(|(idx, value)| {
            let item = {
                let mut c = value.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(c).collect()
                }
            };

            if idx <= arc_pos {
                (value.to_owned(), item)
            } else {
                (format!("arc_{}", value), format!("Arc{}", item))
            }
        })
        .filter(|value| (*value).0 != "arc")
        .collect();

    fn split(key: &(String, String)) -> (String, syn::Ident) {
        (
            (*key).0.to_owned(),
            syn::Ident::new((*key).1.as_str(), Span::mixed_site())
        )
    }

    match keys.len() {
        1 => {
            let (key, item) = split(&keys[0]);

            (quote! {
                impl<'de> serde::de::Deserialize<'de> for crate::base::angle::Angle<#name> {
                    fn deserialize<D>(deserializer: D)
                        -> std::result::Result<Self, D::Error>
                    where
                        D: serde::de::Deserializer<'de>,
                    {
                        use std::fmt;

                        enum Field { #item, }

                        impl<'de> serde::de::Deserialize<'de> for Field {
                            fn deserialize<D>(deserializer: D)
                                -> std::result::Result<Field, D::Error>
                            where
                                D: serde::de::Deserializer<'de>,
                            {
                                struct FieldVisitor;

                                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                                    type Value = Field;

                                    fn expecting(
                                        &self,
                                        formatter: &mut fmt::Formatter
                                    ) -> fmt::Result
                                    {
                                        let parts = vec!["`", #key, "`"];
                                        formatter.write_str(parts.join("").as_str())
                                    }

                                    fn visit_str<E>(self, value: &str)
                                        -> std::result::Result<Field, E>
                                    where
                                        E: serde::de::Error,
                                    {
                                        match value {
                                            #key => Ok(Field::#item),
                                            _ => Err(
                                                serde::de::Error::unknown_field(value, FIELDS)
                                            ),
                                        }
                                    }
                                }

                                deserializer.deserialize_identifier(FieldVisitor)
                            }
                        }

                        struct #visitor;

                        impl<'de> serde::de::Visitor<'de> for #visitor {
                            type Value = crate::base::angle::Angle<#name>;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                let parts = vec!["struct Angle<", #type_name, ">"];
                                formatter.write_str(parts.join("").as_str())
                            }

                            fn visit_seq<V>(self, mut seq: V)
                                -> std::result::Result<crate::base::angle::Angle<#name>, V::Error>
                            where
                                V: serde::de::SeqAccess<'de>,
                            {
                                let value = seq.next_element()?
                                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                                Ok(crate::base::angle::Angle::<#name>::new(value))
                            }

                            fn visit_map<V>(self, mut map: V)
                                -> std::result::Result<crate::base::angle::Angle<#name>, V::Error>
                            where
                                V: serde::de::MapAccess<'de>,
                            {
                                let mut value = None;
                                while let Some(key) = map.next_key()? {
                                    if let Field::#item = key {
                                        if value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(#key));
                                        }
                                        value = Some(map.next_value()?);
                                    }
                                }
                                let value = value.ok_or_else(
                                    || serde::de::Error::missing_field(#key)
                                )?;
                                Ok(crate::base::angle::Angle::<#name>::new(value))
                            }
                        }

                        const FIELDS: &'static [&'static str] = &[#key];
                        deserializer.deserialize_struct(#struct_name, FIELDS, #visitor)
                    }
                }
            }).into()
        },

        2 => {
            (quote! {}).into()
        },

        3 => {
            (quote! {}).into()
        },

        _ => panic!("Illegal angle item name.")
    }
}