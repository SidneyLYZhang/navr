#!/usr/bin/env pwsh
# Navr Installation Script for PowerShell
# Cross-platform installation script for navr - a fast directory navigation tool

#Requires -Version 5.1

[CmdletBinding()]
param(
    [Parameter()]
    [string]$InstallDir = $env:INSTALL_DIR,

    [Parameter()]
    [switch]$NoShell,

    [Parameter()]
    [switch]$NoConfig,

    [Parameter()]
    [switch]$Help
)

# Configuration
$script:RepoUrl = "https://github.com/sidneylyzhang/navr"
$script:DefaultInstallDir = if ($IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6 -and $env:OS -eq "Windows_NT")) {
    "$env:LOCALAPPDATA\navr\bin"
} else {
    "/usr/local/bin"
}
$script:InstallDir = if ($InstallDir) { $InstallDir } else { $script:DefaultInstallDir }
$script:ConfigDir = ""
$script:ShellType = ""
$script:BuildDir = ""

# Colors support detection
$script:SupportsAnsi = $Host.UI.SupportsVirtualTerminal -or ($env:TERM -and $env:TERM -ne "dumb")

# Color definitions
$script:Red = if ($script:SupportsAnsi) { "`e[0;31m" } else { "" }
$script:Green = if ($script:SupportsAnsi) { "`e[0;32m" } else { "" }
$script:Yellow = if ($script:SupportsAnsi) { "`e[1;33m" } else { "" }
$script:Blue = if ($script:SupportsAnsi) { "`e[0;34m" } else { "" }
$script:Cyan = if ($script:SupportsAnsi) { "`e[0;36m" } else { "" }
$script:Bold = if ($script:SupportsAnsi) { "`e[1m" } else { "" }
$script:NC = if ($script:SupportsAnsi) { "`e[0m" } else { "" }

# Logging functions
function Write-LogInfo { param([string]$Message) Write-Host "$($script:Blue)ℹ$($script:NC) $Message" }
function Write-LogSuccess { param([string]$Message) Write-Host "$($script:Green)✓$($script:NC) $Message" }
function Write-LogWarn { param([string]$Message) Write-Host "$($script:Yellow)⚠$($script:NC) $Message" }
function Write-LogError { param([string]$Message) Write-Host "$($script:Red)✗$($script:NC) $Message" -ForegroundColor Red }
function Write-LogStep { param([string]$Message) Write-Host "$($script:Cyan)$($script:Bold)→$($script:NC) $Message" }

function Show-Banner {
    Write-Host "$($script:Cyan)"
    Write-Host " _   _"
    Write-Host "| \ | | __ ___   ___ __"
    Write-Host "|  \| |/ _` \ \ / / '__|"
    Write-Host "| |\  | (_| |\ V /| |"
    Write-Host "|_| \_|\__,_| \_/ |_|"
    Write-Host "$($script:NC)"
    Write-Host "$($script:Bold)Fast directory navigation tool$($script:NC)"
    Write-Host ""
}

function Get-OS {
    if ($IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6 -and $env:OS -eq "Windows_NT")) {
        return "windows"
    } elseif ($IsMacOS) {
        return "macos"
    } elseif ($IsLinux) {
        return "linux"
    } else {
        return "unknown"
    }
}

function Get-Architecture {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
    switch ($arch) {
        "X64" { return "x86_64" }
        "Arm64" { return "aarch64" }
        "X86" { return "i686" }
        default { return "unknown" }
    }
}

function Get-ShellType {
    $parentProcess = Get-Process -Id $PID | Select-Object -ExpandProperty Parent
    $parentName = if ($parentProcess) { $parentProcess.ProcessName } else { "" }

    if ($env:SHELL -like "*zsh*" -or $parentName -like "*zsh*") {
        $script:ShellType = "zsh"
    } elseif ($env:SHELL -like "*bash*" -or $parentName -like "*bash*") {
        $script:ShellType = "bash"
    } elseif ($env:SHELL -like "*fish*" -or $parentName -like "*fish*") {
        $script:ShellType = "fish"
    } elseif ($PSVersionTable.PSEdition -eq "Core" -or $PSVersionTable.PSEdition -eq "Desktop") {
        $script:ShellType = "powershell"
    } else {
        $script:ShellType = "powershell"
        Write-LogWarn "Could not detect shell type, defaulting to PowerShell"
    }
    Write-LogInfo "Detected shell: $($script:Bold)$($script:ShellType)$($script:NC)"
}

function Get-ConfigDir {
    $os = Get-OS
    switch ($os) {
        "macos" { $script:ConfigDir = "$env:HOME/Library/Application Support/navr" }
        "linux" { $script:ConfigDir = "$env:XDG_CONFIG_HOME/navr" }
        "windows" { $script:ConfigDir = "$env:APPDATA\navr" }
        default { $script:ConfigDir = "$env:HOME/.navr" }
    }
}

