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

//! Autogenerated weights for module_transaction_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `295f33c1d5e7`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_transaction_payment.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_transaction_payment::WeightInfo for WeightInfo<T> {
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: TransactionPayment AlternativeFeeSwapPath (r:0 w:1)
	fn set_alternative_fee_swap_path() -> Weight {
		// Minimum execution time: 27_545 nanoseconds.
		Weight::from_ref_time(28_496_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: TransactionPayment PoolSize (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:4 w:0)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: StableAsset Pools (r:1 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: TransactionPayment TokenExchangeRate (r:0 w:1)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	fn enable_charge_fee_pool() -> Weight {
		// Minimum execution time: 90_958 nanoseconds.
		Weight::from_ref_time(94_832_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: TransactionPayment TokenExchangeRate (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: TransactionPayment SwapBalanceThreshold (r:0 w:1)
	// Storage: TransactionPayment GlobalFeeSwapPath (r:0 w:1)
	// Storage: TransactionPayment PoolSize (r:0 w:1)
	fn disable_charge_fee_pool() -> Weight {
		// Minimum execution time: 70_984 nanoseconds.
		Weight::from_ref_time(72_397_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	fn with_fee_path() -> Weight {
		// Minimum execution time: 6_941 nanoseconds.
		Weight::from_ref_time(7_433_000)
	}
	fn with_fee_currency() -> Weight {
		// Minimum execution time: 7_524 nanoseconds.
		Weight::from_ref_time(8_049_000)
	}
	fn with_fee_aggregated_path() -> Weight {
		// Minimum execution time: 8_132 nanoseconds.
		Weight::from_ref_time(8_829_000)
	}
	fn with_fee_paid_by() -> Weight {
		// Minimum execution time: 5_224 nanoseconds.
		Weight::from_ref_time(5_403_000)
	}
	// Storage: TransactionPayment NextFeeMultiplier (r:1 w:1)
	fn on_finalize() -> Weight {
		// Minimum execution time: 9_148 nanoseconds.
		Weight::from_ref_time(9_594_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
