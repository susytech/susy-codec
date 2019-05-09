// Copyright 2017, 2018 Susy Technologies
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

// tag::description[]
//! Implements a serialization and deserialization codec for simple marshalling.
// end::description[]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "std")]
extern crate serde;

extern crate arrayvec;

#[cfg(feature = "susy-codec-derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate susy_codec_derive;

#[cfg(all(feature = "std", test))]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "susy-codec-derive")]
#[doc(hidden)]
pub use susy_codec_derive::*;

#[cfg(feature = "std")]
pub mod alloc {
	pub use ::std::boxed;
	pub use ::std::vec;
	pub use ::std::string;
	pub use ::std::borrow;

	#[cfg(feature = "full")]
	mod full {
		pub use ::std::borrow;
	}
	#[cfg(feature = "full")]
	pub use self::full::*;
}

mod codec;
mod joiner;
mod keyedvec;

pub use self::codec::{Input, Output, Encode, Decode, Codec, Compact, HasCompact, EncodeAsRef, CompactAs};
pub use self::joiner::Joiner;
pub use self::keyedvec::KeyedVec;
