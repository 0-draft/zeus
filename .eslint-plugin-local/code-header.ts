/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

import * as eslint from 'eslint';

// Drop-in for `header/header` after eslint-plugin-header was abandoned
// against ESLint 9 (still calls context.getSourceCode() at runtime).
// Enforces an exact-match block comment as the file header. The header
// lines are passed via rule options:
//
//   'local/code-header': [2, ['line one without the /* */', 'line two', ...]]

interface RuleOptions {
	lines: readonly string[];
}

function buildExpectedFromBareLines(lines: readonly string[]): string {
	// Historical `header/header` config shape: each element is the
	// inner content of a `/* … */` block comment. The closing `*/`
	// sits immediately after the last line's content, on the same
	// line, matching the project's convention.
	return `/*${lines.join('\n')}*/`;
}

export default new class HeaderRule implements eslint.Rule.RuleModule {

	readonly meta: eslint.Rule.RuleMetaData = {
		type: 'layout',
		fixable: 'whitespace',
		messages: {
			missing: 'Missing or incorrect file header. Expected a block comment matching the project header.',
		},
		schema: false,
	};

	create(context: eslint.Rule.RuleContext): eslint.Rule.RuleListener {
		const options = context.options ?? [];
		// Accept both `[2, 'block', [lines...]]` (legacy header/header
		// shape) and `[2, [lines...]]` (compact) so the eslint.config.js
		// migration can stay minimal.
		let lines: readonly string[] = [];
		for (const opt of options) {
			if (Array.isArray(opt)) {
				lines = opt;
				break;
			}
		}
		if (lines.length === 0) {
			return {};
		}

		const expected = buildExpectedFromBareLines(lines);

		return {
			Program(node) {
				const comments = context.sourceCode.getAllComments();
				const first = comments[0];
				const isOk =
					first &&
					first.type === 'Block' &&
					first.range &&
					first.range[0] === 0 &&
					context.sourceCode.text.slice(first.range[0], first.range[1]) === expected;
				if (isOk) {
					return;
				}
				context.report({
					node,
					messageId: 'missing',
					fix(fixer) {
						if (first && first.type === 'Block' && first.range && first.range[0] === 0) {
							return fixer.replaceTextRange(first.range, expected);
						}
						return fixer.insertTextBeforeRange([0, 0], expected + '\n');
					},
				});
			},
		};
	}
};
