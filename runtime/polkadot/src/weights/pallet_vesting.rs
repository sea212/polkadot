// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7306`
		// Minimum execution time: 30_459 nanoseconds.
		Weight::from_ref_time(29_414_981)
			.saturating_add(Weight::from_proof_size(7306))
			// Standard Error: 853
			.saturating_add(Weight::from_ref_time(44_483).saturating_mul(l.into()))
			// Standard Error: 1_518
			.saturating_add(Weight::from_ref_time(64_668).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `377 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `7306`
		// Minimum execution time: 29_310 nanoseconds.
		Weight::from_ref_time(29_062_397)
			.saturating_add(Weight::from_proof_size(7306))
			// Standard Error: 877
			.saturating_add(Weight::from_ref_time(36_222).saturating_mul(l.into()))
			// Standard Error: 1_562
			.saturating_add(Weight::from_ref_time(42_827).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9909`
		// Minimum execution time: 32_792 nanoseconds.
		Weight::from_ref_time(32_223_934)
			.saturating_add(Weight::from_proof_size(9909))
			// Standard Error: 833
			.saturating_add(Weight::from_ref_time(46_218).saturating_mul(l.into()))
			// Standard Error: 1_483
			.saturating_add(Weight::from_ref_time(68_198).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9909`
		// Minimum execution time: 32_330 nanoseconds.
		Weight::from_ref_time(31_644_065)
			.saturating_add(Weight::from_proof_size(9909))
			// Standard Error: 887
			.saturating_add(Weight::from_ref_time(43_137).saturating_mul(l.into()))
			// Standard Error: 1_578
			.saturating_add(Weight::from_ref_time(55_494).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9909`
		// Minimum execution time: 47_622 nanoseconds.
		Weight::from_ref_time(47_953_546)
			.saturating_add(Weight::from_proof_size(9909))
			// Standard Error: 2_056
			.saturating_add(Weight::from_ref_time(39_785).saturating_mul(l.into()))
			// Standard Error: 3_659
			.saturating_add(Weight::from_ref_time(41_672).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `718 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `12512`
		// Minimum execution time: 50_175 nanoseconds.
		Weight::from_ref_time(50_736_243)
			.saturating_add(Weight::from_proof_size(12512))
			// Standard Error: 4_214
			.saturating_add(Weight::from_ref_time(36_528).saturating_mul(l.into()))
			// Standard Error: 7_498
			.saturating_add(Weight::from_ref_time(48_527).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9909`
		// Minimum execution time: 33_600 nanoseconds.
		Weight::from_ref_time(32_676_944)
			.saturating_add(Weight::from_proof_size(9909))
			// Standard Error: 1_328
			.saturating_add(Weight::from_ref_time(51_061).saturating_mul(l.into()))
			// Standard Error: 2_452
			.saturating_add(Weight::from_ref_time(79_609).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Vesting Vesting (r:1 w:1)
	/// Proof: Vesting Vesting (max_values: None, max_size: Some(1057), added: 3532, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `9909`
		// Minimum execution time: 34_035 nanoseconds.
		Weight::from_ref_time(33_098_531)
			.saturating_add(Weight::from_proof_size(9909))
			// Standard Error: 838
			.saturating_add(Weight::from_ref_time(46_629).saturating_mul(l.into()))
			// Standard Error: 1_548
			.saturating_add(Weight::from_ref_time(72_485).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
