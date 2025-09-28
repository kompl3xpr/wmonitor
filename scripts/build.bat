@echo off
REM Build script for Windows
REM Steps:
REM 1. Build release version using cargo
REM 2. Create bin directory
REM 3. Copy executables from target/release to bin
REM 4. Copy database directory and config file to bin

echo Starting build process...

REM Build release version
echo Running cargo build --release...
cargo build --release
if %errorlevel% neq 0 (
    echo Build failed!
    exit /b 1
)

REM Create bin directory
echo Creating bin directory...
if not exist "bin" mkdir bin

REM Copy executable files
echo Copying executables...
xcopy /Y "target\release\*.exe" "bin\"

REM Copy database directory
echo Copying database directory...
if exist "db" (
    xcopy /E /Y "db" "bin\db\"
) else (
    echo Warning: db directory not found
)

REM Copy config file
echo Copying config file...
if exist "cfg.toml" (
    copy /Y "cfg.toml" "bin\"
) else (
    echo Warning: cfg.toml not found
)

echo Build completed successfully!