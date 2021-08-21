#!/usr/bin/env bash

set -e
set -u

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
readonly MONGODB_HOST="localhost"
readonly DB="whoisPresident"
readonly COLLECTION="potentialCandidates"

mongo "${MONGODB_HOST}/${DB}" ${SCRIPT_DIR}/create-index.js
mongoimport --host "${MONGODB_HOST}" --jsonArray --db "${DB}" --collection "${COLLECTION}" --file ${SCRIPT_DIR}/data/potential-candidates.json