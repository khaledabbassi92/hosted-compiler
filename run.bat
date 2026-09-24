@echo off
setlocal

set "CARGO=C:\Users\khaled\.cargo\bin\cargo.exe"
set "NASM=C:\Users\khaled\AppData\Local\bin\NASM\nasm.exe"
set "GPP=C:\Users\khaled\AppData\Local\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.MCF.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin\g++.exe"

echo [1/5] Running compiler...
"%CARGO%" run
if errorlevel 1 (
    echo Compiler failed.
    pause
    exit /b 1
)

echo.
echo [2/5] Assembling user code...
"%NASM%" -f win64 user_output.asm -o user_output.obj
if errorlevel 1 (
    echo Assembly of user_output.asm failed.
    pause
    exit /b 1
)

echo.
echo [3/5] Assembling runtime library...
"%NASM%" -f win64 axiom_rt.asm -o axiom_rt.obj
if errorlevel 1 (
    echo Assembly of axiom_rt.asm failed.
    pause
    exit /b 1
)

echo.
echo [4/5] Linking...
"%GPP%" -nostdlib user_output.obj axiom_rt.obj -o output.exe -lkernel32 -Wl,-e,mainCRTStartup
if errorlevel 1 (
    echo Linking failed.
    pause
    exit /b 1
)

echo.
echo ===== PROGRAM OUTPUT =====
echo.

echo [5/5] Running generated program...
echo.

.\output.exe

echo.
pause