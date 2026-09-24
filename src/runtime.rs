use std::fs::File;
use std::io::{self, Write};

/// Interface that any runtime library emitter must implement.
/// Axiom's user-code emitter relies solely on this trait, keeping
/// the general emitter 100% free of OS kernel API calls.
pub trait RuntimeEmitter {
    fn name(&self) -> &'static str;
    fn alloc_symbol(&self) -> &'static str;
    fn free_symbol(&self) -> &'static str;
    fn print_int_symbol(&self) -> &'static str;
    fn print_double_symbol(&self) -> &'static str;
    fn print_string_symbol(&self) -> &'static str;
    fn print_bool_symbol(&self) -> &'static str;

    fn emit_externs(&self, file: &mut File) -> io::Result<()>;
    fn emit_globals(&self, file: &mut File) -> io::Result<()>;
    fn emit_data(&self, file: &mut File) -> io::Result<()>;
    fn emit_bss(&self, file: &mut File) -> io::Result<()>;
    fn emit_startup_prologue(&self, file: &mut File) -> io::Result<()>;
    fn emit_exit(&self, file: &mut File, code: i32) -> io::Result<()>;
    fn emit_runtime_text(&self, file: &mut File) -> io::Result<()>;
}

/// Windows x64 Runtime Emitter
///
/// Implements the Axiom runtime library specifically for Windows x64.
/// It interacts directly with Windows Win32 kernel32.dll:
///  - HeapAlloc / HeapFree (via GetProcessHeap) for claim() and purge()
///  - WriteFile (via GetStdHandle) for emit statements
///  - ExitProcess for termination
///
/// Zero C standard library (no malloc, no free, no printf).
pub struct WindowsRuntimeEmitter;

impl WindowsRuntimeEmitter {
    pub fn new() -> Self {
        WindowsRuntimeEmitter
    }
}

