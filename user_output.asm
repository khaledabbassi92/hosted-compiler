; ===================================================================
; Axiom User Program Unit
; Target Runtime: Windows x86-64 Native Runtime (kernel32.dll)
; Zero Kernel Instructions in User Emitter!
; ===================================================================
default rel

; Axiom Runtime Library Interface
extern __axiom_claim
extern __axiom_purge
extern __axiom_print_int
extern __axiom_print_double
extern __axiom_print_string
extern __axiom_print_bool
extern __axiom_rt_init
extern ExitProcess

section .data
str_lit_0: db 65, 108, 108, 111, 99, 97, 116, 101, 100, 32, 54, 52, 32, 98, 121, 116, 101, 115, 32, 111, 110, 32, 112, 114, 111, 99, 101, 115, 115, 32, 104, 101, 97, 112, 32, 97, 116, 32, 97, 100, 100, 114, 101, 115, 115, 58, 0
str_lit_1: db 86, 97, 108, 117, 101, 32, 115, 116, 111, 114, 101, 100, 32, 105, 110, 115, 105, 100, 101, 32, 104, 101, 97, 112, 32, 98, 108, 111, 99, 107, 58, 0
str_lit_2: db 83, 117, 99, 99, 101, 115, 115, 102, 117, 108, 108, 121, 32, 112, 117, 114, 103, 101, 100, 32, 104, 101, 97, 112, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 33, 0

section .bss
reg_0: resq 1
reg_1: resq 1
reg_2: resq 1
reg_3: resq 1
reg_4: resq 1
reg_5: resq 1
reg_6: resq 1
reg_7: resq 1
reg_8: resq 1
reg_9: resq 1
reg_10: resq 1
reg_11: resq 1
var_buffer_size: resq 8
var_ptr: resq 8

section .text
global mainCRTStartup

mainCRTStartup:
    push rbp
    mov rbp, rsp
    sub rsp, 48
    ; Initialize Axiom Runtime (heap & stdout handles)
    call __axiom_rt_init
    mov rax, 64
    mov [reg_0], rax
    mov rax, [reg_0]
    mov [var_buffer_size], rax
    mov rax, [var_buffer_size]
    mov [reg_1], rax
    ; claim(size) -> Runtime library call
    mov rcx, [reg_1]
    sub rsp, 32
    call __axiom_claim
    add rsp, 32
    mov [reg_2], rax
    mov rax, [reg_2]
    mov [var_ptr], rax
    mov rax, 1337
    mov [reg_3], rax
    mov rax, [var_ptr]
    mov [reg_4], rax
    mov rax, [reg_4]
    mov rbx, [reg_3]
    mov [rax], rbx
    lea rax, [str_lit_0]
    mov [reg_5], rax
    mov rcx, [reg_5]
    sub rsp, 32
    call __axiom_print_string
    add rsp, 32
    mov rax, [var_ptr]
    mov [reg_6], rax
    mov rcx, [reg_6]
    sub rsp, 32
    call __axiom_print_int
    add rsp, 32
    lea rax, [str_lit_1]
    mov [reg_7], rax
    mov rcx, [reg_7]
    sub rsp, 32
    call __axiom_print_string
    add rsp, 32
    mov rax, [var_ptr]
    mov [reg_8], rax
    mov rax, [reg_8]
    mov rbx, [rax]
    mov [reg_9], rbx
    mov rcx, [reg_9]
    sub rsp, 32
    call __axiom_print_int
    add rsp, 32
    mov rax, [var_ptr]
    mov [reg_10], rax
    ; purge(ptr) -> Runtime library call
    mov rcx, [reg_10]
    sub rsp, 32
    call __axiom_purge
    add rsp, 32
    lea rax, [str_lit_2]
    mov [reg_11], rax
    mov rcx, [reg_11]
    sub rsp, 32
    call __axiom_print_string
    add rsp, 32
    ; Exit process via Windows API
    mov ecx, 0
    call ExitProcess

