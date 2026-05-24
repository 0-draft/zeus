/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Zeus — Model Context Protocol (MCP) server entry point.
// Scaffold only; real implementation lands in a follow-up PR. See
// docs/zeus-mcp-server.md for the tool surface and design.

use crate::commands::args::{McpArgs, McpTransport};
use crate::commands::CommandContext;
use crate::util::errors::{wrap, AnyError};

pub async fn mcp(_ctx: CommandContext, args: McpArgs) -> Result<i32, AnyError> {
	let raw_workspace = match args.workspace {
		Some(p) => p,
		None => std::env::current_dir().map_err(|e| wrap(e, "could not resolve workspace from cwd"))?,
	};

	// Canonicalize so the security check ("refuses operations on paths
	// outside this root") can rely on byte-prefix comparison instead of
	// having to re-resolve relative segments on every request.
	let workspace = std::fs::canonicalize(&raw_workspace)
		.map_err(|e| wrap(e, format!("could not canonicalize workspace path {}", raw_workspace.display())))?;

	match args.transport {
		McpTransport::Stdio => {
			eprintln!(
				"zeus mcp: stdio transport not yet implemented (workspace={})",
				workspace.display()
			);
		}
		McpTransport::Sse => {
			eprintln!(
				"zeus mcp: sse transport not yet implemented (port={}, bind={}, workspace={})",
				args.port,
				args.bind,
				workspace.display()
			);
		}
	}

	Ok(0)
}
