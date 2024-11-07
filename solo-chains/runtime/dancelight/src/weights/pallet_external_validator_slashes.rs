// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_external_validator_slashes
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-10-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `girazoki-XPS-15-9530`, CPU: `13th Gen Intel(R) Core(TM) i9-13900H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_external_validator_slashes
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/pallet_external_validator_slashes.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_external_validator_slashes using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_external_validator_slashes::WeightInfo for SubstrateWeight<T> {
	/// Storage: `ExternalValidators::ActiveEra` (r:1 w:0)
	/// Proof: `ExternalValidators::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidatorSlashes::Slashes` (r:1 w:1)
	/// Proof: `ExternalValidatorSlashes::Slashes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `s` is `[1, 1000]`.
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42194`
		//  Estimated: `45659`
		// Minimum execution time: 67_311_000 picoseconds.
		Weight::from_parts(536_999_990, 45659)
			// Standard Error: 37_157
			.saturating_add(Weight::from_parts(3_022_012, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ExternalValidators::ActiveEra` (r:1 w:0)
	/// Proof: `ExternalValidators::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidatorSlashes::NextSlashId` (r:1 w:1)
	/// Proof: `ExternalValidatorSlashes::NextSlashId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ExternalValidatorSlashes::Slashes` (r:1 w:1)
	/// Proof: `ExternalValidatorSlashes::Slashes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_inject_slash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `3616`
		// Minimum execution time: 7_398_000 picoseconds.
		Weight::from_parts(7_725_000, 3616)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}