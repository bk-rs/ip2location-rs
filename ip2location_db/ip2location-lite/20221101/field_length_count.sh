#!/usr/bin/env bash

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

tmp_file="/tmp/ip2location_lite_field_length_count.txt"

awk -F "\"*,\"*" -v OFS=' ' '{print length($3), length($4), length($5), length($6), length($9), length($10)}' "${script_path_root}IP2LOCATION-LITE-DB11.CSV" > ${tmp_file}

awk -F "\"*,\"*" -v OFS=' ' '{print length($3), length($4), length($5), length($6), length($9), length($10)}' "${script_path_root}IP2LOCATION-LITE-DB11.IPV6.CSV" >> ${tmp_file}

wc -l ${tmp_file}

# MAX 2
echo "MAX COUNTRY_COUNT is $( awk '{print $1}' ${tmp_file} | sort -n | tail -1 )"

# MAX 52
echo "MAX COUNTRY_NAME is $( awk '{print $2}' ${tmp_file} | sort -n | tail -1 )"

# MAX 44
echo "MAX REGION is $( awk '{print $3}' ${tmp_file} | sort -n | tail -1 )"

# MAX 54
echo "MAX CITY is $( awk '{print $4}' ${tmp_file} | sort -n | tail -1 )"

# MAX 14
echo "MAX ZIPCODE is $( awk '{print $5}' ${tmp_file} | sort -n | tail -1 )"

# MAX 8
echo "MAX TIMEZONE is $( awk '{print $6}' ${tmp_file} | sort -n | tail -1 )"
