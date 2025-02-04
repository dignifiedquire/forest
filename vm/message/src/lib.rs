// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod message_receipt;
mod signed_message;
mod unsigned_message;

pub use message_receipt::*;
pub use signed_message::*;
pub use unsigned_message::*;

use address::Address;
use num_bigint::BigUint;
use vm::{MethodNum, Serialized, TokenAmount};

pub trait Message {
    /// Returns the from address of the message
    fn from(&self) -> &Address;
    /// Returns the destination address of the message
    fn to(&self) -> &Address;
    /// Returns the message sequence or nonce
    fn sequence(&self) -> u64;
    /// Returns the amount sent in message
    fn value(&self) -> &TokenAmount;
    /// Returns the method number to be called
    fn method_num(&self) -> &MethodNum;
    /// Returns the encoded parameters for the method call
    fn params(&self) -> &Serialized;
    /// gas_price returns gas price for the message
    fn gas_price(&self) -> &BigUint;
    /// Returns the gas limit for the message
    fn gas_limit(&self) -> &BigUint;
    /// Returns the required funds for the message
    fn required_funds(&self) -> BigUint;
}
