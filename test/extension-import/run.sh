#!/usr/bin/env bash
# Zeus — VS Code extension import smoke test.
#
# Installs a curated list of popular extensions against a built Zeus and
# asserts each one activates cleanly. Run from repo root:
#
#   ./test/extension-import/run.sh
#
# Honors $ZEUS_BIN to override the binary path. Defaults to
# ./scripts/z.sh which works for dev builds.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
MANIFEST="${SCRIPT_DIR}/manifest.json"

ZEUS_BIN="${ZEUS_BIN:-${REPO_ROOT}/scripts/z.sh}"
USER_DATA_DIR="$(mktemp -d -t zeus-ext-import-XXXXXX)"
EXT_DIR="${USER_DATA_DIR}/extensions"

cleanup() {
	rm -rf "${USER_DATA_DIR}"
}
trap cleanup EXIT

echo "==> Using Zeus binary: ${ZEUS_BIN}"
echo "==> User data dir:     ${USER_DATA_DIR}"

if [[ ! -x "${ZEUS_BIN}" ]]; then
	echo "FAIL: Zeus binary not found or not executable at ${ZEUS_BIN}" >&2
	exit 1
fi

# Parse manifest with jq (or python fallback).
if command -v jq >/dev/null 2>&1; then
	mapfile -t EXTENSIONS < <(jq -r '.extensions[]' "${MANIFEST}")
else
	mapfile -t EXTENSIONS < <(python3 -c "import json,sys; [print(e) for e in json.load(open(sys.argv[1]))['extensions']]" "${MANIFEST}")
fi

if [[ ${#EXTENSIONS[@]} -eq 0 ]]; then
	echo "FAIL: no extensions found in ${MANIFEST}" >&2
	exit 1
fi

echo "==> Installing ${#EXTENSIONS[@]} extension(s) in a single call…"

# Batch all --install-extension flags into one invocation; each fork
# of the binary is expensive (Node start-up, gulp watch warm-up).
INSTALL_ARGS=()
for ext in "${EXTENSIONS[@]}"; do
	INSTALL_ARGS+=(--install-extension "${ext}")
done

FAILURES=()
if ! "${ZEUS_BIN}" \
		--user-data-dir "${USER_DATA_DIR}" \
		--extensions-dir "${EXT_DIR}" \
		"${INSTALL_ARGS[@]}" 2>&1 | tee -a "${USER_DATA_DIR}/install.log"; then
	FAILURES+=("install:batch")
fi

if [[ ${#FAILURES[@]} -ne 0 ]]; then
	echo "FAIL: extension(s) failed to install:" >&2
	printf '  - %s\n' "${FAILURES[@]}" >&2
	exit 1
fi

echo "==> Listing installed extensions…"
INSTALLED="$("${ZEUS_BIN}" \
	--user-data-dir "${USER_DATA_DIR}" \
	--extensions-dir "${EXT_DIR}" \
	--list-extensions --show-versions 2>&1)"
echo "${INSTALLED}"

for ext in "${EXTENSIONS[@]}"; do
	if ! echo "${INSTALLED}" | grep -qi "^${ext}@"; then
		FAILURES+=("missing:${ext}")
	fi
done

if [[ ${#FAILURES[@]} -ne 0 ]]; then
	echo "FAIL: extension(s) missing from --list-extensions:" >&2
	printf '  - %s\n' "${FAILURES[@]}" >&2
	exit 1
fi

echo "==> OK: ${#EXTENSIONS[@]} extension(s) installed and listed."
