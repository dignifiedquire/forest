// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use num_bigint::BigUint;
use vm::{ExitCode, Serialized};

/// Result of a state transition from a message
#[derive(PartialEq, Clone)]
pub struct MessageReceipt {
    pub exit_code: ExitCode,
    pub return_data: Serialized,
    pub gas_used: BigUint,
}
