use std::{io::Error, path::Path};

use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{UnixStream, unix::{OwnedReadHalf, OwnedWriteHalf}},
};

use super::cmd::{
    handshake::{ProtocolHandshakeRequest, ProtocolHandshakeResponse},
    traits::QmpCmdRequest,
};

/// QmpClient
pub struct QmpClient {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl QmpClient {
    /// Connects to the unix socket path and performs the QMP handshake
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();

        // connect
        let stream: UnixStream = UnixStream::connect(path).await?;
        // split reader/writer by cloning stream (UnixStream supports split via try_clone on std, but tokio supports reusing the same stream with BufReader)
        let (read_half, write_half) = stream.into_split();
        let mut client = QmpClient { reader: BufReader::new(read_half), writer: write_half };

        // read greeting
        let greeting = client.read_message().await?;
        // optional: parse greeting
        let _g: ProtocolHandshakeResponse = serde_json::from_str(&greeting)?;

        // send qmp_capabilities
        client.execute(ProtocolHandshakeRequest).await?;
        Ok(client)
    }

    /// low-level: read one JSON message string (QEMU sends newline terminated JSON objects)
    async fn read_message(&mut self) -> Result<String, Error> {
        let mut buf_reader = BufReader::new(&mut self.reader);
        let mut line = String::new();

        loop {
            line.clear();

            buf_reader.read_line(&mut line).await?;

            if !line.trim().is_empty() {
                return Ok(line);
            }
        }
    }

    /// Execute a command and return the parsed JSON response (serde_json::Value).
    pub async fn execute<C>(&mut self, cmd: C) -> Result<C::Response, Error>
    where
        C: QmpCmdRequest,
        C::Response: DeserializeOwned,
    {
        let id = rand::random::<u64>();

        let exec = cmd.to_execute(Some(id));

        let mut s = serde_json::to_string(&exec)?;
        s.push('\n');
        self.writer.write_all(s.as_bytes()).await?;

        loop {
            let line = self.read_message().await?;
            let v: Value = serde_json::from_str(&line)?;

            if v.get("id").and_then(Value::as_u64) == Some(id) {
                if let Some(ret) = v.get("return") {
                    let typed: C::Response = serde_json::from_value(ret.clone())?;
                    return Ok(typed);
                }
                if let Some(err) = v.get("error") {
                    return Err(todo!());
                }
            }
        }
    }
}
