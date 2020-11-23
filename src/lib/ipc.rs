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

use std::fs::remove_file;

use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{UnixListener, UnixStream};

use crate::MozimError;

const DEFAULT_SOCKET_PATH: &str = "/tmp/mozim_socket";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MozimResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MozimError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl MozimResult {
    pub fn data(s: String) -> Self {
        MozimResult {
            error: None,
            data: Some(s),
        }
    }
    pub fn error(e: MozimError) -> Self {
        MozimResult {
            error: Some(e),
            data: None,
        }
    }
}

pub fn ipc_bind() -> Result<UnixListener, MozimError> {
    remove_file(DEFAULT_SOCKET_PATH).ok();
    Ok(UnixListener::bind(DEFAULT_SOCKET_PATH)?)
}

pub async fn ipc_connect() -> Result<UnixStream, MozimError> {
    Ok(UnixStream::connect(DEFAULT_SOCKET_PATH).await?)
}

pub async fn ipc_send(
    stream: &mut UnixStream,
    data: &str,
) -> Result<(), MozimError> {
    let data_bytes = data.as_bytes();
    stream.write_u32(data_bytes.len() as u32).await?;
    stream.write_all(data_bytes).await?;
    Ok(())
}

pub async fn ipc_recv(stream: &mut UnixStream) -> Result<String, MozimError> {
    let data_size = stream.read_u32().await? as usize;
    let mut data = vec![0u8; data_size];

    stream.read_exact(&mut data).await?;

    Ok(std::string::String::from_utf8(data)?)
}

pub async fn ipc_exec(
    stream: &mut UnixStream,
    cmd: &str,
) -> Result<String, MozimError> {
    ipc_send(stream, cmd).await?;
    ipc_recv(stream).await
}
