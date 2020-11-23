// Copyright 2020 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::dhcp_manager::{MozimDhcpCmd, MozimDhcpManager};
use mozim::{DhcpStatus, MozimError};
use serde_json;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

#[derive(Debug)]
pub(crate) struct MozimThreadManager {
    pub iface_name: String,
    pub sender: SyncSender<MozimDhcpCmd>,
    pub recver: Receiver<Result<String, MozimError>>,
}

impl MozimThreadManager {
    pub(crate) fn new(iface_name: &str) -> Result<Self, MozimError> {
        // Create rust sync rendezvous channel for thread communication to
        // simplfy things.
        let (to_thread_sender, to_thread_recver) =
            sync_channel::<MozimDhcpCmd>(0);
        let (from_thread_sender, from_thread_recver) =
            sync_channel::<Result<String, MozimError>>(0);
        let to_thread_sender_clone = to_thread_sender.clone();
        let iface_name_clone = iface_name.to_string();
        thread::Builder::new()
            .name(format!("dhcp_{}", &iface_name))
            .spawn(move || {
                MozimDhcpManager::run(
                    iface_name_clone,
                    from_thread_sender,
                    to_thread_sender_clone,
                    to_thread_recver,
                )
            })?;
        Ok(MozimThreadManager {
            iface_name: iface_name.to_string(),
            sender: to_thread_sender,
            recver: from_thread_recver,
        })
    }

    pub(crate) fn stop_dhcp(&self) -> Result<DhcpStatus, MozimError> {
        handle_send_result(self.sender.send(MozimDhcpCmd::StopDhcp))?;
        string_to_dhcp_status(&handle_recv_result(self.recver.recv())?)
    }

    pub(crate) fn start_dhcp(&self) -> Result<DhcpStatus, MozimError> {
        handle_send_result(self.sender.send(MozimDhcpCmd::StartDhcp))?;
        string_to_dhcp_status(&handle_recv_result(self.recver.recv())?)
    }

    pub(crate) fn query_dhcp(&self) -> Result<DhcpStatus, MozimError> {
        handle_send_result(self.sender.send(MozimDhcpCmd::QueryDhcp))?;
        string_to_dhcp_status(&handle_recv_result(self.recver.recv())?)
    }
}

fn handle_send_result(
    send_result: Result<(), std::sync::mpsc::SendError<MozimDhcpCmd>>,
) -> Result<(), MozimError> {
    match send_result {
        Ok(_) => Ok(()),
        Err(e) => Err(MozimError::bug(format!(
            "Thread communication send error: {}",
            e
        ))),
    }
}

fn handle_recv_result(
    recv_result: Result<Result<String, MozimError>, std::sync::mpsc::RecvError>,
) -> Result<String, MozimError> {
    match recv_result {
        Ok(Ok(o)) => Ok(o),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(MozimError::bug(format!(
            "Thread communication recv error: {}",
            e
        ))),
    }
}

fn string_to_dhcp_status(status_str: &str) -> Result<DhcpStatus, MozimError> {
    match serde_json::from_str(&status_str) {
        Ok(s) => Ok(s),
        Err(e) => Err(MozimError::bug(format!(
            "serde_json::from_str() error: {}",
            e
        ))),
    }
}
