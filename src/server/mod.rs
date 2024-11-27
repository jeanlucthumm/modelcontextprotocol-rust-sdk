use anyhow::Result;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use mcp_typegen::mcp_types::{InitializeRequest, InitializeResult};

pub async fn run(addr: &str) -> Result<()> {
    Ok(())
}

#[rpc(server)]
pub trait MCPServer {
    #[method(name = "initialize")]
    async fn initialize(&self, params: InitializeRequest) -> RpcResult<InitializeResult>;

    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<()>;
}

pub struct MCPServerImpl;

impl MCPServerImpl;

impl MCPServerImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MCPServer for MCPServerImpl {
    async fn initialize(&self, params: InitializeRequest) -> RpcResult<InitializeResult> {

    }

    async fn ping(&self) -> RpcResult<()> {
        Ok(())
    }
}
