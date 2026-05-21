use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = input.into_iter().collect::<Vec<_>>();
    if input.len() != 1 {
        let msg = format!(
            "expected exactly one string literal as input, got {} tokens",
            input.len()
        );
        return quote! { compile_error!(#msg) }.into();
    }
    let string_lit = match StringLit::try_from(&input[0]) {
        Ok(lit) => lit,
        Err(e) => return e.to_compile_error(),
    };

    if let Err(e) = pg_query::parse(string_lit.value()) {
        let msg = format!("failed to parse SQL query: {}", e);
        return quote! { compile_error!(#msg) }.into();
    }

    input.into_iter().collect()
}
