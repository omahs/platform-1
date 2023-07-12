use crate::VersionAttributes;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Expr, Lit, LitInt, LitStr, Meta,
    Path, Type,
};

pub(super) fn derive_platform_serialize_struct(
    input: &DeriveInput,
    version_attributes: VersionAttributes,
    data_struct: &DataStruct,
    error_type: Path,
    name: &Ident,
) -> TokenStream {
    let VersionAttributes {
        platform_serialize_limit,
        platform_serialize_into,
        platform_version_path,
        allow_prepend_version,
        force_prepend_version,
        ..
    } = version_attributes;

    // Extract the generics.
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let serialize_into = match platform_serialize_into.clone() {
        Some(inner) => quote! {
            let inner: #inner = self.clone().into();
            bincode::encode_to_vec(inner, config)
        },
        None => quote! {
            bincode::encode_to_vec(self, config)
        },
    };

    let serialize_into_consume = match platform_serialize_into {
        Some(inner) => quote! {
            let inner: #inner = self.into();
            bincode::encode_to_vec(inner, config)
        },
        None => quote! {
            bincode::encode_to_vec(self, config)
        },
    };

    let expanded = if let Some(limit) = platform_serialize_limit {
        quote! {
            impl #impl_generics PlatformSerializable for #name #ty_generics #where_clause
            {
                fn serialize(&self) -> Result<Vec<u8>, #error_type> {
                    let config = config::standard().with_big_endian().with_limit::<{ #limit }>();
                    #serialize_into.map_err(|e| {
                    match e {
                        bincode::error::EncodeError::Io{inner, index} => #error_type::MaxEncodedBytesReachedError{max_size_kbytes: #limit, size_hit: index},
                        _ => #error_type::PlatformSerializationError(format!("unable to serialize {}: {}", stringify!(#name), e)),
                    }})
                }

                fn serialize_consume(self) -> Result<Vec<u8>, #error_type> {
                    let config = config::standard().with_big_endian().with_limit::<{ #limit }>();
                    #serialize_into_consume.map_err(|e| {
                    match e {
                        bincode::error::EncodeError::Io{inner, index} => #error_type::MaxEncodedBytesReachedError{max_size_kbytes: #limit, size_hit: index},
                        _ => #error_type::PlatformSerializationError(format!("unable to serialize {}: {}", stringify!(#name), e)),
                    }})
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics PlatformSerializable for #name #ty_generics #where_clause
            {
                fn serialize(&self) -> Result<Vec<u8>, #error_type> {
                    let config = config::standard().with_big_endian().with_no_limit();
                    #serialize_into.map_err(|e| {
                        #error_type::PlatformSerializationError(format!("unable to serialize {}: {}", stringify!(#name), e))
                    })
                }

                fn serialize_consume(self) -> Result<Vec<u8>, #error_type> {
                    let config = config::standard().with_big_endian().with_no_limit();
                    #serialize_into_consume.map_err(|e| {
                        #error_type::PlatformSerializationError(format!("unable to serialize {}: {}", stringify!(#name), e))
                    })
                }
            }
        }
    };

    TokenStream::from(expanded)
}