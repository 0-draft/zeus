/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// Stable identifiers for the parallel-agents auxiliary-bar view. Exported
// here (rather than left as strings in the README) so the status-bar item
// in feat/prompt-cache-hud, command-palette entries, and tests can all
// import a single source of truth without duplicating literals.
//
// The real view registration lives in feat/agent-sdk; it consumes these
// constants when wiring up ViewContainerRegistry / ViewsRegistry.

export const PARALLEL_AGENTS_VIEW_CONTAINER_ID = 'zeus.parallelAgents';
export const PARALLEL_AGENTS_VIEW_ID = 'zeus.parallelAgents.list';

export const PARALLEL_AGENTS_COMMAND_NEW = 'zeus.parallelAgents.new';
export const PARALLEL_AGENTS_COMMAND_FOCUS = 'zeus.parallelAgents.focus';
