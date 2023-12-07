
//! Autogenerated weights for `pallet_transfer_allowlist`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_transfer_allowlist
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_transfer_allowlist.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_transfer_allowlist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_transfer_allowlist::WeightInfo for WeightInfo<T> {
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn add_transfer_allowance_no_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `3674`
		// Minimum execution time: 88_506_000 picoseconds.
		Weight::from_parts(89_689_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn add_transfer_allowance_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `630`
		//  Estimated: `3674`
		// Minimum execution time: 91_031_000 picoseconds.
		Weight::from_parts(92_643_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn add_allowance_delay_no_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `213`
		//  Estimated: `3556`
		// Minimum execution time: 20_138_000 picoseconds.
		Weight::from_parts(20_719_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn add_allowance_delay_existing_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `336`
		//  Estimated: `3556`
		// Minimum execution time: 22_593_000 picoseconds.
		Weight::from_parts(23_435_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn toggle_allowance_delay_once_future_modifiable() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `3556`
		// Minimum execution time: 23_073_000 picoseconds.
		Weight::from_parts(23_705_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn update_allowance_delay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `3556`
		// Minimum execution time: 22_752_000 picoseconds.
		Weight::from_parts(23_614_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn purge_allowance_delay_no_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `3556`
		// Minimum execution time: 22_462_000 picoseconds.
		Weight::from_parts(23_294_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn purge_allowance_delay_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `344`
		//  Estimated: `3556`
		// Minimum execution time: 23_765_000 picoseconds.
		Weight::from_parts(24_436_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:0)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	fn remove_transfer_allowance_delay_present() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `3596`
		// Minimum execution time: 34_514_000 picoseconds.
		Weight::from_parts(35_316_000, 0)
			.saturating_add(Weight::from_parts(0, 3596))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:0)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	fn remove_transfer_allowance_no_delay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `434`
		//  Estimated: `3596`
		// Minimum execution time: 34_826_000 picoseconds.
		Weight::from_parts(35_466_000, 0)
			.saturating_add(Weight::from_parts(0, 3596))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn purge_transfer_allowance_no_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `811`
		//  Estimated: `3674`
		// Minimum execution time: 82_605_000 picoseconds.
		Weight::from_parts(84_027_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: TransferAllowList AccountCurrencyTransferAllowance (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferAllowance (max_values: None, max_size: Some(131), added: 2606, mode: MaxEncodedLen)
	/// Storage: Fees FeeBalances (r:1 w:0)
	/// Proof: Fees FeeBalances (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: TransferAllowList AccountCurrencyTransferCountDelay (r:1 w:1)
	/// Proof: TransferAllowList AccountCurrencyTransferCountDelay (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn purge_transfer_allowance_remaining_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `850`
		//  Estimated: `3674`
		// Minimum execution time: 82_284_000 picoseconds.
		Weight::from_parts(83_326_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
