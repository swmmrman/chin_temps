#!/bin/bash
if [ $(id -u) -ne 0 ]; then
    echo "This script run this as root."
    exit
fi

if [ ! -e /etc/chin_temps/ ]; then
    mkdir /etc/chin_temps/
fi
cp config.ron /etc/chin_temps/config.ron

if [ ! $(getent group evap) ]; then
    groupadd evap
    usermod -a -G evap $USER
    echo "groups created relog"
fi

if [ ! -e /var/log/evap/ ]; then
    mkdir -p /var/log/evap/
fi

touch /var/log/evap/error.log
touch /var/log/evap/history.log
touch /var/log/evap/adjustments.log

chown -R $USER:evap /var/log/evap
chmod g+w /var/log/evap
