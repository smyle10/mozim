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

use mozim::{DhcpState, DhcpStatus, MozimError};
use serde_json;
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Debug)]
pub(crate) enum MozimDhcpCmd {
    StartDhcp,
    QueryDhcp,
    StopDhcp,
}

pub(crate) struct MozimDhcpManager {
    iface_name: String,
    sender: SyncSender<Result<String, MozimError>>,
    recver: Receiver<MozimDhcpCmd>,
}

impl MozimDhcpManager {
    fn reply_request(&self, result: Result<String, MozimError>) {
        if let Err(e) = self.sender.send(result) {
            eprintln!(
                "BUG: Failed to send data back to threads manager: {}",
                e
            );
        }
    }
    pub(crate) fn run(
        iface_name: String,
        sender: SyncSender<Result<String, MozimError>>,
        // TODO: _dhcp_worker_sender will moved to DHCP worker thread
        // where MozimDhcpManager could get DHCP status update from.
        _dhcp_worker_sender: SyncSender<MozimDhcpCmd>,
        recver: Receiver<MozimDhcpCmd>,
    ) {
        let mgr = MozimDhcpManager {
            iface_name,
            sender,
            recver,
        };
        loop {
            if let Ok(cmd) = mgr.recver.recv() {
                match cmd {
                    MozimDhcpCmd::StartDhcp => {
                        mgr.reply_request(mgr.start());
                    }
                    MozimDhcpCmd::QueryDhcp => {
                        mgr.reply_request(mgr.query());
                    }
                    MozimDhcpCmd::StopDhcp => {
                        mgr.reply_request(mgr.stop());
                        break;
                    }
                };
            }
        }
    }

    fn start(&self) -> Result<String, MozimError> {
        dhcp_status_to_string(&DhcpStatus {
            iface_name: self.iface_name.clone(),
            state: DhcpState::Requesting,
        })
    }

    fn query(&self) -> Result<String, MozimError> {
        dhcp_status_to_string(&DhcpStatus {
            iface_name: self.iface_name.clone(),
            state: DhcpState::Requesting,
        })
    }
    fn stop(&self) -> Result<String, MozimError> {
        dhcp_status_to_string(&DhcpStatus {
            iface_name: self.iface_name.clone(),
            state: DhcpState::Stopped,
        })
    }
}

fn dhcp_status_to_string(
    dhcp_status: &DhcpStatus,
) -> Result<String, MozimError> {
    match serde_json::to_string(dhcp_status) {
        Ok(s) => Ok(s),
        Err(e) => Err(MozimError::bug(format!(
            "serde_json::to_string() error: {}",
            e
        ))),
    }
}
