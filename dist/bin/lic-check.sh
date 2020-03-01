#!/bin/bash

# (c) Copyright 2020 Christian Saide <supernomad>
# SPDX-License-Identifier: GPL-3.0-only

function list_files_missing_lic() {
    find . \
        -not -path './backend/target/*' \
        -not -path './.git/*' \
        -not -name .gitignore \
        -not -name LICENSE \
        -not -name *.lock \
        -not -name *.toml \
        -type f | xargs grep -L 'SPDX-License-Identifier: GPL-3.0-only'
}

function count_files_missing_lic() {
    list_files_missing_lic | wc -l
}

function list_files_missing_copy() {
    find . \
        -not -path './backend/target/*' \
        -not -path './.git/*' \
        -not -name .gitignore \
        -not -name LICENSE \
        -not -name *.lock \
        -not -name *.toml \
        -type f | xargs grep -L -E '(&copy;|\(c\)) Copyright 2020 Christian Saide <supernomad>'
}

function count_files_missing_copy() {
    list_files_missing_copy | wc -l
}

if [ $(count_files_missing_lic) -ne 0 ]; then
    cat <<EOF
There are files missing the 'SPDX-License-Identifier: GPL-3.0-only' license identifier.

Files:
$(list_files_missing_lic)
EOF
    exit 1
fi

if [ $(count_files_missing_copy) -ne 0 ]; then
    cat <<EOF
There are files missing the '&copy;|(c) Copyright 2020 Christian Saide <supernomad>' copyright identifier.

Files:
$(list_files_missing_copy)
EOF
    exit 1
fi

echo "All files have correct copyright and license identifiers."
exit 0
