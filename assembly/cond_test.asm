; Flags and conditional jump and call test for
; the Intel 8080 CPU

; This test assumes MVI, ADD B Instructions work

; @Io3dev 2022

LXI SP, 0xFF
JMP jifz

jifz:
        ADD B
        JZ  jifnz
        HLT

jifnz:
        MVI B, 0x1
        ADD B
        JNZ jifc
        HLT

jifc:
        MVI B, 0xFF
        ADD B
        JC jifnc
        HLT

jifnc:
        ADI 0
        CALL OK
        JMP DONE
DONE:
        JMP DONE

; Sets mem value in 0x1111 to 0x1
OK:
        MVI H, 0x11
        MVI L, 0x11

        MVI M, 0x1

        RET
        



        


