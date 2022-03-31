use super::*;

#[test]
fn simple_buy_works() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(Omnipool::protocol_account(), 100, 2000 * ONE),
			(Omnipool::protocol_account(), 200, 2000 * ONE),
			(LP1, 100, 1000 * ONE),
		])
		.build()
		.execute_with(|| {
			let dai_amount = 1000 * ONE;
			let price = FixedU128::from_float(0.5);
			init_omnipool(dai_amount, price);

			let token_amount = 2000 * ONE;
			let token_price = FixedU128::from_float(0.65);

			assert_ok!(Omnipool::add_token(Origin::root(), 100, token_amount, token_price,));

			assert_ok!(Omnipool::add_token(Origin::root(), 200, token_amount, token_price,));

			let liq_added = 400 * ONE;
			assert_ok!(Omnipool::add_liquidity(Origin::signed(LP1), 100, liq_added));

			let buy_amount = 50 * ONE;
			let max_limit = 100 * ONE;

			assert_eq!(Tokens::free_balance(100, &LP1), 600 * ONE);
			assert_eq!(Tokens::free_balance(200, &Omnipool::protocol_account()), 2000 * ONE);

			assert_ok!(Omnipool::buy(Origin::signed(LP1), 200, 100, buy_amount, max_limit));

			assert_eq!(Tokens::free_balance(100, &LP1), 549790794979080);
			assert_eq!(Tokens::free_balance(200, &LP1), buy_amount);
			assert_eq!(Tokens::free_balance(LRNA, &Omnipool::protocol_account()), 13360 * ONE);
			assert_eq!(
				Tokens::free_balance(100, &Omnipool::protocol_account()),
				2450209205020920
			);
			assert_eq!(Tokens::free_balance(200, &Omnipool::protocol_account()), 1950 * ONE);

			check_state!(13_360 * ONE, 27_320 * ONE, SimpleImbalance::default());

			check_asset_state!(
				100,
				AssetState {
					reserve: 2450209205020920,
					hub_reserve: 1526666666666667,
					shares: 2400 * ONE,
					protocol_shares: 2000 * ONE,
					tvl: 3120 * ONE
				}
			);
			check_asset_state!(
				200,
				AssetState {
					reserve: 1950 * ONE,
					hub_reserve: 1333333333333333,
					shares: 2000 * ONE,
					protocol_shares: 2000 * ONE,
					tvl: 2000 * ONE
				}
			);
		});
}