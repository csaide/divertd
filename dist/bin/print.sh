#!/bin/bash

# (c) Copyright 2020 Christian Saide <supernomad>
# SPDX-License-Identifier: GPL-3.0-only

COLUMNS=0
LEN=0

if [[ $(which tput | wc -c) -ne 0 ]]; then
    COLUMNS=$(tput cols)
    LEN=$(echo "${1}" | wc -c)
fi

printf "=%.s" $(seq 1 $(( (${COLUMNS} / 2) - ((${LEN} / 2) + 1) )))
printf " %s " "${1}"
printf "=%.s" $(seq 1 $(( (${COLUMNS} / 2) - ((${LEN} / 2) + 1) )))
printf "\n"
