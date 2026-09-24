mod core;
mod emitter;
mod ir;
mod lexer;
mod parser;
mod runtime;

use std::fs;
use emitter::Emitter;
use ir::IRGenerator;
use lexer::Lexer;
use parser::Parser;
use runtime::WindowsRuntimeEmitter;

fn main() {
    println!("=========================================================");
    println!("  Axiom Compiler (Windows x64 Native Modular Runtime)    ");
    println!("=========================================================");

    let source_path = "source.txt";
    let user_asm = "user_output.asm";
    let runtime_asm = "axiom_rt.asm";
    let bundled_asm = "bundled_output.asm";

    // 1. Initialize the dedicated Windows Runtime Emitter
    let rt = WindowsRuntimeEmitter::new();
    println!("[1/4] Target Runtime: Windows x86-64 (kernel32.dll direct)");

    // 2. Read and parse user code
    println!("[2/4] Reading '{}'...", source_path);
    let source = fs::read_to_string(source_path)
        .unwrap_or_else(|_| panic!("Failed to read source file: {}", source_path));

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    // 3. Generate 3-Address Intermediate Representation
    println!("[3/4] Generating 3-Address Code (IR)...");
    let mut ir_gen = IRGenerator::new();
    let ir = ir_gen.generate_program(ast);

    // 4. Emit assembly using decoupled architecture
    println!("[4/4] Emitting Assembly Files...");
    let emitter = Emitter::new(&rt);

    // Mode A: User code object (Zero kernel instructions, calls __axiom_claim/__axiom_purge)
    emitter
        .emit_user_code_only(&ir, user_asm)
        .expect("Failed to write user assembly output");

    // Mode B: Standalone runtime library object (Emitted by its dedicated runtime emitter)
    emitter
        .emit_runtime_library_only(runtime_asm)
        .expect("Failed to write runtime assembly output");

    // Mode C: Bundled single-file (for simple single-command assembly)
    emitter
        .emit_bundled_to_file(&ir, bundled_asm)
        .expect("Failed to write bundled assembly output");

    println!("=========================================================");
    println!("SUCCESS! Output files created:");
    println!("  1. {}   -> User code object (zero kernel instructions)", user_asm);
    println!("  2. {}      -> Runtime library object (HeapAlloc/HeapFree/WriteFile)", runtime_asm);
    println!("  3. {} -> Bundled single-file (ready to run)", bundled_asm);
    println!("=========================================================");
    println!();
    println!("HOW TO ASSEMBLE AND LINK ON WINDOWS:");
    println!("---------------------------------------------------------");
    println!("Option 1: True Modular Separate Linking (C/C++ CRT style)");
    println!("   nasm -f win64 {} -o user.obj", user_asm);
    println!("   nasm -f win64 {} -o axiom_rt.obj", runtime_asm);
    println!("   x86_64-w64-mingw32-gcc -nostdlib -e mainCRTStartup user.obj axiom_rt.obj -lkernel32 -o program.exe");
    println!();
    println!("Option 2: Single Bundled Build");
    println!("   nasm -f win64 {} -o bundled.obj", bundled_asm);
    println!("   x86_64-w64-mingw32-gcc -nostdlib -e mainCRTStartup bundled.obj -lkernel32 -o program.exe");
    println!("=========================================================");
}
