// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod actor_code;

pub use self::actor_code::*;

use address::Address;
use cid::Cid;
use clock::ChainEpoch;
use crypto::{DomainSeparationTag, Signature};
use forest_encoding::Cbor;
use ipld_blockstore::BlockStore;
use message::UnsignedMessage;
use vm::{
    ExitCode, MethodNum, PieceInfo, PoStVerifyInfo, RegisteredProof, SealVerifyInfo, Serialized,
    TokenAmount,
};

pub struct Randomness; // TODO

/// Runtime is the VM's internal runtime object.
/// this is everything that is accessible to actors, beyond parameters.
pub trait Runtime<BS: BlockStore> {
    /// Information related to the current message being executed.
    fn message(&self) -> &UnsignedMessage;

    /// The current chain epoch number. The genesis block has epoch zero.
    fn curr_epoch(&self) -> ChainEpoch;

    /// Validates the caller against some predicate.
    /// Exported actor methods must invoke at least one caller validation before returning.
    fn validate_immediate_caller_accept_any(&self);
    fn validate_immediate_caller_is<'a, I>(&self, addresses: I)
    where
        I: Iterator<Item = &'a Address>;
    fn validate_immediate_caller_type<'a, I>(&self, types: I)
    where
        I: Iterator<Item = &'a Cid>;

    /// The balance of the receiver.
    fn current_balance(&self) -> TokenAmount;

    /// Resolves an address of any protocol to an ID address (via the Init actor's table).
    /// This allows resolution of externally-provided SECP, BLS, or actor addresses to the canonical form.
    /// If the argument is an ID address it is returned directly.
    fn resolve_address(&self, address: &Address) -> Option<Address>;

    /// Look up the code ID at an actor address.
    fn get_actor_code_cid(&self, addr: &Address) -> Option<Cid>;

    /// Randomness returns a (pseudo)random byte array drawing from a
    /// random beacon at a given epoch and incorporating reequisite entropy
    fn get_randomness(
        personalization: DomainSeparationTag,
        rand_epoch: ChainEpoch,
        entropy: &[u8],
    ) -> Randomness;

    /// Initializes the state object.
    /// This is only valid in a constructor function and when the state has not yet been initialized.
    fn create<C: Cbor>(&self, obj: &C);

    /// Loads a readonly copy of the state of the receiver into the argument.
    ///
    /// Any modification to the state is illegal and will result in an abort.
    fn state<C: Cbor>(&self) -> C;

    /// Loads a mutable version of the state into the `obj` argument and protects
    /// the execution from side effects (including message send).
    ///
    /// The second argument is a function which allows the caller to mutate the state.
    /// The return value from that function will be returned from the call to Transaction().
    ///
    /// If the state is modified after this function returns, execution will abort.
    ///
    /// The gas cost of this method is that of a Store.Put of the mutated state object.
    fn transaction<C: Cbor, R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut C) -> R;

    /// Returns reference to blockstore
    fn store(&self) -> &BS;

    /// Sends a message to another actor, returning the exit code and return value envelope.
    /// If the invoked method does not return successfully, its state changes (and that of any messages it sent in turn)
    /// will be rolled back.
    fn send<SR: Cbor>(
        &self,
        to: &Address,
        method: MethodNum,
        params: &Serialized,
        value: &TokenAmount,
    ) -> (SR, ExitCode);

    /// Halts execution upon an error from which the receiver cannot recover. The caller will receive the exitcode and
    /// an empty return value. State changes made within this call will be rolled back.
    /// This method does not return.
    /// The message and args are for diagnostic purposes and do not persist on chain.
    fn abort(&self, exit_code: ExitCode, msg: String);

    /// Computes an address for a new actor. The returned address is intended to uniquely refer to
    /// the actor even in the event of a chain re-org (whereas an ID-address might refer to a
    /// different actor after messages are re-ordered).
    /// Always an ActorExec address.
    fn new_actor_address(&self) -> Address;

    /// Creates an actor with code `codeID` and address `address`, with empty state. May only be called by Init actor.
    fn create_actor(&self, code_id: &Cid, address: &Address);

    /// Deletes the executing actor from the state tree. May only be called by the actor itself.
    fn delete_actor(&self);

    /// Provides the system call interface.
    fn syscalls(&self) -> Syscalls {
        // If syscalls need to be handled dynamically, switch to returning Box<dyn Syscalls>
        // If that is changed to a trait
        Syscalls
    }
}

/// Message information available to the actor about executing message.
pub trait MessageInfo {
    // The address of the immediate calling actor. Always an ID-address.
    fn caller(&self) -> Address;

    // The address of the actor receiving the message. Always an ID-address.
    fn receiver(&self) -> Address;

    // The value attached to the message being processed, implicitly added to current_balance() before method invocation.
    fn value_received(&self) -> TokenAmount;
}

/// Pure functions implemented as primitives by the runtime.
// TODO this may have to be a trait impl to handle dynamically
pub struct Syscalls;

impl Syscalls {
    /// Verifies that a signature is valid for an address and plaintext.
    pub fn verify_signature(
        _signature: Signature,
        _signer: Address,
        _plaintext: &[u8],
    ) -> Result<(), &'static str> {
        // TODO
        todo!()
    }
    /// Hashes input data using blake2b with 256 bit output.
    pub fn hash_blake_2b(_data: &[u8]) -> [u8; 32] {
        // TODO
        todo!()
    }
    /// Computes an unsealed sector CID (CommD) from its constituent piece CIDs (CommPs) and sizes.
    pub fn compute_unsealed_sector_cid(
        _reg: RegisteredProof,
        _pieces: &[PieceInfo],
    ) -> Result<Cid, &'static str> {
        // TODO
        todo!()
    }
    /// Verifies a sector seal proof.
    pub fn verify_seal(_vi: SealVerifyInfo) -> Result<(), &'static str> {
        // TODO
        todo!()
    }
    /// Verifies a proof of spacetime.
    pub fn verify_post(_vi: PoStVerifyInfo) -> Result<(), &'static str> {
        // TODO
        todo!()
    }
    /// Verifies that two block headers provide proof of a consensus fault:
    /// - both headers mined by the same actor
    /// - headers are different
    /// - first header is of the same or lower epoch as the second
    /// - at least one of the headers appears in the current chain at or after epoch `earliest`
    /// - the headers provide evidence of a fault (see the spec for the different fault types).
    /// The parameters are all serialized block headers. The third "extra" parameter is consulted only for
    /// the "parent grinding fault", in which case it must be the sibling of h1 (same parent tipset) and one of the
    /// blocks in the parent of h2 (i.e. h2's grandparent).
    /// Returns nil and an error if the headers don't prove a fault.
    pub fn verify_consensus_fault(
        _h1: &[u8],
        _h2: &[u8],
        _extra: &[u8],
        _earliest: ChainEpoch,
    ) -> Result<ConsensusFault, &'static str> {
        // TODO
        todo!()
    }
}

/// Result of checking two headers for a consensus fault.
pub struct ConsensusFault {
    /// Address of the miner at fault (always an ID address).
    pub target: Address,
    /// Epoch of the fault, which is the higher epoch of the two blocks causing it.
    pub epoch: ChainEpoch,
    /// Type of fault.
    pub fault_type: ConsensusFaultType,
}

/// Consensus fault types in VM.
pub enum ConsensusFaultType {
    DoubleForkMining = 1,
    ParentGrinding = 2,
    TimeOffsetMining = 3,
}