function Test-Dependencies {
    Write-LogStep "Checking dependencies..."

    $missingDeps = @()

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        $missingDeps += "cargo"
    }

    if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
        $missingDeps += "git"
    }

    if ($missingDeps.Count -gt 0) {
        Write-LogError "Missing required dependencies: $($missingDeps -join ', ')"
        Write-Host ""
        Write-Host "Please install the missing dependencies:"
        Write-Host "  - Rust/Cargo: https://rustup.rs/"
        Write-Host "  - Git: https://git-scm.com/downloads"
        exit 1
    }

    Write-LogSuccess "All dependencies found"
}

function Get-BuildDir {
    # Check if we're already in the navr repository
    if ((Test-Path "Cargo.toml") -and (Select-String -Path "Cargo.toml" -Pattern '^name = "navr"' -Quiet)) {
        $script:BuildDir = "."
        Write-LogInfo "Building from current directory"
    } elseif ((Test-Path "navr\Cargo.toml")) {
        $script:BuildDir = "navr"
        Write-LogInfo "Building from existing navr/ directory"
    } else {
        $script:BuildDir = "navr"
        Write-LogInfo "Will clone repository to navr/"
    }
}

function Build-Navr {
    Write-LogStep "Building navr..."

    $startDir = Get-Location

    try {
        if ($script:BuildDir -eq ".") {
            # Already in the repo
            git pull --ff-only 2>$null
            if ($LASTEXITCODE -ne 0) {
                Write-LogWarn "Could not pull latest changes"
            }
        } elseif (Test-Path $script:BuildDir) {
            # Existing directory, update it
            Set-Location $script:BuildDir
            git pull --ff-only 2>$null
            if ($LASTEXITCODE -ne 0) {
                Write-LogWarn "Could not pull latest changes"
            }
        } else {
            # Clone fresh
            git clone --depth 1 $script:RepoUrl $script:BuildDir
            Set-Location $script:BuildDir
        }

        # Build with optimizations
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            throw "Build failed"
        }
    } finally {
        Set-Location $startDir
    }

    Write-LogSuccess "Build successful"
}

function Install-Binary {
    Write-LogStep "Installing binary..."

    $binaryPath = if ($IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6 -and $env:OS -eq "Windows_NT")) {
        "$($script:BuildDir)\target\release\navr.exe"
    } else {
        "$($script:BuildDir)/target/release/navr"
    }

    if (-not (Test-Path $binaryPath)) {
        Write-LogError "Binary not found at $binaryPath"
        Write-LogInfo "Did the build complete successfully?"
        exit 1
    }

    # Create install directory if it doesn't exist
    if (-not (Test-Path $script:InstallDir)) {
        try {
            New-Item -ItemType Directory -Path $script:InstallDir -Force | Out-Null
        } catch {
            Write-LogError "Failed to create install directory: $script:InstallDir"
            Write-LogInfo "You may need to run this script as Administrator"
            exit 1
        }
    }

    # Copy binary
    $destPath = Join-Path $script:InstallDir (Split-Path $binaryPath -Leaf)
    try {
        Copy-Item $binaryPath $destPath -Force
    } catch {
        Write-LogError "Failed to copy binary to $destPath"
        Write-LogInfo "You may need to run this script as Administrator"
        exit 1
    }

    # Verify installation
    if (Test-Path $destPath) {
        Write-LogSuccess "Installed to $destPath"
    } else {
        Write-LogError "Installation verification failed"
        exit 1
    }

    # Check if install directory is in PATH
    $pathSeparator = if ($IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6 -and $env:OS -eq "Windows_NT")) { ";" } else { ":" }
    $pathVar = $env:PATH
    if ($pathVar -notlike "*$pathSeparator$($script:InstallDir)$pathSeparator*" -and 
        $pathVar -notlike "$($script:InstallDir)$pathSeparator*" -and 
        $pathVar -notlike "*$pathSeparator$($script:InstallDir)") {
        Write-LogWarn "$($script:InstallDir) is not in your PATH"
        Write-Host "   Add the following to your PowerShell profile:"
        Write-Host "   `$env:PATH = '$($script:InstallDir);' + `$env:PATH"
    }
}

function Set-Config {
    Write-LogStep "Setting up configuration..."

    if (-not (Test-Path $script:ConfigDir)) {
        New-Item -ItemType Directory -Path $script:ConfigDir -Force | Out-Null
    }

    $configFile = Join-Path $script:ConfigDir "config.toml"
    if (-not (Test-Path $configFile)) {
        # Initialize default config using navr itself
        $navrPath = Join-Path $script:InstallDir (if ($IsWindows -or ($PSVersionTable.PSVersion.Major -lt 6 -and $env:OS -eq "Windows_NT")) { "navr.exe" } else { "navr" })
        & $navrPath config reset 2>$null
        if ($LASTEXITCODE -eq 0) {
            Write-LogSuccess "Created default configuration at $($script:ConfigDir)"
        } else {
            Write-LogWarn "Could not create default configuration"
            Write-LogInfo "Run 'navr config reset' manually after installation"
        }
    } else {
        Write-LogWarn "Configuration already exists, skipping"
    }
}

