#!/bin/sh

set -e

cargo build
genzehn --input rsspire.elf --output rsspire.tns --240x320-support true --uses-lcd-blit true
make-prg rsspire.tns rsspire.prg.tns
firebird-send rsspire.prg.tns /programs
