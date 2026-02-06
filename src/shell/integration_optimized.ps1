# =============================================================================
#
# Utility functions for navr.
#

# Call navr binary, returning the output as UTF-8.
function global:__navr_bin {
    $encoding = [Console]::OutputEncoding
    try {
        [Console]::OutputEncoding = [System.Text.Utf8Encoding]::new()
        $result = navr @args
        return $result
    } finally {
        [Console]::OutputEncoding = $encoding
    }
}

# pwd based on navr's format.
function global:__navr_pwd {
    $cwd = Get-Location
    if ($cwd.Provider.Name -eq "FileSystem") {
        $cwd.ProviderPath
    }
}

# cd + custom logic based on navr integration.
function global:__navr_cd($dir, $literal) {
    if ($literal) {
        Set-Location -LiteralPath $dir -Passthru -ErrorAction Stop
    } else {
        if ($dir -eq '-' -and ($PSVersionTable.PSVersion -lt 6.1)) {
            Write-Error "cd - is not supported below PowerShell 6.1. Please upgrade your version of PowerShell."
        }
        elseif ($dir -eq '+' -and ($PSVersionTable.PSVersion -lt 6.2)) {
            Write-Error "cd + is not supported below PowerShell 6.2. Please upgrade your version of PowerShell."
        }
        else {
            Set-Location -Path $dir -Passthru -ErrorAction Stop
        }
    }
}

# =============================================================================
#
# Hook configuration for navr.
#

# Hook to add new entries to the database.
$global:__navr_oldpwd = __navr_pwd
function global:__navr_hook {
    $result = __navr_pwd
    if ($result -ne $global:__navr_oldpwd) {
        if ($null -ne $result) {
            navr add "--" $result
        }
        $global:__navr_oldpwd = $result
    }
}

# Initialize hook.
$global:__navr_hooked = (Get-Variable __navr_hooked -ErrorAction Ignore -ValueOnly)
if ($global:__navr_hooked -ne 1) {
    $global:__navr_hooked = 1
    $global:__navr_prompt_old = $function:prompt
    
    function global:prompt {
        if ($null -ne $__navr_prompt_old) {
            & $__navr_prompt_old
        }
        $null = __navr_hook
    }
}

# =============================================================================
#
# When using navr with --no-cmd, alias these internal functions as desired.
#

# Jump to a directory using only keywords.
function global:__navr_jump {
    if ($args.Length -eq 0) {
        __navr_cd ~ $true
    }
    elseif ($args.Length -eq 1 -and ($args[0] -eq '-' -or $args[0] -eq '+')) {
        __navr_cd $args[0] $false
    }
    elseif ($args.Length -eq 1 -and (Test-Path -PathType Container -LiteralPath $args[0])) {
        __navr_cd $args[0] $true
    }
    elseif ($args.Length -eq 1 -and (Test-Path -PathType Container -Path $args[0] )) {
        __navr_cd $args[0] $false
    }
    else {
        $result = __navr_pwd
        if ($null -ne $result) {
            $result = __navr_bin jump "--" @args
        }
        else {
            $result = __navr_bin jump "--" @args
        }
        if ($LASTEXITCODE -eq 0) {
            __navr_cd $result $true
        }
    }
}

# Jump to a directory using interactive search.
function global:__navr_jump_interactive {
    $result = __navr_bin jump -i "--" @args
    if ($LASTEXITCODE -eq 0) {
        __navr_cd $result $true
    }
}

# Open directory in file manager.
function global:__navr_open {
    __navr_bin open "--" @args
}

# List all shortcuts.
function global:__navr_list {
    __navr_bin jump --list "--" @args
}

# Show configuration.
function global:__navr_config {
    __navr_bin config show "--" @args
}

# =============================================================================
#
# Commands for navr. Disable these using --no-cmd.
#

Set-Alias -Name j -Value __navr_jump -Option AllScope -Scope Global -Force
Set-Alias -Name ji -Value __navr_jump_interactive -Option AllScope -Scope Global -Force
Set-Alias -Name jo -Value __navr_open -Option AllScope -Scope Global -Force
Set-Alias -Name jl -Value __navr_list -Option AllScope -Scope Global -Force
Set-Alias -Name jc -Value __navr_config -Option AllScope -Scope Global -Force

# Override Set-Location (cd) with navr integration
function global:Set-LocationNavr {
    param([string]$Path)
    
    if ([string]::IsNullOrEmpty($Path)) {
        __navr_cd ~ $true
    } elseif (Test-Path -Path $Path -PathType Container) {
        __navr_cd $Path $false
    } else {
        # Try to resolve via navr
        $result = __navr_bin jump "--" $Path
        if ($LASTEXITCODE -eq 0) {
            __navr_cd $result $true
        } else {
            __navr_cd $Path $false
        }
    }
}

Set-Alias -Name cd -Value Set-LocationNavr -Option AllScope -Scope Global -Force

# Tab completion
Register-ArgumentCompleter -CommandName navr -ParameterName target -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)
    
    $shortcuts = & navr jump --list 2>$null | 
        Select-String -Pattern '^  (\w+)' | 
        ForEach-Object { $_.Matches.Groups[1].Value }
    
    $shortcuts | Where-Object { $_ -like "$wordToComplete*" } | 
        ForEach-Object { [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_) }
}

Register-ArgumentCompleter -CommandName j -ScriptBlock {
    param($commandName, $parameterName, $wordToComplete, $commandAst, $fakeBoundParameters)
    
    $shortcuts = & navr jump --list 2>$null | 
        Select-String -Pattern '^  (\w+)' | 
        ForEach-Object { $_.Matches.Groups[1].Value }
    
    $shortcuts | Where-Object { $_ -like "$wordToComplete*" } | 
        ForEach-Object { [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_) }
}

# =============================================================================
#
# To initialize navr, add this to your configuration (find it by running
# `echo $profile` in PowerShell):
#
# Invoke-Expression (& { (navr shell init powershell | Out-String) })