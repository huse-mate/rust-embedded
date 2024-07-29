target remote localhost:3333
monitor reset halt
monitor flash protect 0 0 7 off
monitor flash write_image erase target/thumbv7em-none-eabi/debug/test
monitor reset halt
file target/thumbv7em-none-eabi/debug/test
load


break DefaultHandler
break HardFault
break rust_begin_unwind
break main


# start the process but immediately halt the processor
stepi