extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed,
    FieldsUnnamed,
};

/// Derive macro for the Message trait. It will implement the Message trait for the given type and
/// also implement Debug for the type. The Debug implementation will print the name of the type and
/// the values of all fields that are not marked with the `#[hidden]` attribute.
/// If a field is marked with the `#[obfuscated]` attribute, the value of the field will be replaced
/// with the string "<obfuscated>".
///
/// As in regular Debug derive scenarios, each field type must implement the Debug trait. You can
/// use Message derive on such field types to use the same obfuscation rules, or implement or derive
/// Debug manually.
///
/// # Example of a struct
/// Below is an example of a struct that implements the Message trait. The struct has a field that is
/// marked with the `#[obfuscated]` attribute. The Debug implementation will print the name of the
/// type and the value of the field. The value of the field will be replaced with the string
/// "<obfuscated>". Also, the types `rtype` attribute is set to `usize`, this will auto set the
/// `Result` type of the Message trait to `usize`.
/// ```rust,ignore
/// use protoactor::message::Message;
///
/// #[derive(Message)]
/// #[rtype(result = "usize")]
/// struct MyMessage {
///     #[obfuscated]
///     sensitive_field: Option<String>,
///     other_field: String,
///     #[hide]
///     unprintable_field: String,
/// }
/// ```
/// Generated code:
/// ```rust,ignore
/// impl protoactor::message::Message for MyMessage {
///    type Result = usize;
/// }
/// #[automatically_derived]
/// impl std::fmt::Debug for MyMessage {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("MyMessage")
///             .field("sensitive_field", &"<obfuscated>")
///             .field("other_field", AsDiagnostic(&self.other_field))
///             .finish()
///     }
/// }
/// ```
/// # Example of an enum
/// Below is an example of an enum that implements the Message trait. The enum has a field that is
/// marked with the `#[obfuscated]` attribute. The Debug implementation will print the value of the
/// field as "<obfuscated>". Also, the types `rtype` attribute is set to `usize`, this will auto set
/// the `Result` type of the Message trait to `usize`.
/// ```rust,ignore
/// use protoactor::message::Message;
///
/// #[derive(Message)]
/// #[rtype(result = "usize")]
/// enum MyEnumMessage {
///     TupleVariant(#[obfuscated] Option<String>, String),
///     StructVariant { #[obfuscated] sensitive_field: Option<String>, other_field: String },
///     UnitVariant,
/// }
/// ```
/// Generated code:
/// ```rust,ignore
/// impl protoactor::message::Message for MyEnumMessage {
///   type Result = usize;
/// }
/// #[automatically_derived]
/// impl std::fmt::Debug for MyEnumMessage {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             MyEnumMessage::TupleVariant(__binding_0, __binding_1) => f
///                 .debug_tuple("TupleVariant")
///                 .field(&"<obfuscated>")
///                 .field(&__binding_1)
///                 .finish(),
///             MyEnumMessage::StructVariant { sensitive_field, other_field } => f
///                 .debug_struct("StructVariant")
///                 .field("sensitive_field", &"<obfuscated>")
///                 .field("other_field", &other_field)
///                 .finish(),
///             MyEnumMessage::UnitVariant => ::core::fmt::Formatter::write_str(f, "UnitVariant"),
///         }
///     }
/// }
/// ```
/// # Example of a struct with a field that implements the Message trait
/// This example is similar to the first example, but the struct has a field that implements the
/// Message trait. In this example it is obvious that obfuscation will be applied in graph of types
/// as long as each type derives the Message trait and you mark fields that should be obfuscated with
/// the `#[obfuscated]` attribute.
/// ```rust,ignore
/// use protoactor::message::Message;
///
/// #[derive(Message)]
/// #[rtype(result = "usize")]
/// struct MyMessage {
///     #[obfuscated]
///     sensitive_field: Option<String>,
///     other_field: String,
///     inner_message: InnerMessage,
/// }
///
/// #[derive(Message)]
/// #[rtype(result = "usize")]
/// struct InnerMessage {
///     #[obfuscated]
///     sensitive_field: Option<String>,
///     other_field: String,
/// }
///
/// ```
/// Generated code:
/// ```rust,ignore
/// // MyMessage
/// impl protoactor::message::Message for MyMessage {
///    type Result = usize;
/// }
/// #[automatically_derived]
/// impl std::fmt::Debug for MyMessage {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("MyMessage")
///             .field("sensitive_field", &"<obfuscated>")
///             .field("other_field", &self.other_field)
///             .field("inner_message", &self.inner_message)
///             .finish()
///     }
/// }
///
/// // InnerMessage
/// impl protoactor::message::Message for InnerMessage {
///    type Result = ();
/// }
/// #[automatically_derived]
/// impl std::fmt::Debug for InnerMessage {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("InnerMessage")
///             .field("sensitive_field", &"<obfuscated>")
///             .field("other_field", &self.other_field)
///             .finish()
///     }
/// }
/// ```
/// # Final notes
/// If you want to implement Debug manually, then simply don't use the `#[derive(Message)]` attribute.
/// Instead, implement the Message trait manually, is is few lines of code.
#[proc_macro_derive(Message, attributes(rtype, obfuscated, hidden))]
pub fn message_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = impl_message(&input);
    TokenStream::from(gen)
}

