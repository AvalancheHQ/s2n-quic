// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Subset of the `bench` target that only contains CPU-bound benchmarks.
//!
//! The `bench` target also covers stream throughput and handshakes, which rely on real sockets and
//! multiple threads. Those are useful locally but can't be measured deterministically in CI, so
//! this target is what continuous benchmarking (CodSpeed) runs.

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmarks(c: &mut Criterion) {
    s2n_quic_dc_benches::crypto::benchmarks(c);
    s2n_quic_dc_benches::datagram::benchmarks(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
