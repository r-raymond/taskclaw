# Claw Installation Script for Windows
# Usage: iwr -useb https://raw.githubusercontent.com/r-raymond/taskclaw/main/scripts/install.ps1 | iex

param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"

$REPO = "r-raymond/taskclaw"
$BINARY_NAME = "claw"

Write-Host "Installing $BINARY_NAME..." -ForegroundColor Blue

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

# Download URL
$downloadUrl = "https://github.com/$REPO/releases/download/$latestVersion/$BINARY_NAME-windows-x86_64.zip"

# Create temp directory
$tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }

try {
    Write-Host "Downloading $BINARY_NAME $latestVersion for Windows..." -ForegroundColor Yellow
    
    # Download
    $archivePath = Join-Path $tempDir "archive.zip"
    Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath
    
    Write-Host "Extracting archive..." -ForegroundColor Yellow
    Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force
    
    # Create install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Install binary
    Write-Host "Installing to $InstallDir..." -ForegroundColor Yellow
    $binaryPath = Join-Path $tempDir "$BINARY_NAME.exe"
    $installPath = Join-Path $InstallDir "$BINARY_NAME.exe"
    Copy-Item $binaryPath $installPath -Force
    
    Write-Host "$BINARY_NAME installed successfully!" -ForegroundColor Green
    
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
    
} finally {
    # Cleanup
    Remove-Item $tempDir -Recurse -Force -ErrorAction SilentlyContinue
}
