#!/usr/bin/env bash

set -eo pipefail;

DAY="${1}";

# usage: replace-rs.sh DAY
if [ -z "${DAY}" ]; then
    echo "usage: ${0} DAY";
    exit 1;
fi

PADDED="$(printf '%02d' "${DAY}")";

OUTPUT_FILE="src/day${PADDED}.rs";

rg --passthru 'XX' -r "${PADDED}" < template/dayXX.rs > "${OUTPUT_FILE}";
