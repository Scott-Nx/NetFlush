@echo off
setlocal enabledelayedexpansion

:: Set profile name to 'Network' if not provided as a parameter
set "profileName=Network"

:: Path to the Profiles registry key
set "reg_path=HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles"

:: Query the registry for subkeys under Profiles
for /f "tokens=*" %%A in ('reg query "%reg_path%"') do (
    set "key=%%A"

    :: Query the ProfileName value of the current subkey
    for /f "tokens=2,*" %%B in ('reg query "!key!" /v ProfileName 2^>nul') do (
        set "name=%%C"

        :: Check if the ProfileName starts with the input
        echo !name! | findstr /i "^%profileName%" >nul
        if !errorlevel! == 0 (
            :: Silent delete the key
            reg delete "!key!" /f >nul 2>&1
        )
    )
)

exit /b
