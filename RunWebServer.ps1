param(
    [string]
    $BasePath,
    # By default run both of the servers. 
    # Eventually add an option to run the them standalone or in a docker container
    [switch]
    $RunApi,
    [switch]
    $RunWeb,
    [switch]
    $LaunchDocker
)

function RunAPI {
    Write-Output "Running API ------------------------------------------------------------------------------------------------------------------------------------------"
    Push-Location -Path $pathToAPI
    cargo run
    Pop-Location
    Push-Location -Path $BasePath
}

function RunWeb {
    Write-Output "Running Web ------------------------------------------------------------------------------------------------------------------------------------------"
    Push-Location -Path $pathToWeb
    npm run dev
    Pop-Location
    Push-Location -Path $BasePath
}
    
function RunDockerCompose {
    Write-Output "------------------------------------------------------------------------------------------------------------------------------------------"
    Push-Location -Path $pathToAPI
    cargo run
    Pop-Location
}
    
function PauseForConformation {
    Read-Host -Prompt "---------- Execution Paused Please Press Any Key To Continue ----------"
}
        
# Run full stack by default
if ($BasePath -eq "") {
    $BasePath = "C:\MYDevelopment"
    Push-Location -Path $BasePath
}
            
$pathToAPI = "$BasePath\rust_api"
$pathToCommon = "$BasePath\jojo_common"          
$pathToWeb = "$BasePath\sk_web"
            
if ($PSBoundParameters.ContainsKey('RunApi')) {
    RunAPI
}
                
if ($PSBoundParameters.ContainsKey('RunWeb')) {
    RunWeb
}
                    
if ($PSBoundParameters.ContainsKey('LaunchDocker')) {
    RunDockerCompose       
}
else {   
    RunAPI
    RunWeb
    Push-Location -Path $pathToCommon
}
