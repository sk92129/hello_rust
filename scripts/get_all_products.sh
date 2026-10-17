#!/usr/bin/env bash
set -euo pipefail

catalog_url="${CATALOG_API_URL:-http://127.0.0.1:3000/api/catalog}"

curl --fail --show-error --silent "$catalog_url" | python3 -m json.tool
printf '\n'