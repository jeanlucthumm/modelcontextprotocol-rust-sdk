# MCP Rust SDK
[![MIT licensed][mit-badge]][mit-url]
[![Specification][spec-badge]][spec-url]

[mit-badge]: https://img.shields.io/pypi/l/mcp.svg
[mit-url]: https://github.com/jeanlucthumm/modelcontextprotocol-rust-sdk/blob/master/LICENSE
[spec-badge]: https://img.shields.io/badge/spec-spec.modelcontextprotocol.io-blue.svg
[spec-url]: https://spec.modelcontextprotocol.io

Rust SDK for the Model Context Protocol

Rust implementation of the [Model Context Protocol](https://modelcontextprotocol.io) (MCP), providing both client and server capabilities for integrating with LLM surfaces.

> [!WARNING]  
> This is a VERY early repo. Contact me at jeanlucthumm@gmail.com if you'd like to get involved. Discord coming soon!

## Overview

The Model Context Protocol allows applications to provide context for LLMs in a standardized way, separating the concerns of providing context from the actual LLM interaction. This Rust SDK implements the full MCP specification, making it easy to:

- Build MCP clients that can connect to any MCP server
- Create MCP servers that expose resources, prompts and tools
- Use standard transports like stdio and SSE
- Handle all MCP protocol messages and lifecycle events
