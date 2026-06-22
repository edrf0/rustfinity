#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(fields) => fields.named,
            syn::Fields::Unnamed(..) => panic!("Unnamed fields are not supported"),
        }
        _ => panic!("Only structs can derive Describe yet"),
    };

    let field_descriptors = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { format!("{}: {:?}", stringify!(#name), &self.#name) }
    });

    let expanded = quote! {
        impl Describe for #name {
            fn describe(&self) -> String {
                let fields = vec![#(#field_descriptors),*].join(", ");
                format!("{} {{ {} }}", stringify!(#name), fields)
            }
        }
    };

    expanded.into()
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
