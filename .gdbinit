# Load symbols
file build/kernel.elf

# Set breakpoint on panic handler
break core::panicking::panic_fmt

# Disable signal handlers
handle SIGTRAP nostop