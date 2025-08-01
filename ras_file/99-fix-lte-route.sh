#!/bin/bash
IFACE=$1
STATUS=$2

if [ "$IFACE" = "usb0" ] && [ "$STATUS" = "up" ]; then
    # LTE default route 제거
    ip addr flush dev usb0
    ip addr add 192.168.10.110/24 dev usb0
    ip route del default dev usb0 2>/dev/null
fi


#File path /etc/NetworkManager/dispatcher.d/ras_file/99-fix-lte-route.sh