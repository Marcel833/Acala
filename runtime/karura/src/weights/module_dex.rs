// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_dex
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `187e78510d7a`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_dex.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex::WeightInfo for WeightInfo<T> {
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn enable_trading_pair() -> Weight {
		// Minimum execution time: 16_977 nanoseconds.
		Weight::from_ref_time(17_553_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn disable_trading_pair() -> Weight {
		// Minimum execution time: 17_929 nanoseconds.
		Weight::from_ref_time(18_668_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:0)
	fn list_provisioning() -> Weight {
		// Minimum execution time: 23_623 nanoseconds.
		Weight::from_ref_time(24_291_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn update_provisioning_parameters() -> Weight {
		// Minimum execution time: 10_493 nanoseconds.
		Weight::from_ref_time(10_916_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Dex InitialShareExchangeRates (r:0 w:1)
	fn end_provisioning() -> Weight {
		// Minimum execution time: 46_963 nanoseconds.
		Weight::from_ref_time(48_627_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn add_provision() -> Weight {
		// Minimum execution time: 73_832 nanoseconds.
		Weight::from_ref_time(75_701_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:2 w:1)
	// Storage: Dex InitialShareExchangeRates (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn claim_dex_share() -> Weight {
		// Minimum execution time: 66_957 nanoseconds.
		Weight::from_ref_time(68_958_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 90_364 nanoseconds.
		Weight::from_ref_time(92_276_000)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	fn add_liquidity_and_stake() -> Weight {
		// Minimum execution time: 127_320 nanoseconds.
		Weight::from_ref_time(132_307_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Tokens Accounts (r:3 w:3)
	// Storage: System Account (r:1 w:1)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 85_535 nanoseconds.
		Weight::from_ref_time(88_624_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: System Account (r:2 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn remove_liquidity_by_unstake() -> Weight {
		// Minimum execution time: 136_509 nanoseconds.
		Weight::from_ref_time(139_642_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_supply(u: u32, ) -> Weight {
		// Minimum execution time: 69_719 nanoseconds.
		Weight::from_ref_time(51_311_842)
			// Standard Error: 64_355
			.saturating_add(Weight::from_ref_time(10_813_197).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	/// The range of component `u` is `[2, 4]`.
	fn swap_with_exact_target(u: u32, ) -> Weight {
		// Minimum execution time: 70_138 nanoseconds.
		Weight::from_ref_time(50_974_070)
			// Standard Error: 65_686
			.saturating_add(Weight::from_ref_time(10_915_254).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex InitialShareExchangeRates (r:1 w:0)
	// Storage: Dex ProvisioningPool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn refund_provision() -> Weight {
		// Minimum execution time: 75_756 nanoseconds.
		Weight::from_ref_time(77_482_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Dex TradingPairStatuses (r:1 w:1)
	fn abort_provisioning() -> Weight {
		// Minimum execution time: 21_706 nanoseconds.
		Weight::from_ref_time(22_618_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