impl Default for WindowsRuntimeEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeEmitter for WindowsRuntimeEmitter {
    fn name(&self) -> &'static str {
        "Windows x86-64 Native Runtime (kernel32.dll)"
    }

    fn alloc_symbol(&self) -> &'static str {
        "__axiom_claim"
    }

    fn free_symbol(&self) -> &'static str {
        "__axiom_purge"
    }

    fn print_int_symbol(&self) -> &'static str {
        "__axiom_print_int"
    }

    fn print_double_symbol(&self) -> &'static str {
        "__axiom_print_double"
    }

    fn print_string_symbol(&self) -> &'static str {
        "__axiom_print_string"
    }

    fn print_bool_symbol(&self) -> &'static str {
        "__axiom_print_bool"
    }

    fn emit_externs(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "; --- Windows kernel32.dll API Imports ---")?;
        writeln!(file, "extern GetProcessHeap")?;
        writeln!(file, "extern HeapAlloc")?;
        writeln!(file, "extern HeapFree")?;
        writeln!(file, "extern GetStdHandle")?;
        writeln!(file, "extern WriteFile")?;
        writeln!(file, "extern ExitProcess")?;
        writeln!(file)?;
        Ok(())
    }

    fn emit_globals(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "; --- Axiom Runtime Exported Functions ---")?;
        writeln!(file, "global __axiom_rt_init")?;
        writeln!(file, "global __axiom_claim")?;
        writeln!(file, "global __axiom_purge")?;
        writeln!(file, "global __axiom_print_int")?;
        writeln!(file, "global __axiom_print_double")?;
        writeln!(file, "global __axiom_print_string")?;
        writeln!(file, "global __axiom_print_bool")?;
        writeln!(file)?;
        Ok(())
    }

    fn emit_data(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "    ; Axiom Runtime Static Literals")?;
        writeln!(file, "    __axiom_crlf: db 13, 10")?;
        writeln!(file, "    __axiom_str_yes: db 'yes', 13, 10")?;
        writeln!(file, "    __axiom_str_no: db 'no', 13, 10")?;
        writeln!(file)?;
        Ok(())
    }

    fn emit_bss(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "    ; Axiom Runtime Internal State")?;
        writeln!(file, "    __axiom_hHeap: resq 1")?;
        writeln!(file, "    __axiom_hStdOut: resq 1")?;
        writeln!(file, "    __axiom_bytes_written: resq 1")?;
        writeln!(file, "    __axiom_print_buf: resb 64")?;
        writeln!(file)?;
        Ok(())
    }

    fn emit_startup_prologue(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "    ; Initialize Axiom Runtime (heap & stdout handles)")?;
        writeln!(file, "    call __axiom_rt_init")?;
        Ok(())
    }

    fn emit_exit(&self, file: &mut File, code: i32) -> io::Result<()> {
        writeln!(file, "    ; Exit process via Windows API")?;
        writeln!(file, "    mov ecx, {}", code)?;
        writeln!(file, "    call ExitProcess")?;
        Ok(())
    }

    fn emit_runtime_text(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "; ===================================================================")?;
        writeln!(file, "; Axiom Runtime Library - Windows x64 Native Implementation")?;
        writeln!(file, "; Freestanding / Zero C Runtime")?;
        writeln!(file, "; ===================================================================")?;
        writeln!(file)?;

        // Runtime Initialization
        writeln!(file, "__axiom_rt_init:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 32")?;
        writeln!(file, "    ; Cache STD_OUTPUT_HANDLE (-11)")?;
        writeln!(file, "    mov ecx, -11")?;
        writeln!(file, "    call GetStdHandle")?;
        writeln!(file, "    mov [__axiom_hStdOut], rax")?;
        writeln!(file, "    ; Cache default Process Heap handle")?;
        writeln!(file, "    call GetProcessHeap")?;
        writeln!(file, "    mov [__axiom_hHeap], rax")?;
        writeln!(file, "    add rsp, 32")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Allocation: __axiom_claim(size in bytes) -> pointer in rax
        writeln!(file, "; __axiom_claim(rcx: size) -> rax: pointer")?;
        writeln!(file, "__axiom_claim:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 32")?;
        writeln!(file, "    mov r8, rcx")?;
        writeln!(file, "    mov rcx, [__axiom_hHeap]")?;
        writeln!(file, "    mov edx, 8")?; // HEAP_ZERO_MEMORY
        writeln!(file, "    call HeapAlloc")?;
        writeln!(file, "    add rsp, 32")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Free: __axiom_purge(ptr in rcx)
        writeln!(file, "; __axiom_purge(rcx: pointer)")?;
        writeln!(file, "__axiom_purge:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 32")?;
        writeln!(file, "    test rcx, rcx")?;
        writeln!(file, "    jz .free_done")?;
        writeln!(file, "    mov r8, rcx")?;
        writeln!(file, "    mov rcx, [__axiom_hHeap]")?;
        writeln!(file, "    xor edx, edx")?;
        writeln!(file, "    call HeapFree")?;
        writeln!(file, ".free_done:")?;
        writeln!(file, "    add rsp, 32")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Print Integer: __axiom_print_int(rcx: i64)
        writeln!(file, "; __axiom_print_int(rcx: int64)")?;
        writeln!(file, "__axiom_print_int:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 48")?;
        writeln!(file, "    mov rax, rcx")?;
        writeln!(file, "    lea rdi, [__axiom_print_buf + 31]")?;
        writeln!(file, "    mov byte [rdi], 0")?;
        writeln!(file, "    mov byte [rdi - 1], 10")?; // '\n'
        writeln!(file, "    mov byte [rdi - 2], 13")?; // '\r'
        writeln!(file, "    sub rdi, 2")?;
        writeln!(file, "    test rax, rax")?;
        writeln!(file, "    jns .int_pos")?;
        writeln!(file, "    neg rax")?;
        writeln!(file, "    mov r8b, 1")?; // negative flag
        writeln!(file, "    jmp .int_loop")?;
        writeln!(file, ".int_pos:")?;
        writeln!(file, "    xor r8b, r8b")?;
        writeln!(file, ".int_loop:")?;
        writeln!(file, "    xor edx, edx")?;
        writeln!(file, "    mov rbx, 10")?;
        writeln!(file, "    div rbx")?;
        writeln!(file, "    add dl, '0'")?;
        writeln!(file, "    dec rdi")?;
        writeln!(file, "    mov [rdi], dl")?;
        writeln!(file, "    test rax, rax")?;
        writeln!(file, "    jnz .int_loop")?;
        writeln!(file, "    test r8b, r8b")?;
        writeln!(file, "    jz .int_write")?;
        writeln!(file, "    dec rdi")?;
        writeln!(file, "    mov byte [rdi], '-'")?;
        writeln!(file, ".int_write:")?;
        writeln!(file, "    lea rax, [__axiom_print_buf + 31]")?;
        writeln!(file, "    sub rax, rdi")?; // length in rax
        writeln!(file, "    mov rcx, [__axiom_hStdOut]")?;
        writeln!(file, "    mov rdx, rdi")?;
        writeln!(file, "    mov r8, rax")?;
        writeln!(file, "    lea r9, [__axiom_bytes_written]")?;
        writeln!(file, "    mov qword [rsp + 32], 0")?;
        writeln!(file, "    call WriteFile")?;
        writeln!(file, "    add rsp, 48")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Print String: __axiom_print_string(rcx: *const char)
        writeln!(file, "; __axiom_print_string(rcx: const char*)")?;
        writeln!(file, "__axiom_print_string:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 48")?;
        writeln!(file, "    test rcx, rcx")?;
        writeln!(file, "    jz .str_done")?;
        writeln!(file, "    mov rsi, rcx")?;
        writeln!(file, "    xor r8, r8")?;
        writeln!(file, ".str_len:")?;
        writeln!(file, "    cmp byte [rsi + r8], 0")?;
        writeln!(file, "    je .str_write")?;
        writeln!(file, "    inc r8")?;
        writeln!(file, "    jmp .str_len")?;
        writeln!(file, ".str_write:")?;
        writeln!(file, "    mov rcx, [__axiom_hStdOut]")?;
        writeln!(file, "    mov rdx, rsi")?;
        writeln!(file, "    lea r9, [__axiom_bytes_written]")?;
        writeln!(file, "    mov qword [rsp + 32], 0")?;
        writeln!(file, "    call WriteFile")?;
        writeln!(file, "    ; Print CRLF")?;
        writeln!(file, "    mov rcx, [__axiom_hStdOut]")?;
        writeln!(file, "    lea rdx, [__axiom_crlf]")?;
        writeln!(file, "    mov r8, 2")?;
        writeln!(file, "    lea r9, [__axiom_bytes_written]")?;
        writeln!(file, "    mov qword [rsp + 32], 0")?;
        writeln!(file, "    call WriteFile")?;
        writeln!(file, ".str_done:")?;
        writeln!(file, "    add rsp, 48")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Print Bool: __axiom_print_bool(rcx: i64)
        writeln!(file, "; __axiom_print_bool(rcx: bool flag)")?;
        writeln!(file, "__axiom_print_bool:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 48")?;
        writeln!(file, "    test rcx, rcx")?;
        writeln!(file, "    jz .bool_no")?;
        writeln!(file, "    lea rdx, [__axiom_str_yes]")?;
        writeln!(file, "    mov r8, 5")?; // 'yes\r\n'
        writeln!(file, "    jmp .bool_print")?;
        writeln!(file, ".bool_no:")?;
        writeln!(file, "    lea rdx, [__axiom_str_no]")?;
        writeln!(file, "    mov r8, 4")?; // 'no\r\n'
        writeln!(file, ".bool_print:")?;
        writeln!(file, "    mov rcx, [__axiom_hStdOut]")?;
        writeln!(file, "    lea r9, [__axiom_bytes_written]")?;
        writeln!(file, "    mov qword [rsp + 32], 0")?;
        writeln!(file, "    call WriteFile")?;
        writeln!(file, "    add rsp, 48")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        // Print Double: __axiom_print_double(xmm0: f64)
        writeln!(file, "; __axiom_print_double(xmm0: double)")?;
        writeln!(file, "__axiom_print_double:")?;
        writeln!(file, "    push rbp")?;
        writeln!(file, "    mov rbp, rsp")?;
        writeln!(file, "    sub rsp, 48")?;
        writeln!(file, "    cvttsd2si rcx, xmm0")?;
        writeln!(file, "    call __axiom_print_int")?;
        writeln!(file, "    add rsp, 48")?;
        writeln!(file, "    pop rbp")?;
        writeln!(file, "    ret")?;
        writeln!(file)?;

        Ok(())
    }
}
