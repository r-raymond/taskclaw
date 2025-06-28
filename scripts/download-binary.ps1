# Download standalone static binary script for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/r-raymond/taskclaw/main/scripts/download-binary.ps1 | iex

param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"

$REPO = "r-raymond/taskclaw"
$BINARY_NAME = "claw"

Write-Host "Downloading standalone $BINARY_NAME binary..." -ForegroundColor Blue

# Get the latest release version
Write-Host "Fetching latest release..." -ForegroundColor Yellow
try {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
    $latestVersion = $response.tag_name
} catch {
    Write-Host "Failed to get latest version: $_" -ForegroundColor Red
    exit 1
}

Write-Host "Latest version: $latestVersion" -ForegroundColor Green

# Download URL for standalone binary
$downloadUrl = "https://github.com/$REPO/releases/download/$latestVersion/$BINARY_NAME-windows-x86_64-static.exe"

try {
    # Create install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    Write-Host "Downloading standalone binary..." -ForegroundColor Yellow
    
    # Download the binary directly
    $binaryPath = Join-Path $InstallDir "$BINARY_NAME.exe"
    Invoke-WebRequest -Uri $downloadUrl -OutFile $binaryPath
    
    Write-Host "$BINARY_NAME standalone binary installed successfully!" -ForegroundColor Green
    
    # Verify the binary
    Write-Host "Verifying installation..." -ForegroundColor Yellow
    try {
        & $binaryPath --version | Out-Null
        Write-Host "✓ Binary is working correctly" -ForegroundColor Green
    } catch {
        Write-Host "✗ Binary verification failed" -ForegroundColor Red
        exit 1
    }
    
    # Check if install directory is in PATH
    $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Host "Warning: $InstallDir is not in your PATH" -ForegroundColor Yellow
        Write-Host "Would you like to add it to your PATH? (y/n): " -ForegroundColor Yellow -NoNewline
        $response = Read-Host
        if ($response -eq "y" -or $response -eq "Y") {
            $newPath = "$currentPath;$InstallDir"
            [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
            Write-Host "Added $InstallDir to your PATH. Please restart your terminal." -ForegroundColor Green
        } else {
            Write-Host "You can manually add $InstallDir to your PATH later." -ForegroundColor Yellow
        }
    }
    
    Write-Host "Run '$BINARY_NAME --help' to get started!" -ForegroundColor Green
    Write-Host "This is a standalone static binary with no dependencies." -ForegroundColor Blue
    
} catch {
    Write-Host "Failed to download or install: $_" -ForegroundColor Red
    exit 1
}
