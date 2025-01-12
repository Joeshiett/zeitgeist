// Copyright 2021-2022 Zeitgeist PM LLC.
//
// This file is part of Zeitgeist.
//
// Zeitgeist is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zeitgeist is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zeitgeist. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for pallet_multisig (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::weights::WeightInfo for WeightInfo<T> {
    fn as_multi_threshold_1(z: u32) -> Weight {
        Weight::from_ref_time(40_350_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(1_000).saturating_mul(z.into()))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create(_s: u32, z: u32) -> Weight {
        Weight::from_ref_time(98_340_000)
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(1_000).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn as_multi_create_store(s: u32, z: u32) -> Weight {
        Weight::from_ref_time(89_465_000)
            // Standard Error: 19_000
            .saturating_add(Weight::from_ref_time(14_000).saturating_mul(s.into()))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    fn as_multi_approve(s: u32, z: u32) -> Weight {
        Weight::from_ref_time(48_826_000)
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(139_000).saturating_mul(s.into()))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(2_000).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:1)
    fn as_multi_approve_store(s: u32, z: u32) -> Weight {
        Weight::from_ref_time(73_692_000)
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(110_000).saturating_mul(s.into()))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn as_multi_complete(s: u32, z: u32) -> Weight {
        Weight::from_ref_time(91_645_000)
            // Standard Error: 13_000
            .saturating_add(Weight::from_ref_time(105_000).saturating_mul(s.into()))
            // Standard Error: 0
            .saturating_add(Weight::from_ref_time(3_000).saturating_mul(z.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
    fn approve_as_multi_create(s: u32) -> Weight {
        Weight::from_ref_time(59_874_000)
            // Standard Error: 6_000
            .saturating_add(Weight::from_ref_time(182_000).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:0)
    fn approve_as_multi_approve(s: u32) -> Weight {
        Weight::from_ref_time(43_800_000)
            // Standard Error: 9_000
            .saturating_add(Weight::from_ref_time(173_000).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn approve_as_multi_complete(s: u32) -> Weight {
        Weight::from_ref_time(103_694_000)
            // Standard Error: 17_000
            .saturating_add(Weight::from_ref_time(194_000).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    // Storage: MultiSig Multisigs (r:1 w:1)
    // Storage: MultiSig Calls (r:1 w:1)
    fn cancel_as_multi(s: u32) -> Weight {
        Weight::from_ref_time(89_339_000)
            // Standard Error: 25_000
            .saturating_add(Weight::from_ref_time(197_000).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}
