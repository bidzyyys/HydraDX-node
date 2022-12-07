// This file is part of HydraDX.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{ensure, pallet_prelude::DispatchResult, traits::Get, transactional};
use hydradx_traits::OnPoolStateChangeHandler;
use scale_info::TypeInfo;
use sp_core::MaxEncodedLen;
use sp_runtime::traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedSub};
use sp_runtime::{ArithmeticError, Percent, RuntimeDebug};

pub mod weights;

#[cfg(test)]
mod tests;

#[derive(Clone, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo, Eq, PartialEq)]
#[scale_info(skip_type_params(T))]
pub struct LiquidityRange<T: Config> {
	pub min_limit: T::Balance,
	pub max_limit: T::Balance,
}

impl<T: Config> LiquidityRange<T>
where
	T::Balance: PartialOrd,
{
	pub fn check_min_limit(&self, liquidity: T::Balance) -> DispatchResult {
		ensure!(self.min_limit <= liquidity, Error::<T>::MinTradeVolumePerBlockReached);
		Ok(())
	}

	pub fn check_max_limit(&self, liquidity: T::Balance) -> DispatchResult {
		ensure!(self.max_limit >= liquidity, Error::<T>::MaxTradeVolumePerBlockReached);
		Ok(())
	}

	pub fn check_limits(&self, liquidity: T::Balance) -> DispatchResult {
		self.check_min_limit(liquidity)?;
		self.check_max_limit(liquidity)?;
		Ok(())
	}
}

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::HasCompact;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_finalize(_n: T::BlockNumber) {
			let _ = <AllowedLiquidityRangePerAsset<T>>::clear(u32::MAX, None);
		}

		fn integrity_test() {
			assert!(
				!T::DefaultMaxNetTradeVolumeLimitPerBlock::get().is_zero(),
				"Circuit Breaker: Max Net Trade Volume Limit Per Block is set to 0."
			);
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Identifier for the class of asset.
		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// Balance type.
		type Balance: Parameter
			+ Member
			+ Copy
			+ PartialOrd
			+ MaybeSerializeDeserialize
			+ Default
			+ CheckedAdd
			+ CheckedSub
			+ AtLeast32BitUnsigned;

		/// Origin to be able to change the trade volume limit of an asset.
		type TechnicalOrigin: EnsureOrigin<Self::Origin>;

		/// The maximum percentage of a pool's liquidity that can be traded in a block.
		type DefaultMaxNetTradeVolumeLimitPerBlock: Get<Percent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Default maximum net trade volume limit per block
	#[pallet::type_value]
	pub fn DefaultTradeVolumeLimit<T: Config>() -> Percent {
		T::DefaultMaxNetTradeVolumeLimitPerBlock::get()
	}

	#[pallet::storage]
	/// Allowed liquidity range per block, calculated based on the initial liquidity and trade volume limit percentage
	#[pallet::getter(fn allowed_liqudity_range_per_asset)]
	pub type AllowedLiquidityRangePerAsset<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, LiquidityRange<T>>;

	#[pallet::storage]
	/// Trade volume limits of assets that don't use the default value
	#[pallet::getter(fn trade_volume_limit_per_asset)]
	pub type TradeVolumeLimitPerAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, Percent, ValueQuery, DefaultTradeVolumeLimit<T>>;

	#[pallet::error]
	#[cfg_attr(test, derive(PartialEq, Eq))]
	pub enum Error<T> {
		/// Allowed liquidity limit is not stored for asset
		LiquidityLimitNotStoredForAsset,
		/// Minimum pool trade volume per block has been reached
		MinTradeVolumePerBlockReached,
		/// Maximum pool trade volume per block has been reached
		MaxTradeVolumePerBlockReached,
		/// Invalid trade volume limit. Limit must be non-zero.
		InvalidTradeVolumeLimit,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::set_trade_volume_limit())]
		#[transactional]
		pub fn set_trade_volume_limit(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			trade_volume_limit: Percent,
		) -> DispatchResult {
			T::TechnicalOrigin::ensure_origin(origin)?;
			ensure!(!trade_volume_limit.is_zero(), Error::<T>::InvalidTradeVolumeLimit);
			<TradeVolumeLimitPerAsset<T>>::insert(asset_id, trade_volume_limit);
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn calculate_and_store_liquidity_limits(asset_id: T::AssetId, initial_liquidity: T::Balance) -> DispatchResult {
		if !<AllowedLiquidityRangePerAsset<T>>::contains_key(asset_id) {
			let liquidity_diff = Pallet::<T>::trade_volume_limit_per_asset(asset_id).mul_floor(initial_liquidity);
			let min_limit = initial_liquidity
				.checked_sub(&liquidity_diff)
				.ok_or(ArithmeticError::Underflow)?;
			let max_limit = initial_liquidity
				.checked_add(&liquidity_diff)
				.ok_or(ArithmeticError::Overflow)?;
			<AllowedLiquidityRangePerAsset<T>>::insert(asset_id, LiquidityRange::<T> { min_limit, max_limit });
		}
		Ok(())
	}

	fn ensure_liquidity_limits(asset_id: T::AssetId, updated_liquidity: T::Balance) -> DispatchResult {
		let allowed_liquidity_range = Pallet::<T>::allowed_liqudity_range_per_asset(asset_id)
			.ok_or(Error::<T>::LiquidityLimitNotStoredForAsset)?;

		allowed_liquidity_range.check_limits(updated_liquidity)?;

		Ok(())
	}
}

impl<T: Config> OnPoolStateChangeHandler<T::AssetId, T::Balance> for Pallet<T> {
	fn before_pool_state_change(asset_in: T::AssetId, initial_liquidity: T::Balance) -> DispatchResult {
		Pallet::<T>::calculate_and_store_liquidity_limits(asset_in, initial_liquidity)?;
		Ok(())
	}
	fn after_pool_state_change(asset_in: T::AssetId, updated_liquidity: T::Balance) -> DispatchResult {
		Pallet::<T>::ensure_liquidity_limits(asset_in, updated_liquidity)?;
		Ok(())
	}
}