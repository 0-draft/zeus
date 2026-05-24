/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Zeus — Model Context Protocol (MCP) server entry point.
// Scaffold only; real implementation lands in a follow-up PR. See
// docs/zeus-mcp-server.md for the tool surface and design.

use crate::commands::args::{McpArgs, McpTransport};
use crate::commands::CommandContext;
use crate::util::errors::{wrap, AnyError, SetupError};

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

	if !workspace.is_dir() {
		return Err(SetupError(format!(
			"workspace path is not a directory: {}",
			workspace.display()
		))
		.into());
	}

	match args.transport {
		McpTransport::Stdio => {
			eprintln!(
				"zeus mcp: stdio transport not yet implemented (workspace={})",
				workspace.display()
			);
		}
		McpTransport::Sse => {
			// Refuse to bind to a non-loopback interface without explicit opt-in,
			// to keep the default posture local-only. clap already parsed
			// `--bind` into `IpAddr`, so no string-level validation needed here.
			// See docs/zeus-mcp-server.md.
			if !args.bind.is_loopback() && !args.allow_non_loopback {
				return Err(SetupError(format!(
					"refusing to bind SSE transport on non-loopback address {} \
without --allow-non-loopback (use 127.0.0.1 / ::1 for local-only)",
					args.bind
				))
				.into());
			}

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
