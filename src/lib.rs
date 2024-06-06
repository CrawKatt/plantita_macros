use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(FromStr, attributes(str))]
pub fn derive_from_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = if let Data::Enum(data) = &input.data {
        &data.variants
    } else {
        panic!("FromStr can only be derived for enums");
    };

    let from_str_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let str_attr = variant.attrs.iter().find_map(|attr| {
            if attr.path.is_ident("str") {
                Some(attr.parse_args::<syn::LitStr>().unwrap())
            } else {
                None
            }
        }).expect("Each variant must have a #[str = \"...\"] attribute");

        quote! {
            #str_attr => #name::#variant_ident,
        }
    });

    let expanded = quote! {
        impl std::convert::From<&str> for #name {
            fn from(s: &str) -> Self {
                match s {
                    #(#from_str_arms)*
                    _ => #name::Unknown,
                }
            }
        }
    };

    TokenStream::from(expanded)
}