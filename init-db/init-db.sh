#!/usr/bin/env bash

set -e
set -u

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
readonly DB="whoisPresident"
readonly COLLECTION="potentialCandidates"

mongo "${DB}" ${SCRIPT_DIR}/create-index.js
mongoimport --jsonArray --db "${DB}" --collection "${COLLECTION}" --file ${SCRIPT_DIR}/data/potential-candidates.json