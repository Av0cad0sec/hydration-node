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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// --chain=local
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_scheduler
// --output=scheduler.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_scheduler::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(10_521_000 as u64) // Standard Error: 70_000
			.saturating_add(Weight::from_ref_time(28_878_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(8_527_000 as u64) // Standard Error: 56_000
			.saturating_add(Weight::from_ref_time(22_786_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic_resolved(s: u32) -> Weight {
		Weight::from_ref_time(10_186_000 as u64) // Standard Error: 59_000
			.saturating_add(Weight::from_ref_time(25_010_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_resolved(s: u32) -> Weight {
		Weight::from_ref_time(11_218_000 as u64) // Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(20_848_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named_aborted(s: u32) -> Weight {
		Weight::from_ref_time(10_082_000 as u64) // Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(9_962_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_aborted(s: u32) -> Weight {
		Weight::from_ref_time(10_833_000 as u64) // Standard Error: 17_000
			.saturating_add(Weight::from_ref_time(6_456_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn on_initialize_periodic_named(s: u32) -> Weight {
		Weight::from_ref_time(15_599_000 as u64) // Standard Error: 42_000
			.saturating_add(Weight::from_ref_time(15_250_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_periodic(s: u32) -> Weight {
		Weight::from_ref_time(14_967_000 as u64) // Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(11_488_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize_named(s: u32) -> Weight {
		Weight::from_ref_time(15_970_000 as u64) // Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(9_299_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(15_021_000 as u64) // Standard Error: 30_000
			.saturating_add(Weight::from_ref_time(7_843_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn schedule(s: u32) -> Weight {
		Weight::from_ref_time(20_193_000 as u64) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(127_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn cancel(s: u32) -> Weight {
		Weight::from_ref_time(19_436_000 as u64) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(1_734_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn schedule_named(s: u32) -> Weight {
		Weight::from_ref_time(23_998_000 as u64) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(158_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn cancel_named(s: u32) -> Weight {
		Weight::from_ref_time(22_116_000 as u64) // Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(1_742_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}