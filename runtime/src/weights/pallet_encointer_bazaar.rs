
//! Autogenerated weights for `pallet_encointer_bazaar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-node-notee
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_bazaar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_encointer_bazaar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_encointer_bazaar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_bazaar::WeightInfo for WeightInfo<T> {
	// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	fn create_business() -> Weight {
		Weight::from_ref_time(26_500_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	fn update_business() -> Weight {
		Weight::from_ref_time(24_200_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	fn delete_business() -> Weight {
		Weight::from_ref_time(27_900_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerBazaar BusinessRegistry (r:1 w:1)
	// Storage: EncointerBazaar OfferingRegistry (r:0 w:1)
	fn create_offering() -> Weight {
		Weight::from_ref_time(29_600_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: EncointerBazaar OfferingRegistry (r:1 w:1)
	fn update_offering() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EncointerBazaar OfferingRegistry (r:1 w:1)
	fn delete_offering() -> Weight {
		Weight::from_ref_time(23_500_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}