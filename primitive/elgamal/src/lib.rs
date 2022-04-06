// Copyright (C) 2021-2022 Artree (JP) LLC.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Lifted-ElGamal Pallet
//!
//! ## Overview
//!
//! This is the additive homomorphic encryption which supports one-time multiplication.
//! This library is implemented based on [original paper](https://github.com/herumi/mcl/blob/master/misc/she/she.pdf).

#![cfg_attr(not(feature = "std"), no_std)]

use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Ciphertext {}

impl Ciphertext {
    pub fn new() -> Self {
        Ciphertext {}
    }

    pub fn zero() -> Self {
        Ciphertext {}
    }

    pub fn encrypt() -> Self {
        Ciphertext {}
    }

    pub fn decrypt() -> Self {
        Ciphertext {}
    }

    pub fn add(&self, other: &Self) -> Self {
        Ciphertext {}
    }

    pub fn sub(&self, other: &Self) -> Self {
        Ciphertext {}
    }
}
