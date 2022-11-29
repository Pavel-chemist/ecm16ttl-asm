.const
    input_address = 0x0100
    rot_amt = 0x3

.text
    main:

    # alu
        ADD r2 r3 r4
        SUB r2 r3 r4
        AND r2 r3 r4
        ANDN r2 r3 r4
        OR r2 r3 r4
        ORN r2 r3 r4
        XOR r2 r3 r4
        XNOR r2 r3 r4

        ADDC r2 r3 r4
        SUBC r2 r3 r4

        ROT r2 r3 r4
    
    # alu_const
        ADDi r1 number
        SUBi r1 number
        ANDi r1 number
        ANDNi r1 number
        ORi r1 number
        ORNi r1 number
        XORi r1 number
        XNORi r1 number

    #alu_test
        TEQ r2 r3
        TCM r2 r3
        CMN r2 r3
        CMP r2 r3
        TST r2 r3
        TIB r2 r3

    #alu_one_src
        SHL r2 r3
        SHR r2 r3
        ASHL r2 r3
        ASHR r2 r3
        ROLC r2 r3
        RORC r2 r3

        BSE r2 r3

        INV r2 r3

    #alu_rot 
        ROTi r2 r3 rot_amt

.data
    number: 
        .word 0x001F
