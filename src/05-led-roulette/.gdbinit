define hook-quit
set confirm off
target remote :3333
load
break led_roulette::main
continue
