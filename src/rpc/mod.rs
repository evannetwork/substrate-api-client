/*
   Copyright 2019 Supercomputing Systems AG

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

use client::*;
use futures_signals::signal::Mutable;
mod client;
pub mod json_req;

use futures::channel::mpsc::{
    Receiver,
    Sender,
    channel
};

#[derive(Debug, PartialEq)]
pub enum XtStatus {
    Finalized,
    InBlock,
    Broadcast,
    Ready,
    Future,
    Error,
    Unknown,
}
pub async fn get(url: String, json_req: String, result_in: Mutable<String>) {
    start_rpc_client_thread(url, json_req, result_in, on_get_request_msg)
}

pub async fn send_extrinsic(url: String, json_req: String, result_in: Mutable<String>) {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_ready)
}

pub async fn send_extrinsic_and_wait_until_broadcast(
    url: String,
    json_req: String,
    result_in: Mutable<String>,
) {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_broadcast)
}

pub async fn send_extrinsic_and_wait_until_in_block(
    url: String,
    json_req: String,
    result_in: Mutable<String>,
) {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_in_block)
}

pub fn send_extrinsic_and_wait_until_finalized(
    url: String,
    json_req: String,
    result_in: Mutable<String>,
) {
    start_rpc_client_thread(url, json_req, result_in, on_extrinsic_msg_until_finalized)
}

pub async fn start_subcriber(url: String, json_req: String, result_in: Sender<String>) {
    start_rpc_client_thread_sender(url, json_req, result_in, on_subscription_msg).await
}


pub use client::start_rpc_client_thread;