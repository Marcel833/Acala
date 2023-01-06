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

//! Autogenerated weights for module_dex_oracle
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

/// Weight functions for module_dex_oracle.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_dex_oracle::WeightInfo for WeightInfo<T> {
	// Storage: Aura CurrentSlot (r:1 w:1)
	// Storage: Aura Authorities (r:1 w:0)
	// Storage: DexOracle AveragePrices (r:1 w:0)
	// Storage: System ParentHash (r:0 w:1)
	// Storage: System Digest (r:0 w:1)
	// Storage: System BlockHash (r:0 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:0 w:1)
	// Storage: Timestamp Now (r:0 w:1)
	// Storage: Timestamp DidUpdate (r:0 w:1)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: DexOracle Cumulatives (r:1 w:1)
	/// The range of component `n` is `[0, 3]`.
	/// The range of component `u` is `[0, 3]`.
	fn on_initialize_with_update_average_prices(n: u32, u: u32, ) -> Weight {
		// Minimum execution time: 15_308 nanoseconds.
		Weight::from_ref_time(15_887_000)
			// Standard Error: 135_031
			.saturating_add(Weight::from_ref_time(11_077_514).saturating_mul(n.into()))
			// Standard Error: 135_031
			.saturating_add(Weight::from_ref_time(4_266_490).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(u.into())))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	// Storage: Dex LiquidityPool (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: DexOracle Cumulatives (r:0 w:1)
	fn enable_average_price() -> Weight {
		// Minimum execution time: 18_395 nanoseconds.
		Weight::from_ref_time(19_074_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	// Storage: DexOracle Cumulatives (r:0 w:1)
	fn disable_average_price() -> Weight {
		// Minimum execution time: 11_206 nanoseconds.
		Weight::from_ref_time(11_562_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: DexOracle AveragePrices (r:1 w:1)
	fn update_average_price_interval() -> Weight {
		// Minimum execution time: 10_677 nanoseconds.
		Weight::from_ref_time(11_025_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
