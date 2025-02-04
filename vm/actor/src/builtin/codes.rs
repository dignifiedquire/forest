// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::{multihash::Identity, Cid, Codec, Version};

lazy_static! {
    pub static ref SYSTEM_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/system");
    pub static ref INIT_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/init");
    pub static ref CRON_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/cron");
    pub static ref ACCOUNT_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/account");
    pub static ref POWER_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/storagepower");
    pub static ref MINER_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/storageminer");
    pub static ref MARKET_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/storagemarket");
    pub static ref PAYCH_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/paymentchannel");
    pub static ref MULTISIG_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/multisig");
    pub static ref REWARD_ACTOR_CODE_ID: Cid = make_builtin(b"fil/1/reward");

    // Set of actor code types that can represent external signing parties.
    pub static ref CALLER_TYPES_SIGNABLE: [Cid; 2] =
        [ACCOUNT_ACTOR_CODE_ID.clone(), MULTISIG_ACTOR_CODE_ID.clone()];
}

fn make_builtin(bz: &[u8]) -> Cid {
    Cid::new(Codec::Raw, Version::V1, Identity::digest(bz))
}
