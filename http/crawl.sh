#!/usr/bin/env bash
set -euo pipefail

LAT="${1:?Usage: $0 LAT LONG}"
LONG="${2:?Usage: $0 LAT LONG}"


RESULT="$(
  curl --fail --silent --show-error \
    "$API_PROTO://$API_HOST/api/v1/location/search?latitude=$LAT&longitude=$LONG&radius=5&limit=10"
)"


while IFS= read -r locationId; do
  LOCATION_RESULT="$(curl --fail --silent --show-error "$API_PROTO://$API_HOST/api/v1/location/$locationId")"
  jq -rc '.rooms.[]| [.locationId, .roomId]' <<< "$LOCATION_RESULT"
done < <(
  jq -r '.[] | select(.connectedTechnology == "CSCGo") | .locationId' <<< "$RESULT"
)