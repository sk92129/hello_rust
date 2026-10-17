#!/usr/bin/env bash
set -euo pipefail

catalog_url="${CATALOG_API_URL:-http://127.0.0.1:3000/api/catalog}"
product_name="${PRODUCT_NAME:-Notebook}"
product_description="${PRODUCT_DESCRIPTION:-Ruled paper notebook}"
product_price_cents="${PRODUCT_PRICE_CENTS:-599}"
payload="$(
  PRODUCT_NAME="$product_name" \
    PRODUCT_DESCRIPTION="$product_description" \
    PRODUCT_PRICE_CENTS="$product_price_cents" \
    python3 -c 'import json, os; print(json.dumps({
      "name": os.environ["PRODUCT_NAME"],
      "description": os.environ["PRODUCT_DESCRIPTION"],
      "price_cents": int(os.environ["PRODUCT_PRICE_CENTS"])
    }))'
)"

curl --fail --show-error --silent \
  --request POST \
  --header 'Content-Type: application/json' \
  --data "$payload" \
  "$catalog_url" | python3 -m json.tool
printf '\n'