function Install-ShellIntegration {
    Write-LogStep "Installing shell integration..."

    $profilePath = $PROFILE
    $integrationLine = "Invoke-Expression (& navr shell init powershell)"

    if (-not (Test-Path $profilePath)) {
        $profileDir = Split-Path $profilePath -Parent
        if (-not (Test-Path $profileDir)) {
            New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
        }
        New-Item -ItemType File -Path $profilePath -Force | Out-Null
    }

    # Check if already installed
    $profileContent = Get-Content $profilePath -Raw -ErrorAction SilentlyContinue
    if ($profileContent -and $profileContent.Contains("navr shell init")) {
        Write-LogWarn "Shell integration already exists in $profilePath"
        return
    }

    # Add integration
    Add-Content $profilePath "`n# navr - fast directory navigation`n$integrationLine`n"
    Write-LogSuccess "Added shell integration to $profilePath"
}

function Show-NextSteps {
    $profilePath = $PROFILE

    Write-Host ""
    Write-Host "$($script:Green)$($script:Bold)Installation complete!$($script:NC)"
    Write-Host ""
    Write-Host "$($script:Cyan)$($script:Bold)Next steps:$($script:NC)"
    Write-Host ""
    Write-Host "1. Reload your PowerShell profile:"
    Write-Host "   . $profilePath"
    Write-Host ""
    Write-Host "2. Add your first shortcut:"
    Write-Host "   cd C:\path\to\project"
    Write-Host "   navr jump --add myproject"
    Write-Host ""
    Write-Host "3. Jump to your shortcut:"
    Write-Host "   j myproject"
    Write-Host ""
    Write-Host "4. View all shortcuts:"
    Write-Host "   navr jump --list"
    Write-Host ""
    Write-Host "5. Get help:"
    Write-Host "   navr --help"
    Write-Host ""
    Write-Host "For more information, visit: $($script:Blue)$($script:RepoUrl)$($script:NC)"
}

function Show-Help {
    @"
Navr Installation Script for PowerShell

Usage: .\install.ps1 [OPTIONS]

Options:
    -Help               Show this help message
    -InstallDir DIR     Install directory (default: %LOCALAPPDATA%\navr\bin on Windows)
    -NoShell            Skip shell integration
    -NoConfig           Skip configuration setup

Environment Variables:
    INSTALL_DIR         Override default install directory

Examples:
    .\install.ps1                  # Default installation
    .\install.ps1 -InstallDir C:\Tools  # Install to C:\Tools
    .\install.ps1 -NoShell         # Install without shell integration
"@
}

# Cleanup function
function Clear-BuildDir {
    if ($script:BuildDir -eq "navr" -and (Test-Path $script:BuildDir)) {
        Write-Host ""
        Write-LogInfo "Build directory retained at: $(Get-Location)\$($script:BuildDir)"
        Write-LogInfo "You can manually remove it with: Remove-Item -Recurse -Force $($script:BuildDir)"
    }
}

# Main installation flow
function Start-Installation {
    Show-Banner

    $os = Get-OS
    $arch = Get-Architecture

    Write-LogInfo "OS: $($script:Bold)$os$($script:NC) | Architecture: $($script:Bold)$arch$($script:NC)"
    Write-Host ""

    Get-ShellType
    Get-ConfigDir
    Test-Dependencies
    Get-BuildDir

    Write-Host ""
    Write-Host "Installation summary:"
    Write-Host "  Install directory: $($script:Bold)$($script:InstallDir)$($script:NC)"
    Write-Host "  Config directory:  $($script:Bold)$($script:ConfigDir)$($script:NC)"
    Write-Host "  Shell type:        $($script:Bold)$($script:ShellType)$($script:NC)"
    Write-Host ""

    $continue = Read-Host "Continue with installation? (Y/n)"
    if ($continue -match '^[Nn]$') {
        Write-LogInfo "Installation cancelled."
        exit 0
    }

    Write-Host ""

    try {
        Build-Navr
        Install-Binary

        if (-not $NoConfig) {
            Set-Config
        }

        if (-not $NoShell) {
            Install-ShellIntegration
        }

        Show-NextSteps
    } catch {
        Write-LogError "Installation failed: $_"
        Clear-BuildDir
        exit 1
    }
}

# Run main function
if ($Help) {
    Show-Help
    exit 0
}

Start-Installation
