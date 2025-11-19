// SPDX-License-Identifier: GPL-2.0

// Suppress warnings from auto-generated BPF bindings
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bpf_intf.rs"));
