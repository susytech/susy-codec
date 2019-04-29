// Copyright 2018 Susy Technologies
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

use syn::{Meta, NestedMeta, Lit, Attribute, Variant, Field};
use proc_macro2::{TokenStream, Span};
use std::str::FromStr;

fn find_meta_item<'a, F, R, I>(itr: I, pred: F) -> Option<R> where
	F: FnMut(&NestedMeta) -> Option<R> + Clone,
	I: Iterator<Item=&'a Attribute>
{
	itr.filter_map(|attr| {
		let pair = attr.path.segments.first()?;
		let seg = pair.value();
		if seg.ident == "codec" {
			let meta = attr.interpret_meta();
			if let Some(Meta::List(ref meta_list)) = meta {
				return meta_list.nested.iter().filter_map(pred.clone()).next();
			}
		}

		None
	}).next()
}

pub fn index(v: &Variant, i: usize) -> TokenStream {
	// look for an index in attributes
	let index = find_meta_item(v.attrs.iter(), |meta| {
		if let NestedMeta::Meta(Meta::NameValue(ref nv)) = meta {
			if nv.ident == "index" {
				if let Lit::Str(ref s) = nv.lit {
					let byte: u8 = s.value().parse().expect("Numeric index expected.");
					return Some(byte)
				}
			}
		}

		None
	});

	// then fallback to discriminant or just index
	index.map(|i| quote! { #i })
		.unwrap_or_else(|| v.discriminant
			.as_ref()
			.map(|&(_, ref expr)| quote! { #expr })
			.unwrap_or_else(|| quote! { #i })
		)
}

pub fn get_encoded_as_type(field_entry: &Field) -> Option<TokenStream> {
	// look for an encoded_as in attributes
	find_meta_item(field_entry.attrs.iter(), |meta| {
		if let NestedMeta::Meta(Meta::NameValue(ref nv)) = meta {
			if nv.ident == "encoded_as"{
				if let Lit::Str(ref s) = nv.lit {
					return Some(
						TokenStream::from_str(&s.value())
							.expect("`encoded_as` should be a valid rust type!")
					);
				}
			}
		}

		None
	})
}

pub fn get_enable_compact(field_entry: &Field) -> bool {
	// look for `encode(compact)` in the attributes
	find_meta_item(field_entry.attrs.iter(), |meta| {
		if let NestedMeta::Meta(Meta::Word(ref word)) = meta {
			if word == "compact" {
				return Some(());
			}
		}

		None
	}).is_some()
}

// return span of skip if found
pub fn get_skip(attrs: &Vec<Attribute>) -> Option<Span> {
	// look for `skip` in the attributes
	find_meta_item(attrs.iter(), |meta| {
		if let NestedMeta::Meta(Meta::Word(ref word)) = meta {
			if word == "skip" {
				return Some(word.span());
			}
		}

		None
	})
}
