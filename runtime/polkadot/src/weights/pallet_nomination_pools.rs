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
//! Autogenerated weights for `pallet_nomination_pools`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/var/lib/gitlab-runner/builds/zyw4fam_/0/parity/mirrors/polkadot/.git/.artifacts/bench.json
// --pallet=pallet_nomination_pools
// --chain=polkadot-dev
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nomination_pools`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nomination_pools::WeightInfo for WeightInfo<T> {
	/// Storage: NominationPools MinJoinBond (r:1 w:0)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn join() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3545`
		//  Estimated: `39055`
		// Minimum execution time: 151_047 nanoseconds.
		Weight::from_ref_time(152_189_000)
			.saturating_add(Weight::from_proof_size(39055))
			.saturating_add(T::DbWeight::get().reads(18))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn bond_extra_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3587`
		//  Estimated: `39650`
		// Minimum execution time: 149_336 nanoseconds.
		Weight::from_ref_time(150_402_000)
			.saturating_add(Weight::from_proof_size(39650))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools ClaimPermissions (r:1 w:0)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:2 w:2)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn bond_extra_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3470`
		//  Estimated: `39537`
		// Minimum execution time: 161_133 nanoseconds.
		Weight::from_ref_time(162_471_000)
			.saturating_add(Weight::from_proof_size(39537))
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: NominationPools ClaimPermissions (r:1 w:0)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1297`
		//  Estimated: `14072`
		// Minimum execution time: 61_090 nanoseconds.
		Weight::from_ref_time(61_986_000)
			.saturating_add(Weight::from_proof_size(14072))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:3 w:3)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:2 w:2)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(1197), added: 3672, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools CounterForSubPoolsStorage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn unbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3825`
		//  Estimated: `45261`
		// Minimum execution time: 154_331 nanoseconds.
		Weight::from_ref_time(155_416_000)
			.saturating_add(Weight::from_proof_size(45261))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn pool_withdraw_unbonded(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1676`
		//  Estimated: `13081`
		// Minimum execution time: 52_546 nanoseconds.
		Weight::from_ref_time(54_467_358)
			.saturating_add(Weight::from_proof_size(13081))
			// Standard Error: 3_643
			.saturating_add(Weight::from_ref_time(5_483).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(1197), added: 3672, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:0 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_update(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2200`
		//  Estimated: `23047`
		// Minimum execution time: 100_942 nanoseconds.
		Weight::from_ref_time(106_524_077)
			.saturating_add(Weight::from_proof_size(23047))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools SubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools SubPoolsStorage (max_values: None, max_size: Some(1197), added: 3672, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:1)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:1 w:0)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools ReversePoolIdLookup (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools CounterForReversePoolIdLookup (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	/// Proof: NominationPools CounterForRewardPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForSubPoolsStorage (r:1 w:1)
	/// Proof: NominationPools CounterForSubPoolsStorage (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools Metadata (r:1 w:1)
	/// Proof: NominationPools Metadata (max_values: None, max_size: Some(270), added: 2745, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	/// Proof: NominationPools CounterForBondedPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:1)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:0 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn withdraw_unbonded_kill(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2588`
		//  Estimated: `46093`
		// Minimum execution time: 159_736 nanoseconds.
		Weight::from_ref_time(162_919_849)
			.saturating_add(Weight::from_proof_size(46093))
			// Standard Error: 16_508
			.saturating_add(Weight::from_ref_time(31_316).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	/// Storage: NominationPools LastPoolId (r:1 w:1)
	/// Proof: NominationPools LastPoolId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MinCreateBond (r:1 w:0)
	/// Proof: NominationPools MinCreateBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MinJoinBond (r:1 w:0)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPools (r:1 w:0)
	/// Proof: NominationPools MaxPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForBondedPools (r:1 w:1)
	/// Proof: NominationPools CounterForBondedPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools PoolMembers (r:1 w:1)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:1 w:0)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForPoolMembers (r:1 w:1)
	/// Proof: NominationPools CounterForPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:1)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForRewardPools (r:1 w:1)
	/// Proof: NominationPools CounterForRewardPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools ReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools ReversePoolIdLookup (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForReversePoolIdLookup (r:1 w:1)
	/// Proof: NominationPools CounterForReversePoolIdLookup (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:1)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1220`
		//  Estimated: `32090`
		// Minimum execution time: 141_144 nanoseconds.
		Weight::from_ref_time(142_512_000)
			.saturating_add(Weight::from_proof_size(32090))
			.saturating_add(T::DbWeight::get().reads(21))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking MinNominatorBond (r:1 w:0)
	/// Proof: Staking MinNominatorBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking MaxNominatorsCount (r:1 w:0)
	/// Proof: Staking MaxNominatorsCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:17 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 16]`.
	fn nominate(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1809`
		//  Estimated: `22054 + n * (2520 ±0)`
		// Minimum execution time: 65_331 nanoseconds.
		Weight::from_ref_time(64_849_311)
			.saturating_add(Weight::from_proof_size(22054))
			// Standard Error: 57_463
			.saturating_add(Weight::from_ref_time(1_585_804).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_proof_size(2520).saturating_mul(n.into()))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	fn set_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1397`
		//  Estimated: `8808`
		// Minimum execution time: 32_736 nanoseconds.
		Weight::from_ref_time(33_293_000)
			.saturating_add(Weight::from_proof_size(8808))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools Metadata (r:1 w:1)
	/// Proof: NominationPools Metadata (max_values: None, max_size: Some(270), added: 2745, mode: MaxEncodedLen)
	/// Storage: NominationPools CounterForMetadata (r:1 w:1)
	/// Proof: NominationPools CounterForMetadata (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 256]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `529`
		//  Estimated: `5939`
		// Minimum execution time: 14_287 nanoseconds.
		Weight::from_ref_time(15_073_749)
			.saturating_add(Weight::from_proof_size(5939))
			// Standard Error: 427
			.saturating_add(Weight::from_ref_time(1_283).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools MinJoinBond (r:0 w:1)
	/// Proof: NominationPools MinJoinBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembers (r:0 w:1)
	/// Proof: NominationPools MaxPoolMembers (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPoolMembersPerPool (r:0 w:1)
	/// Proof: NominationPools MaxPoolMembersPerPool (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MinCreateBond (r:0 w:1)
	/// Proof: NominationPools MinCreateBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:0 w:1)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: NominationPools MaxPools (r:0 w:1)
	/// Proof: NominationPools MaxPools (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_configs() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_678 nanoseconds.
		Weight::from_ref_time(6_005_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn update_roles() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `529`
		//  Estimated: `2695`
		// Minimum execution time: 19_892 nanoseconds.
		Weight::from_ref_time(20_142_000)
			.saturating_add(Weight::from_proof_size(2695))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn chill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2036`
		//  Estimated: `20545`
		// Minimum execution time: 65_685 nanoseconds.
		Weight::from_ref_time(66_500_000)
			.saturating_add(Weight::from_proof_size(20545))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_commission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `832`
		//  Estimated: `8364`
		// Minimum execution time: 32_039 nanoseconds.
		Weight::from_ref_time(32_443_000)
			.saturating_add(Weight::from_proof_size(8364))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn set_commission_max() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `569`
		//  Estimated: `2695`
		// Minimum execution time: 19_446 nanoseconds.
		Weight::from_ref_time(19_864_000)
			.saturating_add(Weight::from_proof_size(2695))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:1)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	fn set_commission_change_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `529`
		//  Estimated: `2695`
		// Minimum execution time: 20_303 nanoseconds.
		Weight::from_ref_time(20_490_000)
			.saturating_add(Weight::from_proof_size(2695))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools PoolMembers (r:1 w:0)
	/// Proof: NominationPools PoolMembers (max_values: None, max_size: Some(717), added: 3192, mode: MaxEncodedLen)
	/// Storage: NominationPools ClaimPermissions (r:1 w:1)
	/// Proof: NominationPools ClaimPermissions (max_values: None, max_size: Some(41), added: 2516, mode: MaxEncodedLen)
	fn set_claim_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `5708`
		// Minimum execution time: 14_857 nanoseconds.
		Weight::from_ref_time(15_065_000)
			.saturating_add(Weight::from_proof_size(5708))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: NominationPools BondedPools (r:1 w:0)
	/// Proof: NominationPools BondedPools (max_values: None, max_size: Some(220), added: 2695, mode: MaxEncodedLen)
	/// Storage: NominationPools RewardPools (r:1 w:1)
	/// Proof: NominationPools RewardPools (max_values: None, max_size: Some(92), added: 2567, mode: MaxEncodedLen)
	/// Storage: NominationPools GlobalMaxCommission (r:1 w:0)
	/// Proof: NominationPools GlobalMaxCommission (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_commission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1062`
		//  Estimated: `8364`
		// Minimum execution time: 43_212 nanoseconds.
		Weight::from_ref_time(43_826_000)
			.saturating_add(Weight::from_proof_size(8364))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
