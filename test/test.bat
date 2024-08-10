@echo off

REM Compile the C test file using cl.exe, linking necessary Windows libraries
cl test_print_pretty.c ^
    /I ..\target\release ^
    /I ..\target\debug ^
    /link ^
    /LIBPATH:..\target\release ^
    /LIBPATH:..\target\debug ^
    bat_c.lib ^
    Kernel32.lib ^
    Advapi32.lib ^
    User32.lib ^
    Gdi32.lib ^
    Shell32.lib ^
    ntdll.lib ^
    ws2_32.lib ^
    bcrypt.lib ^
    secur32.lib ^
    Shlwapi.lib ^
    Userenv.lib ^
    ucrt.lib ^
    /NODEFAULTLIB:MSVCRT ^
    /NODEFAULTLIB:libucrt.lib ^
    /out:test_print_pretty.exe

REM Check if the compilation was successful
if not exist "test_print_pretty.exe" (
    echo Compilation failed
    exit /b 1
)

REM Run the compiled test
test_print_pretty.exe
