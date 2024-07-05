use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

#[proc_macro_derive(
    Electroplate,
    attributes(Internal, AllowSetter, NoGetter, NoSetter, ChildGetters)
)]
pub fn create_encapsulation_impls(t: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(t as DeriveInput);

    create_std_impl(ast).into()
}

fn create_std_impl(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident.clone();

    let struct_fields = match input.data {
        syn::Data::Struct(struct_data) => struct_data.fields,
        syn::Data::Enum(_) => {
            return syn::Error::new_spanned(
                input,
                "Electroplate does not support enumerated types yet",
            )
            .to_compile_error()
        }
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(input, "Electroplate does not support unions")
                .to_compile_error()
        }
    };

    let struct_getters = struct_fields
        .iter()
        .map(|field| process_struct_member(field));

    quote! {
        impl #struct_name {
            #(#struct_getters)*
        }
    }
}

//For now, just generates getters/setters for
fn process_struct_member(field: &Field) -> TokenStream {
    //Skip public fields
    // if field.vis == Visibility::Public(token!()) {
    //     return quote! {};
    // }

    let ident = field.ident.clone().unwrap();
    // let set_ident =
    let field_type = field.ty.clone();

    let getter_name = quote! {
        pub fn #ident(&self) -> #field_type{
            self.#ident
        }
    };

    quote! {
        #getter_name
    }
}
