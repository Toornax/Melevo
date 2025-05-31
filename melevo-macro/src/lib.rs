use proc_macro::TokenStream;

mod flagset;

#[proc_macro_derive(FlagSet, attributes(flagset))]
pub fn flagset_derive(input: TokenStream) -> TokenStream {
	flagset::proc_macro(input)

}