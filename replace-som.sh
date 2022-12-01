#!/usr/bin/env bash

set -eo pipefail;

DAY="${1}";

# usage: replace-som.sh DAY
if [ -z "${DAY}" ]; then
    echo "usage: ${0} DAY";
    exit 1;
fi

PADDED="$(printf '%02d' "${DAY}")";

OUTPUT_FILE="som/Day${PADDED}.som";

rg --passthru 'XX' -r "${PADDED}" < template/DayXX.som > "${OUTPUT_FILE}";
