// This file is part of Hydra-node.

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

//! Autogenerated weights for pallet_otc
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-17, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-otc
// --output=otc.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_otc::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn place_order() -> Weight {
		Weight::from_ref_time(68_222_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn partial_fill_order() -> Weight {
		Weight::from_ref_time(122_519_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn fill_order() -> Weight {
		Weight::from_ref_time(118_970_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn cancel_order() -> Weight {
		Weight::from_ref_time(55_247_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
