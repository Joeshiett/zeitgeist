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

//! Fuzz test: Rikiddo is called with calculated fee
//! -> force EmaMarketVolume, cost, price, all_prices
#![allow(
    // Mocks are only used for fuzzing and unit tests
    clippy::integer_arithmetic
)]
#![allow(clippy::type_complexity)]
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

mod shared;
use shared::fixed_from_u128;
use substrate_fixed::{types::extra::U33, FixedI128, FixedU128};
use zrml_rikiddo::{
    traits::{Lmsr, RikiddoMV},
    types::{
        EmaMarketVolume, FeeSigmoid, RikiddoSigmoidMV, Timespan, TimestampedVolume, UnixTimestamp,
    },
};

fuzz_target!(|data: Data| {
    let asset_balances_fixed: Vec<FixedU128<U33>> =
        data.asset_balances.iter().map(|e| fixed_from_u128(*e)).collect();
    let price_for_fixed = fixed_from_u128(data.price_for);
    let mut rikiddo = data.rikiddo;
    // Use reasonable parameters for fee calculation and EMA
    rikiddo.fees = Default::default();
    rikiddo.ma_short = Default::default();
    rikiddo.ma_long = Default::default();
    rikiddo.ma_short.config.ema_period = Timespan::Seconds(0);
    rikiddo.ma_long.config.ema_period = Timespan::Seconds(1);

    // Initialize ma_short and ma_long ema
    for (idx, volume) in data.update_volumes.iter().enumerate() {
        let timestamped_volume = TimestampedVolume {
            timestamp: (idx / 2) as UnixTimestamp,
            volume: fixed_from_u128(*volume),
        };
        let _ = rikiddo.update_volume(&timestamped_volume);
    }

    // Calculate cost and price using calculated fee from r = ma_short / ma_long
    let _ = rikiddo.cost(&asset_balances_fixed[..]);
    let _ = rikiddo.price(&asset_balances_fixed[..], &price_for_fixed);
    let _ = rikiddo.all_prices(&asset_balances_fixed[..]);
});

#[derive(Debug, Arbitrary)]
struct Data {
    asset_balances: [u128; 8],
    price_for: u128,
    rikiddo: RikiddoSigmoidMV<
        FixedU128<U33>,
        FixedI128<U33>,
        FeeSigmoid<FixedI128<U33>>,
        EmaMarketVolume<FixedU128<U33>>,
    >,
    update_volumes: [u128; 5],
}
