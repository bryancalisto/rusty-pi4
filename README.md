# rusty-py

### I wanted to learn Rust, and thought "what's better than a hardware/software hello world when talking about low level?".
So, I was exploring some tutorials and ran into an interesting video in YouTube: Low Level Learning's BAREMETAL RUST Runs on EVERYTHING.
I wanted to try it, but not in a Raspberry Pi 3 B+ (I don't have one) as in the video, but a Raspberry Pi 4 (I've got one).

This repo contains a working version of the blinking LED baremetal code inspired in the video that can run in the RPi 4.
I couldn't find a solid guide about it when doubts kicked in while I was trying to do it myself, so here I leave a guide about it that may
clarify the path to others.

## Main differences between the implementation in RPi 3 and RPi 4.
### The processor.
RPi 3 uses a Broadcom BCM2837 ARM Cortex-A53 and the RPi 4 uses a Broadcom BCM2711 ARM Cortex-A72.

### The boot files.

### The kernel img name.
