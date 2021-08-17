// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::buffer::Buffer;
use core::{
    fmt,
    ops::{Deref, DerefMut},
    pin::Pin,
};
use s2n_quic_core::path::DEFAULT_MAX_MTU;

// TODO decide on better defaults
const DEFAULT_MESSAGE_COUNT: usize = 1024;

pub struct VecBuffer {
    region: Pin<alloc::boxed::Box<[u8]>>,
    mtu: usize,
}

impl VecBuffer {
    /// Create a contiguous buffer with the specified number of messages
    pub fn new(message_count: usize, mtu: usize) -> Self {
        let len = message_count * mtu;
        let mut region = Pin::new(alloc::vec![0; len].into_boxed_slice());

        // try to mlock the buffer into memory to ensure it doesn't get swapped to disk
        let _ = libc!(mlock(region.as_mut_ptr() as *mut _ as _, region.len()));

        Self { region, mtu }
    }
}

impl Drop for VecBuffer {
    fn drop(&mut self) {
        let _ = libc!(munlock(
            self.region.as_mut_ptr() as *mut _ as _,
            self.region.len()
        ));
    }
}

impl Default for VecBuffer {
    fn default() -> Self {
        // when testing this crate, make buffers smaller to avoid
        // repeated large allocations
        if cfg!(test) {
            Self::new(64, 1200)
        } else {
            Self::new(DEFAULT_MESSAGE_COUNT, DEFAULT_MAX_MTU.into())
        }
    }
}

impl fmt::Debug for VecBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VecBuffer")
            .field("mtu", &self.mtu())
            .field("message_count", &self.len())
            .finish()
    }
}

impl Buffer for VecBuffer {
    fn len(&self) -> usize {
        self.region.len()
    }

    fn mtu(&self) -> usize {
        self.mtu
    }
}

impl Deref for VecBuffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.region.deref()
    }
}

impl DerefMut for VecBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.region.deref_mut()
    }
}
