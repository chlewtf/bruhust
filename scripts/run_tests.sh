#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
#  Bruhust Integration Test Runner
#  Runs every .bruh file in tests/bruh/ and compares stdout to its .expected
#  Usage: bash scripts/run_tests.sh
# ─────────────────────────────────────────────────────────────────────────────

set -euo pipefail

BINARY="./target/release/bruhust"
TESTS_DIR="./tests/bruh"
RESULTS_DIR="./test-results"
PASS=0
FAIL=0
TOTAL=0

mkdir -p "$RESULTS_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

echo ""
echo -e "${BOLD}${CYAN}╔══════════════════════════════════════════╗${RESET}"
echo -e "${BOLD}${CYAN}║           BRUHUST TEST RUNNER            ║${RESET}"
echo -e "${BOLD}${CYAN}╚══════════════════════════════════════════╝${RESET}"
echo ""

if [[ ! -f "$BINARY" ]]; then
  echo -e "${RED}[ERROR]${RESET} binary not found at $BINARY — run 'cargo build --release' first bestie"
  exit 1
fi

for bruh_file in "$TESTS_DIR"/*.bruh; do
  [[ -f "$bruh_file" ]] || continue
  expected_file="${bruh_file%.bruh}.expected"
  test_name=$(basename "$bruh_file" .bruh)
  TOTAL=$((TOTAL + 1))

  if [[ ! -f "$expected_file" ]]; then
    echo -e "  ${YELLOW}[SKIP]${RESET} $test_name — no .expected file found"
    continue
  fi

  actual=$("$BINARY" "$bruh_file" 2>&1 || true)
  expected=$(cat "$expected_file")

  if [[ "$actual" == "$expected" ]]; then
    echo -e "  ${GREEN}[PASS]${RESET} $test_name ✅"
    PASS=$((PASS + 1))
    echo "PASS" > "$RESULTS_DIR/${test_name}.result"
  else
    echo -e "  ${RED}[FAIL]${RESET} $test_name ❌"
    echo -e "         ${BOLD}expected:${RESET}"
    echo "$expected" | sed 's/^/           /'
    echo -e "         ${BOLD}got:${RESET}"
    echo "$actual" | sed 's/^/           /'
    FAIL=$((FAIL + 1))
    {
      echo "FAIL"
      echo "=== expected ==="
      echo "$expected"
      echo "=== actual ==="
      echo "$actual"
    } > "$RESULTS_DIR/${test_name}.result"
  fi
done

echo ""
echo -e "${BOLD}─────────────────────────────────────────────${RESET}"
echo -e "${BOLD}  Results: ${GREEN}${PASS} passed${RESET} | ${RED}${FAIL} failed${RESET} | ${TOTAL} total${RESET}"
echo -e "${BOLD}─────────────────────────────────────────────${RESET}"
echo ""

if [[ $FAIL -gt 0 ]]; then
  echo -e "${RED}Some tests are giving L energy 💀 — fix em bestie${RESET}"
  exit 1
else
  echo -e "${GREEN}All tests are absolutely bussin 🔥 no cap${RESET}"
  exit 0
fi
