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

//! Autogenerated weights for zrml_swaps
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-25, STEPS: `10`, REPEAT: 1000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=1000
// --pallet=zrml_swaps
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/weight_template.hbs
// --output=./zrml/swaps/src/weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

///  Trait containing the required functions for weight retrival within
/// zrml_swaps (automatically generated)
pub trait WeightInfoZeitgeist {
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight;
    fn admin_clean_up_pool_cpmm_scalar() -> Weight;
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight;
    fn apply_to_cached_pools_noop(a: u32) -> Weight;
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight;
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight;
    fn end_subsidy_phase(a: u32, b: u32) -> Weight;
    fn execute_arbitrage_buy_burn(a: u32) -> Weight;
    fn execute_arbitrage_mint_sell(a: u32) -> Weight;
    fn execute_arbitrage_skipped(a: u32) -> Weight;
    fn pool_exit(a: u32) -> Weight;
    fn pool_exit_subsidy() -> Weight;
    fn pool_exit_with_exact_asset_amount() -> Weight;
    fn pool_exit_with_exact_pool_amount() -> Weight;
    fn pool_join(a: u32) -> Weight;
    fn pool_join_subsidy() -> Weight;
    fn pool_join_with_exact_asset_amount() -> Weight;
    fn pool_join_with_exact_pool_amount() -> Weight;
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight;
    fn swap_exact_amount_in_cpmm() -> Weight;
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight;
    fn swap_exact_amount_out_cpmm() -> Weight;
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight;
    fn open_pool(a: u32) -> Weight;
    fn close_pool(a: u32) -> Weight;
    fn destroy_pool(a: u32) -> Weight;
}

/// Weight functions for zrml_swaps (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfoZeitgeist for WeightInfo<T> {
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_categorical(a: u32) -> Weight {
        (57_698_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((1_027_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: MarketCommons MarketPool (r:1 w:0)
    // Storage: Swaps Pools (r:1 w:1)
    fn admin_clean_up_pool_cpmm_scalar() -> Weight {
        (52_361_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:7 w:6)
    // Storage: Swaps Pools (r:6 w:0)
    // Storage: Tokens Accounts (r:396 w:396)
    // Storage: System Account (r:6 w:0)
    // Storage: Tokens TotalIssuance (r:64 w:64)
    fn apply_to_cached_pools_execute_arbitrage(a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 905_000
            .saturating_add((3_336_767_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(44 as Weight))
            .saturating_add(T::DbWeight::get().reads((69 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(44 as Weight))
            .saturating_add(T::DbWeight::get().writes((67 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps PoolsCachedForArbitrage (r:7 w:6)
    fn apply_to_cached_pools_noop(a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 7_000
            .saturating_add((10_520_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:0)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn destroy_pool_in_subsidy_phase(a: u32) -> Weight {
        (44_428_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((25_843_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: Tokens Accounts (r:46 w:22)
    // Storage: System Account (r:2 w:1)
    fn distribute_pool_share_rewards(a: u32, b: u32) -> Weight {
        (29_974_000 as Weight)
            // Standard Error: 77_000
            .saturating_add((43_084_000 as Weight).saturating_mul(a as Weight))
            // Standard Error: 77_000
            .saturating_add((50_328_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(b as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:11 w:10)
    // Storage: Tokens Accounts (r:22 w:22)
    // Storage: System Account (r:11 w:11)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:0)
    fn end_subsidy_phase(a: u32, b: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 194_000
            .saturating_add((36_944_000 as Weight).saturating_mul(a as Weight))
            // Standard Error: 1_231_000
            .saturating_add((175_983_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().reads((9 as Weight).saturating_mul(b as Weight)))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes((9 as Weight).saturating_mul(b as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_buy_burn(a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 18_000
            .saturating_add((52_561_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:2 w:1)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    fn execute_arbitrage_mint_sell(a: u32) -> Weight {
        (19_956_000 as Weight)
            // Standard Error: 16_000
            .saturating_add((48_596_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:2 w:0)
    fn execute_arbitrage_skipped(a: u32) -> Weight {
        (16_944_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((9_053_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    // Storage: System Account (r:1 w:0)
    fn pool_exit(a: u32) -> Weight {
        (47_671_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((39_937_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    fn pool_exit_subsidy() -> Weight {
        (86_760_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_asset_amount() -> Weight {
        (184_191_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_exit_with_exact_pool_amount() -> Weight {
        (184_751_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:5 w:5)
    fn pool_join(a: u32) -> Weight {
        (42_632_000 as Weight)
            // Standard Error: 14_000
            .saturating_add((36_906_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:1 w:1)
    // Storage: Swaps SubsidyProviders (r:1 w:1)
    fn pool_join_subsidy() -> Weight {
        (88_700_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_asset_amount() -> Weight {
        (165_460_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens TotalIssuance (r:1 w:1)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn pool_join_with_exact_pool_amount() -> Weight {
        (162_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn clean_up_pool_categorical_without_reward_distribution(a: u32) -> Weight {
        (11_083_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((581_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_in_cpmm() -> Weight {
        (223_361_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:3 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_in_rikiddo(a: u32) -> Weight {
        (140_111_000 as Weight)
            // Standard Error: 10_000
            .saturating_add((25_517_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:4)
    // Storage: System Account (r:1 w:0)
    // Storage: Swaps PoolsCachedForArbitrage (r:0 w:1)
    fn swap_exact_amount_out_cpmm() -> Weight {
        (217_140_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:0)
    // Storage: Tokens Accounts (r:4 w:3)
    // Storage: Tokens TotalIssuance (r:2 w:1)
    // Storage: RikiddoSigmoidFeeMarketEma RikiddoPerPool (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn swap_exact_amount_out_rikiddo(a: u32) -> Weight {
        (73_337_000 as Weight)
            // Standard Error: 10_000
            .saturating_add((45_756_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn open_pool(a: u32) -> Weight {
        (26_996_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((880_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:1)
    fn close_pool(a: u32) -> Weight {
        (25_651_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((650_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Swaps Pools (r:1 w:1)
    // Storage: Tokens Accounts (r:2 w:2)
    // Storage: Tokens TotalIssuance (r:2 w:2)
    fn destroy_pool(a: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 14_000
            .saturating_add((33_316_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
}
