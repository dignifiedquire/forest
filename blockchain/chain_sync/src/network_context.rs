// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::network_handler::RPCReceiver;
use async_std::future;
use async_std::prelude::*;
use async_std::sync::{Receiver, Sender};
use blocks::{TipSetKeys, Tipset};
use forest_libp2p::{
    blocksync::{BlockSyncRequest, BlockSyncResponse},
    rpc::{RPCEvent, RPCRequest, RPCResponse, RequestId},
    NetworkEvent, NetworkMessage,
};
use libp2p::core::PeerId;
use log::trace;
use std::time::Duration;

/// Timeout for response from an RPC request
const RPC_TIMEOUT: u64 = 5;

/// Context used in chain sync to handle network requests
pub struct SyncNetworkContext {
    /// Channel to send network messages through p2p service
    network_send: Sender<NetworkMessage>,

    /// Handles sequential request ID enumeration for requests
    request_id: RequestId,

    /// Receiver channel for BlockSync responses
    rpc_receiver: RPCReceiver,

    /// Receiver channel for network events
    pub receiver: Receiver<NetworkEvent>,
}

impl SyncNetworkContext {
    pub fn new(
        network_send: Sender<NetworkMessage>,
        rpc_receiver: RPCReceiver,
        receiver: Receiver<NetworkEvent>,
    ) -> Self {
        Self {
            network_send,
            rpc_receiver,
            receiver,
            request_id: 0,
        }
    }

    /// Send a blocksync request for only block headers (ignore messages)
    pub async fn blocksync_headers(
        &mut self,
        peer_id: PeerId,
        tsk: &TipSetKeys,
        count: u64,
    ) -> Result<Vec<Tipset>, String> {
        let bs_res = self
            .blocksync_request(
                peer_id,
                BlockSyncRequest {
                    start: tsk.cids().to_vec(),
                    request_len: count,
                    options: 1,
                },
            )
            .await?;

        let ts = bs_res.into_result()?;
        ts.iter()
            .map(|fts| fts.tipset().map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()
    }

    /// Send a blocksync request to the network and await response
    pub async fn blocksync_request(
        &mut self,
        peer_id: PeerId,
        request: BlockSyncRequest,
    ) -> Result<BlockSyncResponse, &'static str> {
        trace!("Sending BlockSync Request {:?}", request);
        let rpc_res = self
            .send_rpc_request(peer_id, RPCRequest::BlockSync(request))
            .await?;

        if let RPCResponse::BlockSync(bs_res) = rpc_res {
            Ok(bs_res)
        } else {
            Err("Invalid response type")
        }
    }

    /// Send any RPC request to the network and await the response
    pub async fn send_rpc_request(
        &mut self,
        peer_id: PeerId,
        rpc_request: RPCRequest,
    ) -> Result<RPCResponse, &'static str> {
        let request_id = self.request_id;
        self.request_id += 1;
        self.send_rpc_event(peer_id, RPCEvent::Request(request_id, rpc_request))
            .await;
        loop {
            match future::timeout(Duration::from_secs(RPC_TIMEOUT), self.rpc_receiver.next()).await
            {
                Ok(Some((id, response))) => {
                    if id == request_id {
                        return Ok(response);
                    }
                    // Ignore any other RPC responses for now
                }
                Ok(None) => return Err("RPC Stream closed"),
                Err(_) => return Err("Connection timeout"),
            }
        }
    }

    /// Handles sending the base event to the network service
    async fn send_rpc_event(&mut self, peer_id: PeerId, event: RPCEvent) {
        self.network_send
            .send(NetworkMessage::RPC { peer_id, event })
            .await
    }
}
