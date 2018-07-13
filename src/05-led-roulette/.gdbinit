define hook-quit
    set confirm off
end
target remote :3333
load
break led_roulette::main
continue
