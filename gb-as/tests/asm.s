
.text
_start: ; program entry point
    ; disable interrupts
    di
    ; no operation
    nop
fini:
    ; halt the cpu
    halt

.data
x:
    .byte 1
