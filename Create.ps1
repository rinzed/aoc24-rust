[CmdletBinding(DefaultParameterSetName = 'date')]
param (
    [Parameter(ParameterSetName="date")]
    [datetime] $Date = (Get-Date),
    [Parameter(ParameterSetName="year-day", Mandatory=$true)]
    [int] $Year,
    [Parameter(ParameterSetName="year-day", Mandatory=$true)]
    [int] $Day
)

if ($PSCmdlet.ParameterSetName -eq "year-day") {
    $Date = New-Object -TypeName DateTime -ArgumentList $Year, 12, $Day
}
if ($PSCmdlet.ParameterSetName -eq "date") {
    $Year = $Date.Year
    $Day = $Date.Day
}

if ($Date.Month -ne 12) {
    Write-Error "Advent of Code only can be a day in December.";
    exit
}
if ($Day -gt 25) {
    Write-Error "Advent of Code is only active until Christmas.";
    exit
}

$Url = "https://adventofcode.com/$Year/day/$Day"
$Page = Invoke-RestMethod $Url
$Result = [regex]::Match($page, "<h2>--- Day \d+: (.+) ---</h2>")
$Title = $Result[0].Groups[1].Value
$Name = "aoc{0:yy}-d{0:dd}" -f $Date
cargo generate --path .\Template\ --name $Name -d "year=$Year" -d "day=$Day" -d "title=$Title"

Set-Location $Name

cargo run