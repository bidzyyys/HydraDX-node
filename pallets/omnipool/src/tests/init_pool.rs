use super::*;
use frame_support::assert_noop;

#[test]
fn initialize_pool_should_work_when_called_first_time_with_correct_params() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 100 * ONE),
			(Omnipool::protocol_account(), HDX, 200 * ONE),
		])
		.build()
		.execute_with(|| {
			let stable_amount = 100 * ONE;
			let native_amount = 200 * ONE;

			let stable_price = FixedU128::from_float(0.5);
			let native_price = FixedU128::from_float(1.5);

			// ACT
			assert_ok!(Omnipool::initialize_pool(
				Origin::root(),
				stable_price,
				native_price,
				Permill::from_percent(50),
				Permill::from_percent(50)
			));

			// ASSERT
			// - pool state
			// - native and stable asset states
			// - correct balances
			assert_pool_state!(
				stable_price.checked_mul_int(stable_amount).unwrap()
					+ native_price.checked_mul_int(native_amount).unwrap(),
				native_price.checked_mul_int(native_amount).unwrap()
					* (stable_amount / stable_price.checked_mul_int(stable_amount).unwrap())
					+ stable_amount,
				SimpleImbalance::default()
			);

			assert_asset_state!(
				DAI,
				AssetReserveState {
					reserve: 100000000000000,
					hub_reserve: 50000000000000,
					shares: 100000000000000,
					protocol_shares: 100000000000000,
					cap: 500_000_000_000_000_000,
					tradable: Tradability::default(),
				}
			);
			assert_asset_state!(
				HDX,
				AssetReserveState {
					reserve: 200000000000000,
					hub_reserve: 300000000000000,
					shares: 200000000000000,
					protocol_shares: 200000000000000,
					cap: 500_000_000_000_000_000,
					tradable: Tradability::default(),
				}
			);

			assert_balance!(Omnipool::protocol_account(), DAI, stable_amount);
			assert_balance!(Omnipool::protocol_account(), HDX, native_amount);

			assert_eq!(HubAssetTradability::<Test>::get(), Tradability::SELL);
		});
}

#[test]
fn initialize_pool_should_fail_when_already_initialized() {
	ExtBuilder::default()
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			let stable_price = FixedU128::from_float(0.5);
			let native_price = FixedU128::from_float(1.5);

			assert_noop!(
				Omnipool::initialize_pool(
					Origin::root(),
					stable_price,
					native_price,
					Permill::from_percent(100),
					Permill::from_percent(100)
				),
				Error::<Test>::AssetAlreadyAdded
			);
		});
}

#[test]
fn initialize_pool_should_fail_when_stable_funds_missing_in_pool_account() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![])
		.build()
		.execute_with(|| {
			let stable_price = FixedU128::from_float(0.5);
			let native_price = FixedU128::from_float(1.5);

			assert_noop!(
				Omnipool::initialize_pool(
					Origin::root(),
					stable_price,
					native_price,
					Permill::from_percent(100),
					Permill::from_percent(100)
				),
				Error::<Test>::MissingBalance
			);
		});
}

#[test]
fn initialize_pool_should_fail_when_native_funds_missing_in_pool_account() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(Omnipool::protocol_account(), DAI, 1000 * ONE)])
		.build()
		.execute_with(|| {
			let stable_price = FixedU128::from_float(0.5);
			let native_price = FixedU128::from_float(1.5);

			assert_noop!(
				Omnipool::initialize_pool(
					Origin::root(),
					stable_price,
					native_price,
					Permill::from_percent(100),
					Permill::from_percent(100)
				),
				Error::<Test>::MissingBalance
			);
		});
}

#[test]
fn initialize_pool_should_fail_when_stable_price_is_zero() {
	ExtBuilder::default().build().execute_with(|| {
		let stable_price = FixedU128::from(0);
		let native_price = FixedU128::from(1);

		assert_noop!(
			Omnipool::initialize_pool(
				Origin::root(),
				stable_price,
				native_price,
				Permill::from_percent(100),
				Permill::from_percent(100)
			),
			Error::<Test>::InvalidInitialAssetPrice
		);
	});
}

#[test]
fn initialize_pool_should_fail_when_native_price_is_zero() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![(Omnipool::protocol_account(), DAI, 1000 * ONE)])
		.build()
		.execute_with(|| {
			let stable_price = FixedU128::from(1);
			let native_price = FixedU128::from(0);

			assert_noop!(
				Omnipool::initialize_pool(
					Origin::root(),
					stable_price,
					native_price,
					Permill::from_percent(100),
					Permill::from_percent(100)
				),
				Error::<Test>::InvalidInitialAssetPrice
			);
		});
}

