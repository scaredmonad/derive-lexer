use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_derive(Tokenize, attributes(lex))]
pub fn tokenize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;
    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("Tokenize can only be derived for enums");
    };

    let mut patterns = Vec::new();
    let mut variant_names = Vec::new();

    for variant in variants {
        let variant_name = variant.ident;
        let pattern = variant
            .attrs
            .into_iter()
            .find_map(|attr| {
                if let Ok(Meta::List(meta)) = attr.parse_meta() {
                    if meta.path.is_ident("lex") {
                        if let Some(NestedMeta::Meta(Meta::NameValue(mnv))) = meta.nested.first() {
                            if let Lit::Str(lit_str) = &mnv.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
                None
            })
            .expect("Pattern not found for variant");

        patterns.push(pattern);
        variant_names.push(variant_name);
    }

    let match_arms =
        patterns
            .into_iter()
            .zip(variant_names.into_iter())
            .map(|(pattern, variant_name)| {
                quote! {
                    if let Some(mat) = regex::Regex::new(#pattern).unwrap().find(input) {
                        return Some((Self::#variant_name, &input[mat.end()..]));
                    }
                }
            });

    let gen = quote! {
        impl Tokenize for #enum_name {
            fn tokenize(input: &str) -> Option<(Self, &str)> {
                #(#match_arms)*
                None
            }
        }
    };

    gen.into()
}
