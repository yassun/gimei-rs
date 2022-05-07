extern crate proc_macro;

use darling::{ast, FromDeriveInput, FromField, FromVariant};
use quote::quote;

use syn::{Ident, Type};

#[allow(dead_code)]
#[derive(FromVariant)]
#[darling(from_ident, attributes(gimei))]
struct GimeiVariant {
    ident: Ident,
    fields: darling::ast::Fields<GimeiField>,
}

impl From<Ident> for GimeiVariant {
    fn from(ident: Ident) -> Self {
        GimeiVariant {
            ident,
            fields: darling::ast::Style::Unit.into(),
        }
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(gimei))]
struct GimeiField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    generator: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(gimei), supports(struct_any))]
struct Gimei {
    ident: Ident,
    data: ast::Data<GimeiVariant, GimeiField>,
}

#[proc_macro_derive(Gimei, attributes(gimei))]
pub fn derive_self_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed = syn::parse(input).expect("syn::parse error");
    let receiver = Gimei::from_derive_input(&parsed).expect("GimeiField::from_derive_input error");
    let receiver_name = &receiver.ident;
    let expanded = match receiver.data {
        darling::ast::Data::Struct(darling::ast::Fields {
            ref fields,
            ref style,
            ..
        }) => match style {
            ast::Style::Struct => {
                let struct_fields: Vec<_> = fields
                    .iter()
                    .filter_map(|f| {
                        if f.generator.is_some() {
                            Some(f.ident.as_ref().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();

                let let_statements: Vec<_> = fields
                    .iter()
                    .filter_map(|f| {
                        let field_name = f.ident.as_ref().unwrap();
                        let field_ty = &f.ty;
                        if let Some(ref expr) = f.generator {
                            let generator = syn::parse_str::<syn::Expr>(expr).unwrap();
                            Some(quote! {
                                let #field_name: #field_ty = #generator.clone();
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                let mut name_with_rng = false;
                let mut address_with_rng = false;

                let rng_with_statements: Vec<_> = fields
                    .iter()
                    .filter_map(|f| {
                        if let Some(ref expr) = f.generator {
                            if let Some(index) = expr.find("name_with_rng") {
                                if index == 0 && !name_with_rng {
                                    name_with_rng = true;
                                    return Some(quote! {
                                        let name_with_rng = Name::new_with_rand(rng);
                                    });
                                }
                            }
                            if let Some(index) = expr.find("address_with_rng") {
                                if index == 0 && !address_with_rng {
                                    address_with_rng = true;
                                    return Some(quote! {
                                        let address_with_rng = Address::new_with_rand(rng);
                                    });
                                }
                            }
                        }
                        None
                    })
                    .collect();

                let impl_dummy = quote! {
                    impl Dummy for #receiver_name {
                        fn with_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                            #(#rng_with_statements)*

                            #(#let_statements)*
                            #receiver_name {
                                #(#struct_fields),*
                                ,..Default::default()
                            }
                        }
                    }
                };
                impl_dummy
            }
            ast::Style::Tuple => {
                panic!("Tuple is not supported")
            }
            ast::Style::Unit => {
                panic!("Unit is not supported")
            }
        },
        ast::Data::Enum(_) => {
            panic!("Enum is not supported")
        }
    };

    expanded.into()
}
