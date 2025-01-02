Get-ChildItem | Where-Object { $_.Name -like "aoc24-d*" } | ForEach-Object {
    Push-Location $_

    cargo run -r

    Pop-Location
}