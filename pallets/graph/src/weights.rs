// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_graph
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-24, STEPS: `20`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/mrc-collator
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_graph
// --extrinsic
// *
// --steps
// 20
// --repeat
// 5
// --output
// ./pallets/graph/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_graph.
pub trait WeightInfo {
	fn add_node(n: u32, ) -> Weight;
	fn follow(n: u32, ) -> Weight;
	fn unfollow(n: u32, ) -> Weight;
}

/// Weights for pallet_graph using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Graph Nodes (r:1 w:1)
	// Storage: Graph NodeCount (r:1 w:1)
	fn add_node(n: u32, ) -> Weight {
		(12_868_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Graph Nodes (r:2 w:0)
	// Storage: Graph Graph (r:1 w:1)
	// Storage: Graph EdgeCount (r:1 w:1)
	fn follow(n: u32, ) -> Weight {
		(51_990_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Graph Nodes (r:2 w:0)
	// Storage: Graph EdgeCount (r:1 w:1)
	// Storage: Graph Graph (r:1 w:1)
	fn unfollow(n: u32, ) -> Weight {
		(52_544_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Graph Nodes (r:1 w:1)
	// Storage: Graph NodeCount (r:1 w:1)
	fn add_node(n: u32, ) -> Weight {
		(12_868_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Graph Nodes (r:2 w:0)
	// Storage: Graph Graph (r:1 w:1)
	// Storage: Graph EdgeCount (r:1 w:1)
	fn follow(n: u32, ) -> Weight {
		(51_990_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Graph Nodes (r:2 w:0)
	// Storage: Graph EdgeCount (r:1 w:1)
	// Storage: Graph Graph (r:1 w:1)
	fn unfollow(n: u32, ) -> Weight {
		(52_544_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}