@echo off
REM Check if the correct number of arguments are provided
if "%~2" == "" (
    echo Usage: %0 ^<secret_name^> ^<data_file^>
    exit /b 1
)

REM Assign arguments to variables
set "SECRET_NAME=%~1"
set "DATA_FILE=%~2"
set "IMAGE_NAME=azure_vault_file_upload"

REM Execute the docker run command
docker run ^
  -v %cd%/data/.env:/usr/src/app/.env ^
  -v %cd%/data:/usr/src/app/data ^
  %IMAGE_NAME% ^
  sh -c "sh azure_upload.sh --secret-name %SECRET_NAME% --file-path data/%DATA_FILE%"
