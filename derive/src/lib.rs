// Copyright 2017-2018 Susy Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Derives serialization and deserialization codec for complex structs for simple marshalling.

#![recursion_limit = "128"]
extern crate proc_macro;
use proc_macro2;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, Ident, parse::Error};
use proc_macro_crate::crate_name;

use std::env;

mod decode;
mod encode;
mod utils;
mod trait_bounds;

/// Include the `susy-codec` crate under a known name (`_susy_codec`).
fn include_susy_codec_crate() -> proc_macro2::TokenStream {
	// This "hack" is required for the tests.
	if env::var("CARGO_PKG_NAME").unwrap() == "susy-codec" {
		quote!( extern crate susy_codec as _susy_codec; )
	} else {
		match crate_name("susy-codec") {
			Ok(susy_codec_crate) => {
				let ident = Ident::new(&susy_codec_crate, Span::call_site());
				quote!( extern crate #ident as _susy_codec; )
			},
			Err(e) => Error::new(Span::call_site(), &e).to_compile_error(),
		}
	}
}

#[proc_macro_derive(Encode, attributes(codec))]
pub fn encode_derive(input: TokenStream) -> TokenStream {
	let mut input: DeriveInput = match syn::parse(input) {
		Ok(input) => input,
		Err(e) => return e.to_compile_error().into(),
	};

	if let Err(e) = trait_bounds::add(
		&input.ident,
		&mut input.generics,
		&input.data,
		parse_quote!(_susy_codec::Encode)
	) {
		return e.to_compile_error().into();
	}

	let name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let encode_impl = encode::quote(&input.data, name);

	let impl_block = quote! {
		impl #impl_generics _susy_codec::Encode for #name #ty_generics #where_clause {
			#encode_impl
		}
	};

	let mut new_name = "_IMPL_ENCODE_FOR_".to_string();
	new_name.push_str(name.to_string().trim_start_matches("r#"));
	let dummy_const = Ident::new(&new_name, Span::call_site());
	let susy_codec_crate = include_susy_codec_crate();

	let generated = quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			#[allow(unknown_lints)]
			#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
			#[allow(rust_2018_idioms)]
			#susy_codec_crate
			#impl_block
		};
	};

	generated.into()
}

#[proc_macro_derive(Decode, attributes(codec))]
pub fn decode_derive(input: TokenStream) -> TokenStream {
	let mut input: DeriveInput = match syn::parse(input) {
		Ok(input) => input,
		Err(e) => return e.to_compile_error().into(),
	};

	if let Err(e) = trait_bounds::add(
		&input.ident,
		&mut input.generics,
		&input.data,
		parse_quote!(_susy_codec::Decode),
	) {
		return e.to_compile_error().into();
	}

	let name = &input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let input_ = quote!(input);
	let decoding = decode::quote(&input.data, name, &input_);

	let impl_block = quote! {
		impl #impl_generics _susy_codec::Decode for #name #ty_generics #where_clause {
			fn decode<DecIn: _susy_codec::Input>(#input_: &mut DecIn) -> Result<Self, _susy_codec::Error> {
				#decoding
			}
		}
	};

	let mut new_name = "_IMPL_DECODE_FOR_".to_string();
	new_name.push_str(name.to_string().trim_start_matches("r#"));
	let dummy_const = Ident::new(&new_name, Span::call_site());
	let susy_codec_crate = include_susy_codec_crate();

	let generated = quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			#[allow(unknown_lints)]
			#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
			#[allow(rust_2018_idioms)]
			#susy_codec_crate
			#impl_block
		};
	};

	generated.into()
}
