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

//! Autogenerated weights for module_session_manager
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

/// Weight functions for module_session_manager.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_session_manager::WeightInfo for WeightInfo<T> {
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: SessionManager SessionDuration (r:1 w:0)
	// Storage: SessionManager DurationOffset (r:1 w:0)
	// Storage: SessionManager SessionDurationChanges (r:0 w:1)
	fn schedule_session_duration() -> Weight {
		// Minimum execution time: 19_974 nanoseconds.
		Weight::from_ref_time(20_527_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: SessionManager SessionDurationChanges (r:1 w:1)
	fn on_initialize_skip() -> Weight {
		// Minimum execution time: 4_378 nanoseconds.
		Weight::from_ref_time(4_702_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: SessionManager SessionDurationChanges (r:1 w:1)
	// Storage: SessionManager DurationOffset (r:0 w:1)
	// Storage: SessionManager SessionDuration (r:0 w:1)
	fn on_initialize() -> Weight {
		// Minimum execution time: 5_462 nanoseconds.
		Weight::from_ref_time(5_761_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: SessionManager DurationOffset (r:1 w:0)
	// Storage: SessionManager SessionDuration (r:1 w:0)
	fn estimate_current_session_progress() -> Weight {
		// Minimum execution time: 3_897 nanoseconds.
		Weight::from_ref_time(4_215_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: SessionManager DurationOffset (r:1 w:0)
	// Storage: SessionManager SessionDuration (r:1 w:0)
	fn estimate_next_session_rotation() -> Weight {
		// Minimum execution time: 3_973 nanoseconds.
		Weight::from_ref_time(4_256_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
