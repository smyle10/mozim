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

use crate::thread_manager::MozimThreadManager;
use crate::MozimError;
use mozim::{DhcpState, DhcpStatus};
use std::collections::HashMap;

pub(crate) struct MozimThreadsManager {
    pub threads: HashMap<String, MozimThreadManager>,
}

impl MozimThreadsManager {
    pub(crate) fn new() -> Self {
        MozimThreadsManager {
            threads: HashMap::new(),
        }
    }

    pub(crate) fn start_dhcp(
        &mut self,
        iface_name: &str,
    ) -> Result<DhcpStatus, MozimError> {
        let thread = self.get_thread(iface_name)?;
        thread.start_dhcp()
    }

    pub(crate) fn query_dhcp(
        &mut self,
        iface_name: &str,
    ) -> Result<DhcpStatus, MozimError> {
        if self.threads.contains_key(iface_name) {
            let thread = self.get_thread(iface_name)?;
            thread.query_dhcp()
        } else {
            Ok(gen_dhcp_stop_status(iface_name))
        }
    }

    pub(crate) fn stop_dhcp(
        &mut self,
        iface_name: &str,
    ) -> Result<DhcpStatus, MozimError> {
        if let Some(thread) = self.threads.remove(iface_name) {
            thread.stop_dhcp()
        } else {
            Ok(gen_dhcp_stop_status(iface_name))
        }
    }

    // Get thread, if not found, create one
    fn get_thread(
        &mut self,
        iface_name: &str,
    ) -> Result<&mut MozimThreadManager, MozimError> {
        if !self.threads.contains_key(iface_name) {
            let thread = MozimThreadManager::new(iface_name)?;
            self.threads.insert(iface_name.into(), thread);
        }

        self.threads.get_mut(iface_name).ok_or_else(|| {
            MozimError::bug(format!(
                "BUG: MozimThreadsManager::get_thread() failed \
                 to find out the thread of {}",
                iface_name
            ))
        })
    }

    pub(crate) fn query_all(&mut self) -> Result<Vec<DhcpStatus>, MozimError> {
        let mut infos = Vec::new();
        for thread in self.threads.values_mut() {
            infos.push(thread.query_dhcp()?);
        }
        Ok(infos)
    }
}

fn gen_dhcp_stop_status(iface_name: &str) -> DhcpStatus {
    DhcpStatus {
        iface_name: iface_name.to_string(),
        state: DhcpState::Stopped,
    }
}
