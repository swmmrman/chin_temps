#!/bin/bash
if [ $(id -u) -ne 0 ]
    then echo "This script run this as root."
    exit
fi

if [ ! -e /etc/chin_temps/ ]
    then
    mkdir /etc/chin_temps/
fi
cp config.ron /etc/chin_temps/config.ron