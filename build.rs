// SPDX-License-Identifier: GPL-2.0
//
// Build script for scx_horoscope
// Uses scx_rustland_core for simpler BPF integration

fn main() {
    scx_rustland_core::RustLandBuilder::new()
        .unwrap()
        .build()
        .unwrap();
}
