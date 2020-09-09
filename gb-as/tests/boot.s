
; Include program info
.use "info.s"

; Program entry point
_start: ; Jump to the main routine
    nop
    stop
    call main
    jp _start

main: ; Store registers on the stack
    push af
    push bc
    push de
    push hl

    ; Do some operation

    ; Restore registers from the stack
    pop hl
    pop de
    pop bc
    pop af
    ret

; End of program rom
.org 0x8000, 0x00
