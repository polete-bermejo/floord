
$ErrorActionPreference = "Stop"

# CONFIGURATION
$Repo = "polete-bermejo/floord"
$InstallDir = Join-Path $env:LOCALAPPDATA "Floord"
$AppExe = Join-Path $InstallDir "floord.exe"
$TaskName = "Floord"

$AppArgs = "--daemon --organizer --cleanup --allow-deletion"

# CREATE INSTALLATION DIRECTORY
Write-Host "Preparing installation directory..." -ForegroundColor Cyan

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# DOWNLOAD LATEST RELEASE
Write-Host "Downloading latest Floord release..." -ForegroundColor Cyan

$Release = Invoke-RestMethod `
    -Uri "https://api.github.com/repos/$Repo/releases/latest"

$Asset = $Release.assets |
        Where-Object { $_.name -eq "floord-windows.exe" } |
        Select-Object -First 1

if (-not $Asset) {
    throw "Windows executable not found in latest GitHub Release."
}

Invoke-WebRequest `
    -Uri $Asset.browser_download_url `
    -OutFile $AppExe

if (!(Test-Path $AppExe)) {
    throw "Floord executable download failed."
}

Write-Host "Downloaded version: $($Release.tag_name)" -ForegroundColor Green

# STOP PREVIOUS INSTANCE
$ExistingTask = Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue

if ($ExistingTask) {
    Write-Host "Stopping existing Floord instance..."

    Stop-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue

    Start-Sleep -Seconds 3
}

# STOP ANY REMAINING INSTANCE
Get-Process -Name "floord" -ErrorAction SilentlyContinue |
        Stop-Process -Force

# CREATE LOG DIRECTORY
New-Item -ItemType Directory -Force -Path "$InstallDir\logs" | Out-Null

# CREATE SCHEDULED TASK ACTION
$Action = New-ScheduledTaskAction `
    -Execute "powershell.exe" `
    -Argument "-NoProfile -WindowStyle Hidden -Command `"Start-Process -FilePath '$AppExe' -ArgumentList '$AppArgs' -WorkingDirectory '$InstallDir' -WindowStyle Hidden`""

# RUN WHEN CURRENT USER LOGS IN
$CurrentUser = [Security.Principal.WindowsIdentity]::GetCurrent().Name

$Trigger = New-ScheduledTaskTrigger `
    -AtLogOn `
    -User $CurrentUser

# TASK SETTINGS
$Settings = New-ScheduledTaskSettingsSet `
    -ExecutionTimeLimit ([TimeSpan]::Zero) `
    -RestartCount 3 `
    -RestartInterval (New-TimeSpan -Minutes 1) `
    -StartWhenAvailable

# REGISTER TASK
Write-Host "Registering Floord startup task..." -ForegroundColor Cyan

Register-ScheduledTask `
    -TaskName $TaskName `
    -Action $Action `
    -Trigger $Trigger `
    -Settings $Settings `
    -Description "Floord background file organizer" `
    -User $CurrentUser `
    -RunLevel Limited `
    -Force | Out-Null

# START FLOORD
Write-Host "Starting Floord..." -ForegroundColor Cyan

Start-ScheduledTask -TaskName $TaskName

Start-Sleep -Seconds 2

# VERIFY TASK STATE
$Task = Get-ScheduledTask -TaskName $TaskName

Write-Host ""
Write-Host "Floord installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Version:      $($Release.tag_name)"
Write-Host "User:         $CurrentUser"
Write-Host "Directory:    $InstallDir"
Write-Host "Task:         $TaskName"
Write-Host "Arguments:    $AppArgs"
Write-Host "Task state:   $($Task.State)"
Write-Host ""
Write-Host "Floord will automatically start when you log in."