#[test]
fn update_weight_cap_of_native_stable_asset_should_work_when_pool_is_initialized() {
	ExtBuilder::default()
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			assert_ok!(Omnipool::set_asset_weight_cap(
				Origin::root(),
				HDX,
				Permill::from_rational(1u32, 100000u32),
			));
			assert_asset_state!(
				HDX,
				AssetReserveState {
					reserve: 10000000000000000,
					hub_reserve: 10000000000000000,
					shares: 10000000000000000,
					protocol_shares: 10000000000000000,
					cap: 10_000_000_000_000,
					tradable: Tradability::default(),
				}
			);
			assert_ok!(Omnipool::set_asset_weight_cap(
				Origin::root(),
				DAI,
				Permill::from_percent(2u32),
			));
			assert_asset_state!(
				DAI,
				AssetReserveState {
					reserve: 1000000000000000,
					hub_reserve: 500000000000000,
					shares: 1000000000000000,
					protocol_shares: 1000000000000000,
					cap: 20_000_000_000_000_000,
					tradable: Tradability::default(),
				}
			);
		});
}

#[test]
fn initialize_pool_should_fail_when_stable_asset_is_not_registered() {
	ExtBuilder::default()
		.without_stable_asset_in_registry()
		.build()
		.execute_with(|| {
			let stable_price = FixedU128::from_float(0.5);
			let native_price = FixedU128::from_float(1.5);
			assert_noop!(
				Omnipool::initialize_pool(
					Origin::root(),
					stable_price,
					native_price,
					Permill::from_percent(50),
					Permill::from_percent(50)
				),
				Error::<Test>::AssetNotRegistered
			);
		});
}

#[test]
fn rococo() {
	let stable_amount =50_000 * ONE * 1_000_000;
	let native_amount = 936_329_588_000_000_000;
	let dot_amount = 8771_929_825_0000;

	ExtBuilder::default()
		.with_registered_asset(1000)
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), HDX, native_amount),
			(Omnipool::protocol_account(), DAI, stable_amount),
			(Omnipool::protocol_account(), 1000, dot_amount),
		])
		.build()
		.execute_with(|| {

			let native_price = FixedU128::from_inner(1201500000000000);
			let stable_price = FixedU128::from_inner(45_000_000_000);

			dbg!(native_price);
			dbg!(stable_price);

			// ACT
			assert_ok!(Omnipool::initialize_pool(
				Origin::root(),
				stable_price,
				native_price,
				Permill::from_percent(100),
				Permill::from_percent(10)
			));

			assert_pool_state!(
				3374999999982000,
				74999999999600000000000,
				SimpleImbalance::default()
			);

			assert_asset_state!(
				DAI,
				AssetReserveState {
					reserve: 50000000000000000000000,
					hub_reserve: 2250_000_000_000_000,
					shares: 50000000000000000000000,
					protocol_shares: 50000000000000000000000,
					cap: 1000000000000000000,
					tradable: Tradability::default(),
				}
			);
			assert_asset_state!(
				HDX,
				AssetReserveState {
					reserve: native_amount,
					hub_reserve: 1124999999982000,
					shares:936329588000000000 ,
					protocol_shares:936329588000000000 ,
					cap: 100000000000000000,
					tradable: Tradability::default(),
				}
			);

			assert_balance!(Omnipool::protocol_account(), DAI, stable_amount);
			assert_balance!(Omnipool::protocol_account(), HDX, native_amount);

			assert_eq!(HubAssetTradability::<Test>::get(), Tradability::SELL);

			let token_price = FixedU128::from_inner(25_650_000_000_000_000_000);

			assert_ok!(Omnipool::add_token(
				Origin::root(),
				1_000,
				token_price,
				Permill::from_percent(100),
				LP1
			));

			assert_pool_state!(
				5625000000094500,
				125000000002100000000000,
				SimpleImbalance::default()
			);

			assert_asset_state!(
				1000,
				AssetReserveState {
					reserve: dot_amount,
					hub_reserve: 2250_000_000_112_500,
					shares:87719298250000,
					protocol_shares:0,
					cap: 1000000000000000000,
					tradable: Tradability::default(),
				}
			);


		});
}