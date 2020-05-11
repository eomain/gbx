
.text
_start: ; program entry point
    ; disable interrupts
    di
    ei
    ; no operation
    nop
fini:
    ; halt the cpu
    halt

.data
x:
    .byte 1
