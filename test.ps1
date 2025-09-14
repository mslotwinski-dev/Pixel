$exe = "target\debug\pixel.exe"
$input = "test-images\fall.jpg"

# Filters to test
$filters = @(
    "london",
    "paris",
    "venice",
    "milan",
    "madrid",
    "berlin",
    "oslo",
    "warsaw",
    "new_york",
    "los_angeles",
    "las_vegas",
    "miami",
    "rio",
    "tokio",
    "shanghai",
    "dubai",
    "cairo",
    "lagos"
)

$inputName = [System.IO.Path]::GetFileNameWithoutExtension($input)

foreach ($filter in $filters) {
    $outputDir = "dist\$inputName"
    $outputFile = "$outputDir\$filter.png"

    if (-not (Test-Path $outputDir)) {
        New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
    }
    
    & $exe $input --output $outputFile --filter $filter
}