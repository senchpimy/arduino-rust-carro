#! /usr/bin/zsh

echo $1
cargo build
sudo avrdude -q -C/etc/avrdude.conf -patmega328p -carduino -P/dev/ttyACM0 -D "-Uflash:w:$1:e"


