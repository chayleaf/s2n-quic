// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use criterion::Criterion;

mod crypto;
mod frame;
mod packet;
mod varint;

pub fn benchmarks(c: &mut Criterion) {
    crypto::benchmarks(c);
    frame::benchmarks(c);
    packet::benchmarks(c);
    varint::benchmarks(c);
}
