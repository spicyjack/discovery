define hook-quit
set confirm off
end
target remote :3333
load
break main.rs:main
continue
