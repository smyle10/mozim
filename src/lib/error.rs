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

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    IpcError,
    InvalidIpcCommand,
    MozimBug,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MozimError {
    pub kind: ErrorKind,
    pub msg: String,
}

impl MozimError {
    pub fn bug(msg: String) -> MozimError {
        MozimError {
            kind: ErrorKind::MozimBug,
            msg: msg,
        }
    }
    pub fn invalid_ipc_command(msg: String) -> MozimError {
        MozimError {
            kind: ErrorKind::InvalidIpcCommand,
            msg: msg,
        }
    }
}

impl std::fmt::Display for MozimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl std::convert::From<std::io::Error> for MozimError {
    fn from(e: std::io::Error) -> Self {
        MozimError {
            kind: ErrorKind::IpcError,
            msg: e.to_string(),
        }
    }
}

impl std::convert::From<std::string::FromUtf8Error> for MozimError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        MozimError {
            kind: ErrorKind::IpcError,
            msg: e.to_string(),
        }
    }
}
