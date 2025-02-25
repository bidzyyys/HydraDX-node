// This file is part of HydraDX.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_omnipool_liquidity_mining
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-07, STEPS: 1, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/hydradx
// benchmark
// pallet
// --pallet=pallet_omnipool_liquidity_mining
// --chain=dev
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template.hbs
// --output=pallets/omnipool-liquidity-mining/src/weights.rs
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_omnipool_liquidity_mining.
pub trait WeightInfo {
	fn create_global_farm() -> Weight;
	fn update_global_farm() -> Weight;
	fn terminate_global_farm() -> Weight;
	fn create_yield_farm() -> Weight;
	fn update_yield_farm() -> Weight;
	fn stop_yield_farm() -> Weight;
	fn resume_yield_farm() -> Weight;
	fn terminate_yield_farm() -> Weight;
	fn deposit_shares() -> Weight;
	fn redeposit_shares() -> Weight;
	fn claim_rewards() -> Weight;
	fn withdraw_shares() -> Weight;
}

/// Weights for pallet_omnipool_liquidity_mining using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn create_global_farm() -> Weight {
		Weight::from_ref_time(86_884_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn update_global_farm() -> Weight {
		Weight::from_ref_time(32_552_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn terminate_global_farm() -> Weight {
		Weight::from_ref_time(85_822_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn create_yield_farm() -> Weight {
		Weight::from_ref_time(110_999_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn update_yield_farm() -> Weight {
		Weight::from_ref_time(114_095_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn stop_yield_farm() -> Weight {
		Weight::from_ref_time(107_221_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn resume_yield_farm() -> Weight {
		Weight::from_ref_time(111_089_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn terminate_yield_farm() -> Weight {
		Weight::from_ref_time(78_328_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn deposit_shares() -> Weight {
		Weight::from_ref_time(183_245_000 as u64)
			.saturating_add(T::DbWeight::get().reads(14 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	fn redeposit_shares() -> Weight {
		Weight::from_ref_time(152_727_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn claim_rewards() -> Weight {
		Weight::from_ref_time(156_835_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn withdraw_shares() -> Weight {
		Weight::from_ref_time(236_615_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_global_farm() -> Weight {
		Weight::from_ref_time(86_884_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn update_global_farm() -> Weight {
		Weight::from_ref_time(32_552_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn terminate_global_farm() -> Weight {
		Weight::from_ref_time(85_822_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn create_yield_farm() -> Weight {
		Weight::from_ref_time(110_999_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn update_yield_farm() -> Weight {
		Weight::from_ref_time(114_095_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn stop_yield_farm() -> Weight {
		Weight::from_ref_time(107_221_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn resume_yield_farm() -> Weight {
		Weight::from_ref_time(111_089_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn terminate_yield_farm() -> Weight {
		Weight::from_ref_time(78_328_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	fn deposit_shares() -> Weight {
		Weight::from_ref_time(183_245_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(14 as u64))
			.saturating_add(RocksDbWeight::get().writes(14 as u64))
	}
	fn redeposit_shares() -> Weight {
		Weight::from_ref_time(152_727_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(12 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	fn claim_rewards() -> Weight {
		Weight::from_ref_time(156_835_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn withdraw_shares() -> Weight {
		Weight::from_ref_time(236_615_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(13 as u64))
			.saturating_add(RocksDbWeight::get().writes(15 as u64))
	}
}
