/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Zeus — Model Context Protocol (MCP) server entry point.
// Scaffold only; real implementation lands in a follow-up PR. See
// docs/zeus-mcp-server.md for the tool surface and design.

use crate::commands::args::{McpArgs, McpTransport};
use crate::util::errors::{wrap, AnyError};

pub async fn mcp(args: McpArgs) -> Result<i32, AnyError> {
	let workspace = match args.workspace {
		Some(p) => p,
		None => std::env::current_dir().map_err(|e| wrap(e, "could not resolve workspace from cwd"))?,
	};

	match args.transport {
		McpTransport::Stdio => {
			eprintln!(
				"zeus mcp: stdio transport not yet implemented (workspace={})",
				workspace.display()
			);
		}
		McpTransport::Sse => {
			eprintln!(
				"zeus mcp: sse transport not yet implemented (port={}, workspace={})",
				args.port,
				workspace.display()
			);
		}
	}

	Ok(0)
}
