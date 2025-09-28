@echo off
REM Setup script for git hooks configuration
REM Sets the git hooks path and ensures execute permissions

echo Setting up git hooks...

REM Set git hooks path to .githooks
echo Setting git hooks path to .githooks...
git config core.hooksPath .githooks

REM Check if .githooks directory exists
if exist ".githooks" (
    echo Adding execute permissions to git hooks...
    
    REM For PowerShell scripts
    for %%f in (.githooks\*.bat) do (
        powershell -Command "if (Get-Command 'Set-ExecutionPolicy' -ErrorAction SilentlyContinue) { Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force }"
    )
    
    echo Git hooks in .githooks\:
    dir .githooks /B
    echo.
    echo Note: On Windows, .exe, .bat and .cmd files are executable by default.
    echo For PowerShell scripts, execution policy might need to be adjusted.
) else (
    echo Warning: .githooks directory not found
    echo You can create it with: mkdir .githooks
)

echo Setup completed!
echo Git hooks path is now set to: .githooks