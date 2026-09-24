default rel
; --- Windows kernel32.dll API Imports ---
extern GetProcessHeap
extern HeapAlloc
extern HeapFree
extern GetStdHandle
extern WriteFile
extern ExitProcess

; --- Axiom Runtime Exported Functions ---
global __axiom_rt_init
global __axiom_claim
global __axiom_purge
global __axiom_print_int
global __axiom_print_double
global __axiom_print_string
global __axiom_print_bool

section .data
    ; Axiom Runtime Static Literals
    __axiom_crlf: db 13, 10
    __axiom_str_yes: db 'yes', 13, 10
    __axiom_str_no: db 'no', 13, 10

section .bss
    ; Axiom Runtime Internal State
    __axiom_hHeap: resq 1
    __axiom_hStdOut: resq 1
    __axiom_bytes_written: resq 1
    __axiom_print_buf: resb 64

section .text
; ===================================================================
; Axiom Runtime Library - Windows x64 Native Implementation
; Freestanding / Zero C Runtime
; ===================================================================

__axiom_rt_init:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    ; Cache STD_OUTPUT_HANDLE (-11)
    mov ecx, -11
    call GetStdHandle
    mov [__axiom_hStdOut], rax
    ; Cache default Process Heap handle
    call GetProcessHeap
    mov [__axiom_hHeap], rax
    add rsp, 32
    pop rbp
    ret

; __axiom_claim(rcx: size) -> rax: pointer
__axiom_claim:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    mov r8, rcx
    mov rcx, [__axiom_hHeap]
    mov edx, 8
    call HeapAlloc
    add rsp, 32
    pop rbp
    ret

; __axiom_purge(rcx: pointer)
__axiom_purge:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    test rcx, rcx
    jz .free_done
    mov r8, rcx
    mov rcx, [__axiom_hHeap]
    xor edx, edx
    call HeapFree
.free_done:
    add rsp, 32
    pop rbp
    ret

; __axiom_print_int(rcx: int64)
__axiom_print_int:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    mov rax, rcx
    lea rdi, [__axiom_print_buf + 31]
    mov byte [rdi], 0
    mov byte [rdi - 1], 10
    mov byte [rdi - 2], 13
    sub rdi, 2
    test rax, rax
    jns .int_pos
    neg rax
    mov r8b, 1
    jmp .int_loop
.int_pos:
    xor r8b, r8b
.int_loop:
    xor edx, edx
    mov rbx, 10
    div rbx
    add dl, '0'
    dec rdi
    mov [rdi], dl
    test rax, rax
    jnz .int_loop
    test r8b, r8b
    jz .int_write
    dec rdi
    mov byte [rdi], '-'
.int_write:
    lea rax, [__axiom_print_buf + 31]
    sub rax, rdi
    mov rcx, [__axiom_hStdOut]
    mov rdx, rdi
    mov r8, rax
    lea r9, [__axiom_bytes_written]
    mov qword [rsp + 32], 0
    call WriteFile
    add rsp, 48
    pop rbp
    ret

; __axiom_print_string(rcx: const char*)
__axiom_print_string:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    test rcx, rcx
    jz .str_done
    mov rsi, rcx
    xor r8, r8
.str_len:
    cmp byte [rsi + r8], 0
    je .str_write
    inc r8
    jmp .str_len
.str_write:
    mov rcx, [__axiom_hStdOut]
    mov rdx, rsi
    lea r9, [__axiom_bytes_written]
    mov qword [rsp + 32], 0
    call WriteFile
    ; Print CRLF
    mov rcx, [__axiom_hStdOut]
    lea rdx, [__axiom_crlf]
    mov r8, 2
    lea r9, [__axiom_bytes_written]
    mov qword [rsp + 32], 0
    call WriteFile
.str_done:
    add rsp, 48
    pop rbp
    ret

; __axiom_print_bool(rcx: bool flag)
__axiom_print_bool:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    test rcx, rcx
    jz .bool_no
    lea rdx, [__axiom_str_yes]
    mov r8, 5
    jmp .bool_print
.bool_no:
    lea rdx, [__axiom_str_no]
    mov r8, 4
.bool_print:
    mov rcx, [__axiom_hStdOut]
    lea r9, [__axiom_bytes_written]
    mov qword [rsp + 32], 0
    call WriteFile
    add rsp, 48
    pop rbp
    ret

; __axiom_print_double(xmm0: double)
__axiom_print_double:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    cvttsd2si rcx, xmm0
    call __axiom_print_int
    add rsp, 48
    pop rbp
    ret

