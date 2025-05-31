use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

pub(crate) fn proc_macro(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;

	// List struc fields
	let fields = match input.data {
		Data::Struct(data) => match data.fields {
			Fields::Named(named) => named.named,
			_ => panic!("FlagSet only supports named fields"),
		},
		_ => panic!("FlagSet can only be derived for structs"),
	};

	let relevant_fields: Vec<_> = fields.iter()
		.filter(|field| {
			!is_field_skipped(field)
		})
		.collect();

	let field_names: Vec<_> = relevant_fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
	let field_strs: Vec<_> = field_names.iter().map(|f| f.to_string()).collect();

	let contains = quote! {
		pub fn contains(&self, other: &Self) -> bool {
			true #( && (self.#field_names || !other.#field_names) )*
		}
	};

	let union = quote! {
		pub fn union(&self, other: &Self) -> Self {
			Self {
				#(#field_names: self.#field_names || other.#field_names),*,
				..Default::default()
			}
		}
	};

	let intersection = quote! {
		pub fn intersection(&self, other: &Self) -> Self {
			Self {
				#(#field_names: self.#field_names && other.#field_names),*,
				..Default::default()
			}
		}
	};

	let difference = quote! {
		pub fn difference(&self, other: &Self) -> Self {
			Self {
				#(#field_names: self.#field_names && !other.#field_names),*,
				..Default::default()
			}
		}
	};

	let debug = quote! {
		impl std::fmt::Debug for #name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "[")?;
				let mut first = true;
				#(if self.#field_names {
					if !first { write!(f, ", ")?; }
					write!(f, "{}", #field_strs)?;
					first = false;
				})*
				write!(f, "]")
			}
		}
	};

	let from_iter = quote! {
		impl<'a> FromIterator<&'a str> for #name {
			fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
				let mut set = Self::default();
				for field in iter {
					match field {
						#(#field_strs => set.#field_names = true,)*
						_ => {}
					}
				}
				set
			}
		}
	};

	let into_iter = quote! {
		impl IntoIterator for #name {
			type Item = (String, bool);
			type IntoIter = std::vec::IntoIter<Self::Item>;

			fn into_iter(self) -> Self::IntoIter {
				let mut items = Vec::new();
				#(if self.#field_names {
					items.push((String::from(#field_strs), self.#field_names));
				})*
				items.into_iter()
			}
		}
	};

	let into_iter_ref = quote! {
		impl<'a> IntoIterator for &'a #name {
			type Item = (&'a str, bool);
			type IntoIter = std::vec::IntoIter<Self::Item>;

			fn into_iter(self) -> Self::IntoIter {
				let mut items = Vec::new();
				#(if self.#field_names {
					items.push((#field_strs, self.#field_names));
				})*
				items.into_iter()
			}
		}
	};

	let iter = quote! {
		pub fn iter(&self) -> impl Iterator<Item = (&str, bool)> {
			self.into_iter()
		}
	};

	let into_iter_ref_mut = quote! {
		impl<'a> IntoIterator for &'a mut #name {
			type Item = (&'a str, &'a mut bool);
			type IntoIter = std::vec::IntoIter<Self::Item>;

			fn into_iter(self) -> Self::IntoIter {
				let mut items = Vec::new();
				#(if self.#field_names {
					items.push((#field_strs, &mut self.#field_names));
				})*
				items.into_iter()
			}
		}
	};

	let iter_mut = quote! {
		pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut bool)> {
			self.into_iter()
		}
	};

	let ops = quote! {
		impl std::ops::BitAnd for #name {
			type Output = Self;
			fn bitand(self, rhs: Self) -> Self::Output {
				self.intersection(&rhs)
			}
		}

		impl std::ops::BitAndAssign for #name {
			fn bitand_assign(&mut self, rhs: Self) {
				*self = self.intersection(&rhs);
			}
		}

		impl std::ops::BitOr for #name {
			type Output = Self;
			fn bitor(self, rhs: Self) -> Self::Output {
				self.union(&rhs)
			}
		}

		impl std::ops::BitOrAssign for #name {
			fn bitor_assign(&mut self, rhs: Self) {
				*self = self.union(&rhs);
			}
		}

		impl std::ops::Sub for #name {
			type Output = Self;
			fn sub(self, rhs: Self) -> Self::Output {
				self.difference(&rhs)
			}
		}

		impl std::ops::SubAssign for #name {
			fn sub_assign(&mut self, rhs: Self) {
				*self = self.difference(&rhs);
			}
		}
	};

	let output = quote! {
		impl #name {
			#contains
			#union
			#intersection
			#difference
			#iter
			#iter_mut
		}

		#debug
		#from_iter
		#into_iter
		#into_iter_ref
		#into_iter_ref_mut
		#ops
	};

	output.into()
}

fn is_field_skipped(field: &syn::Field) -> bool {
	field.attrs.iter().any(|attr| {
		eprintln!("Attr: {:?}", attr.path().get_ident());
		if attr.path().is_ident("flagset") {
			if let Meta::List(meta_list) = &attr.meta {
				return meta_list.tokens.to_string().contains("skip");
			}
		}

		return false;
	})
}

