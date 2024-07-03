// use proc_macro::TokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Visibility};

#[proc_macro_derive(
    Electroplate,
    attributes(Internal, AllowSetter, NoGetter, NoSetter, ChildGetters)
)]
pub fn create_encapsulation_impls(t: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(t as DeriveInput);

    todo!()
}

fn create_std_impl(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    let struct_fields = match input.data {
        syn::Data::Struct(struct_data) => struct_data.fields,
        syn::Data::Enum(_) => panic!("Electroplate does not support enumerated types yet"),
        syn::Data::Union(_) => panic!("Electroplate does not support unions"),
    };

    for field in struct_fields {
        // let field_attr = field.attrs;
    }

    quote! {
        impl #struct_name {

        }
    }
}

//For now, just generates getters/setters for
fn process_struct_member(field: Field) -> TokenStream {
    //Skip public fields
    if field.vis == Visibility::Public(token!()) {
        return quote! {};
    }

	let ident = field.ident.unwrap_or_default();
	// let set_ident =
	let field_type = field.ty;

	quote!{
		pub fn #ident() -> #field_type{
			ident
		}
	}

}
