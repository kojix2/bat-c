@echo off

REM Compile the C test file using cl.exe, linking necessary Windows libraries
cl test_pretty_print.c ^
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
    OleAut32.lib ^
    Ole32.lib ^
    Propsys.lib ^
    RuntimeObject.lib ^
    /NODEFAULTLIB:MSVCRT ^
    /NODEFAULTLIB:libucrt.lib ^
    /out:test_pretty_print.exe

REM Check if the compilation was successful
if not exist "test_pretty_print.exe" (
    echo Compilation failed
    exit /b 1
)

REM Run the compiled test
test_pretty_print.exe
