#!/usr/bin/env bash

set -ex

script_path=$(cd $(dirname $0) ; pwd -P)
script_path_root="${script_path}/"

tmp_file="/tmp/ip2proxy_lite_20220401_field_length_count.txt"

awk -F "\"*,\"*" -v OFS=' ' '{print length($3), length($4), length($5), length($6), length($7), length($8), length($9), length($10), length($11), length($12), length($13), length($14), length($15)}' "${script_path_root}IP2PROXY-LITE-PX11.CSV" > ${tmp_file}

awk -F "\"*,\"*" -v OFS=' ' '{print length($3), length($4), length($5), length($6), length($7), length($8), length($9), length($10), length($11), length($12), length($13), length($14), length($15)}' "${script_path_root}IP2PROXY-LITE-PX11.IPV6.CSV" >> ${tmp_file}

wc -l ${tmp_file}

# MAX 3
echo "MAX PROXYTYPE is $( awk '{print $1}' ${tmp_file} | sort -n | tail -1 )"

# MAX 2
echo "MAX COUNTRY_COUNT is $( awk '{print $2}' ${tmp_file} | sort -n | tail -1 )"

# MAX 52
echo "MAX COUNTRY_NAME is $( awk '{print $3}' ${tmp_file} | sort -n | tail -1 )"

# MAX 41
echo "MAX REGION is $( awk '{print $4}' ${tmp_file} | sort -n | tail -1 )"

# MAX 39
echo "MAX CITY is $( awk '{print $5}' ${tmp_file} | sort -n | tail -1 )"

# MAX 127
echo "MAX ISP is $( awk '{print $6}' ${tmp_file} | sort -n | tail -1 )"

# MAX 111
echo "MAX DOMAIN is $( awk '{print $7}' ${tmp_file} | sort -n | tail -1 )"

# MAX 29
echo "MAX USAGETYPE is $( awk '{print $8}' ${tmp_file} | sort -n | tail -1 )"

# MAX 10
echo "MAX ASN is $( awk '{print $9}' ${tmp_file} | sort -n | tail -1 )"

# MAX 197
echo "MAX AS is $( awk '{print $10}' ${tmp_file} | sort -n | tail -1 )"

# MAX 132
echo "MAX LASTSEEN is $( awk '{print $11}' ${tmp_file} | sort -n | tail -1 )"

# MAX 19
echo "MAX THREAT is $( awk '{print $12}' ${tmp_file} | sort -n | tail -1 )"

# MAX 24
echo "MAX PROVIDER is $( awk '{print $13}' ${tmp_file} | sort -n | tail -1 )"
