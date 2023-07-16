as
            test sp, 15
            jz no_error_code

            push 1
            jmp continue_to_handler

        no_error_code:
            push 0
            push 0

        continue_to_handler:
        // remember about `cld` as direction flag could be set
