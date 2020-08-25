.PHONY: build upload

build:
	avr-as --mmcu=atmega328p blink.asm 

upload: build
	avrdude -P /dev/ttyACM0 -c arduino -p m328p a.out

build-128:
	avr-as --mmcu=atmega128 blink.asm 

simul-128: build-128
	simulavr -d atmega128 a.out