fn impl_message(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let result_type = get_rtype(&ast.attrs);
    let debug_impl = match &ast.data {
        Data::Struct(ref data) => impl_debug_for_struct(name, data),
        Data::Enum(ref data) => impl_debug_for_enum(name, data),
        Data::Union(_) => {
            // Unions are not supported for this macro
            return quote!();
        }
    };

    quote! {
        #[automatically_derived]
        impl protoactor::message::Message for #name {
            type Result = #result_type;
        }

        #debug_impl
    }
}

fn get_rtype(attrs: &[Attribute]) -> syn::Type {
    attrs
        .iter()
        .find(|attr| attr.meta.path().is_ident("rtype"))
        .and_then(|rtype_attr| syn::parse2(rtype_attr.parse_args().unwrap()).ok())
        .unwrap_or_else(|| syn::parse_str("()").unwrap())
}

fn impl_debug_for_struct(name: &Ident, data: &DataStruct) -> proc_macro2::TokenStream {
    let format_debug = match data.fields {
        Fields::Named(ref fields) => {
            let (field_names, field_debug) = map_named_struct(data.fields.len(), fields);
            quote! {
                #name { #(#field_names),* } => f.debug_struct(stringify!(#name))
                    #(#field_debug)*
                    .finish()
            }
        }
        Fields::Unnamed(ref fields) => {
            let (field_names, field_debug) = map_unnamed_struct(data.fields.len(), fields);
            quote! {
                #name ( #(#field_names),* ) => f.debug_tuple(stringify!(#name))
                    #(#field_debug)*
                    .finish()
            }
        }
        Fields::Unit => {
            quote! {
                #name => write!(f, stringify!(#name))
            }
        }
    };

    quote! {
        #[automatically_derived]
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #format_debug
                }
            }
        }
    }
}

fn impl_debug_for_enum(name: &Ident, data: &DataEnum) -> proc_macro2::TokenStream {
    let variants_debug = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Named(ref fields) => {
                let (field_names, field_debug) = map_named_struct(fields.named.len(), fields);
                quote! {
                    #name::#variant_name { #(#field_names),* } => {
                        f.debug_struct(format!("{}::{}", stringify!(#name), stringify!(#variant_name)).as_str())
                            #(#field_debug)*
                            .finish()
                    }
                }
            }
            Fields::Unnamed(ref fields) => {
                let (field_names, field_debug) = map_unnamed_struct(fields.unnamed.len(), fields);
                quote! {
                    #name::#variant_name( #(#field_names),* ) => {
                        f.debug_tuple(format!("{}::{}", stringify!(#name), stringify!(#variant_name)).as_str())
                            #(#field_debug)*
                            .finish()
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #name::#variant_name => {
                        write!(f, "{}::{}", stringify!(#name), stringify!(#variant_name))
                    }
                }
            }
        }
    });

    quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#variants_debug),*
                }
            }
        }
    }
}

fn map_named_struct(
    len: usize,
    fields: &FieldsNamed,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut field_names: Vec<proc_macro2::TokenStream> = Vec::with_capacity(len);
    let mut field_debug: Vec<proc_macro2::TokenStream> = Vec::with_capacity(len);
    fields.named.iter().enumerate().for_each(|(i, field)| {
        let is_hidden = field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("hidden"));
        let binding_name = format_ident!("__binding_{}", i);
        let is_obfuscated = field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("obfuscated"));

        let field_name = field.ident.clone().unwrap();

        field_names.push(quote! {
            #field_name: ref #binding_name
        });
        if is_hidden {
            return;
        }
        if is_obfuscated {
            field_debug.push(quote! {
                .field(stringify!(#field_name), &"<obfuscated>")
            });
        } else {
            field_debug.push(quote! {
                .field(stringify!(#field_name), &#binding_name)
            });
        }
    });
    (field_names, field_debug)
}

fn map_unnamed_struct(
    len: usize,
    fields: &FieldsUnnamed,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut field_names: Vec<proc_macro2::TokenStream> = Vec::with_capacity(len);
    let mut field_debug: Vec<proc_macro2::TokenStream> = Vec::with_capacity(len);
    fields.unnamed.iter().enumerate().for_each(|(i, field)| {
        let is_hidden = field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("hidden"));

        let binding_name = format_ident!("__binding_{}", i);
        let is_obfuscated = field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("obfuscated"));

        field_names.push(quote! {
            ref #binding_name
        });
        if is_hidden {
            return;
        }
        if is_obfuscated {
            field_debug.push(quote! {
                .field(&"<obfuscated>")
            });
        } else {
            field_debug.push(quote! {
                .field(#binding_name)
            });
        }
    });
    (field_names, field_debug)
}
