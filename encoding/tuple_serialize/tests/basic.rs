// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use tuple_serialize::TupleSerialize;
use serde;

#[derive(TupleSerialize)]
pub struct Serialize {
    field1: String,
    field2: u8,
    field3: (u8, String),
}

fn main() {}
