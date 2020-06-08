$source = "https://github.com/derPiepmatz/TableDataMerge/releases/latest/download/TableDataMerge.exe"
$installPath = "$Home\AppData\Local\TableDataMerge"
$outFile = "$installPath\TableDataMerge.exe"
$tdmOutput = "$installPath\Output"
$tdmLink = "$Home\Desktop\TDM"
$shortcutArguments = @("-latex", "-latex -mathmode", "-latex -hline")

# create install directory
if (-Not (Test-Path -Path $installPath -PathType Container)) {
  New-Item -Path "$Home\AppData\Local" -Name "TableDataMerge" -ItemType "directory"
}

# ensure you have the newest version downloaded
if (Test-Path -Path $outFile -PathType Leaf) {
  Remove-Item -Path $outFile
}
Invoke-WebRequest -Uri $source -OutFile $outFile

# create shortcuts on desktop
foreach ($args in $shortcutArguments) {
  if (-Not (Test-Path -Path "$tdmLink $args.lnk" -PathType Leaf)) {
    $WshShell = New-Object -comObject Wscript.Shell
    $Shortcut = $WshShell.CreateShortcut("$tdmLink $args.lnk")
    $Shortcut.TargetPath = $outFile
    $Shortcut.Arguments = "$args -nopause"
    $Shortcut.WorkingDirectory = $tdmOutput
    $Shortcut.Save()
  }
}

# create output shortcut to desktop
$dirIconSource = "https://raw.githubusercontent.com/derPiepmatz/TableDataMerge/master/icon/dirIcon.ico"
if (-Not(Test-Path -Path "$installPath/dirIcon.ico" -PathType Leaf)) {
  Invoke-WebRequest -Uri $dirIconSource -OutFile "$installPath/dirIcon.ico"
  attrib +h "$installPath/dirIcon.ico"
}
if (-Not (Test-Path -Path "$tdmLink Output.lnk" -PathType Leaf)) {
  $WshShell = New-Object -comObject Wscript.Shell
  $Shortcut = $WshShell.CreateShortcut("$tdmLink Output.lnk")
  $Shortcut.TargetPath = $tdmOutput
  $Shortcut.IconLocation = "$installPath/dirIcon.ico"
  $Shortcut.Working
  $Shortcut.Save()
}
if (-Not (Test-Path -Path $tdmOutput -PathType Container)) {
  New-Item -Path "$installPath" -Name "Output" -ItemType "directory"
}

# refresh shell
$code = @'
  [System.Runtime.InteropServices.DllImport("Shell32.dll")]
  private static extern int SHChangeNotify(int eventId, int flags, IntPtr item1, IntPtr item2);

  public static void Refresh()  {
      SHChangeNotify(0x8000000, 0x1000, IntPtr.Zero, IntPtr.Zero);
  }
'@

Add-Type -MemberDefinition $code -Namespace WinAPI -Name Explorer
[WinAPI.Explorer]::Refresh()