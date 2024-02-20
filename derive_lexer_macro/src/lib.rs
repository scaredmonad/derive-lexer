use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Lit, Meta, NestedMeta, Variant};

#[proc_macro_derive(Tokenize, attributes(lex))]
pub fn tokenize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!("Tokenize can only be derived for enums");
    };

    let match_arms = variants.into_iter().map(
        |Variant {
             ident: variant_name,
             attrs,
             ..
         }| {
            let pattern = attrs
                .into_iter()
                .find_map(|attr| {
                    if let Meta::List(meta) = attr.parse_meta().expect("Error parsing meta") {
                        if meta.path.is_ident("lex") {
                            if let Some(NestedMeta::Meta(Meta::NameValue(meta_nv))) =
                                meta.nested.first()
                            {
                                if let Lit::Str(lit) = &meta_nv.lit {
                                    return Some(lit.value());
                                }
                            }
                        }
                    }
                    None
                })
                .expect("Pattern not found");

            quote! {
                if input.starts_with(#pattern) {
                    return Some((Self::#variant_name, &input[#pattern.len()..]));
                }
            }
        },
    );

    let expanded = quote! {
        impl Tokenize for #enum_name {
            fn tokenize(input: &str) -> Option<(Self, &str)> {
                #(#match_arms)*
                None
            }
        }
    };

    TokenStream::from(expanded)
}
