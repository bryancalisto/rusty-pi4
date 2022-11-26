# rusty-pi4

####  I wanted to start with Rust, and thought "what's better than a hardware/software hello world when talking about low level?"
So, I was exploring some tutorials and ran into an interesting video in YouTube: [Low Level Learning's BAREMETAL RUST Runs on EVERYTHING](https://www.youtube.com/watch?v=jZT8APrzvc4).
I wanted to try it, but not in a Raspberry Pi 3 B+ (I don't have one) as in the video, but a Raspberry Pi 4 (I've got one).

I had some incompatibility problems.

This repo contains a working version of the blinking LED 32 bit baremetal code inspired in the video that can be run in the RPi 4.
I couldn't find a solid guide about it when doubts kicked in while I was trying to do it myself, so here I leave a guide about it that may
clarify the path to others.

## Main differences between the implementation in RPi 3 and RPi 4.
### The processor
RPi 3 uses a Broadcom BCM2837 ARM Cortex-A53 and the RPi 4 uses a Broadcom BCM2711 ARM Cortex-A72. The main difference is the peripherals' starting address: For RPi 4, we use the 32-bit 'Legacy master addresses' starting at 0x0_FEnn_nnnn because 'Low Peripheral mode' is enabled by default for the Pi.

### The boot files
In the RPi 4 you have to use the files that have the '4' suffix: fixup4.dat and start4.elf. You don't need bootcode.bin because that code is now stored in the onboard EEPROM. [See here.](https://www.riscosopen.org/wiki/documentation/show/Software%20information:%20Raspberry%20Pi:%20Firmware)

### The kernel img name
The kernel img is expected to be called kernel7l.img (notice the 'l'). [See here.](https://www.raspberrypi.com/documentation/computers/config_txt.html#kernel)