#!/bin/sh

SERVERID="1897";
PING="69"; # 4 max digits
UPLOAD="69000"; # 7 max digits
DOWNLOAD="69000"; # 7 max didits

# 10^18 / 2^59 ~= 1 
# (at least) approximately 59 bits of free data storage!

do_md5_hash() {

    # replace with however you compute md5 hashes on your computer
    __md5=$(md5 -qs "$1")
    
    printf "%s" "${__md5}"
}

MD5PAYLD="${PING}-${UPLOAD}-${DOWNLOAD}-817d699764d33f89c"

PAYLOAD="{
    \"serverid\": ${SERVERID},
    \"ping\": ${PING},
    \"upload\": ${UPLOAD},
    \"download\": ${DOWNLOAD},
    \"hash\": \"$(do_md5_hash $MD5PAYLD)\"
}";

printf "PAYLOAD: %s\n" "$PAYLOAD";

RESULT=$(curl -s 'https://www.speedtest.net/api/results.php' \
    --compressed \
    -X POST \
    -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; rv:128.0) Gecko/20100101 Firefox/128.0' \
    -H 'Accept: application/json, text/plain, */*' \
    -H 'Accept-Language: en-US,en;q=0.5' \
    -H 'Accept-Encoding: gzip, deflate, br, zstd' \
    -H 'Content-Type: application/json;charset=UTF-8' \
    -H 'Origin: https://www.speedtest.net' \
    -H 'DNT: 1' \
    -H 'Sec-GPC: 1' \
    -H 'Connection: keep-alive' \
    -H 'Referer: https://www.speedtest.net/run' \
    -H 'Sec-Fetch-Dest: empty' \
    -H 'Sec-Fetch-Mode: cors' \
    -H 'Sec-Fetch-Site: same-origin' \
    -H 'TE: trailers' \
    --data-raw "${PAYLOAD}");

printf "RESULT: %s\n" "$RESULT";

RESULTID=$(printf "%s" "$RESULT" | awk -F'resultid":' '{split($2, a, ","); print a[1]}');

# curl -s "https://www.speedtest.net/result/${RESULTID}" | rg  "window.OOKLA.INIT_DATA" | awk -F' = ' '{ sub(/;$/, "", $2); print $2 }' | jq;
printf "URL: https://www.speedtest.net/result/%s\n" "${RESULTID}";