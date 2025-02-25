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

//! Autogenerated weights for pallet_circuit_breaker
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-26, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-circuit-breaker
// --chain=local
// --steps=5
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// weights.rs
// --template
// .maintain/pallet-weight-template.hbs
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_omnipool.
pub trait WeightInfo {
	fn on_finalize(m: u32, n: u32) -> Weight;
	fn on_finalize_single() -> Weight;
	fn on_finalize_empty() -> Weight;
	fn set_trade_volume_limit() -> Weight;
	fn set_add_liquidity_limit() -> Weight;
	fn set_remove_liquidity_limit() -> Weight;
	fn ensure_pool_state_change_limit() -> Weight;
	fn ensure_add_liquidity_limit() -> Weight;
	fn ensure_remove_liquidity_limit() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_finalize(_m: u32, _n: u32) -> Weight {
		Weight::zero()
	}
	fn on_finalize_single() -> Weight {
		Weight::zero()
	}
	fn on_finalize_empty() -> Weight {
		Weight::zero()
	}
	fn set_trade_volume_limit() -> Weight {
		Weight::zero()
	}
	fn set_add_liquidity_limit() -> Weight {
		Weight::zero()
	}
	fn set_remove_liquidity_limit() -> Weight {
		Weight::zero()
	}
	fn ensure_pool_state_change_limit() -> Weight {
		Weight::zero()
	}
	fn ensure_add_liquidity_limit() -> Weight {
		Weight::zero()
	}
	fn ensure_remove_liquidity_limit() -> Weight {
		Weight::zero()
	}
}
