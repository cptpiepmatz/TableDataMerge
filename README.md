<p align="center">
    <img src="icon/icon.svg" width="256"/>
</p>

# TableDataMerge
![GitHub release (latest by date)](https://img.shields.io/github/v/release/derPiepmatz/TableDataMerge)
![GitHub](https://img.shields.io/github/license/derPiepmatz/TableDataMerge)

## About
TableDataMerge is a tool to merge together plain text tables as they are in a
.csv or .dat file.

## Easy Install (recommended)
If you just want to use this. Grab the easy installer <a href="https://raw.githubusercontent.com/derPiepmatz/TableDataMerge/master/EasySetup.ps1" download="EasySetup.ps1">here</a>.

## Usage
If you used the easy installer you can just drag and drop files to the shortcut on your desktop.

If you want to use it differently or don't want to use the 
easy installer you'll probably need *cmd*,
*powershell* or similar.

```
TableDataMerge.exe <file(s) to merge...> [option flags...]
```

You can download it yourself [here](https://github.com/derPiepmatz/TableDataMerge/releases/latest).

### Flags
These allow some extra functionality

Flag | Functionality
-----| -------------
-precision \<digits after seperator\> | Changes the digits after every separator on numbers
-comma | Converts all inputs to use commas instead of dots as separator
-science | Every number will be displayed in scientific notation
-sign | Every number will be displayed signed
-nopause | After runtime the application will no longer ask for input
-latex | Outputs merged table in latex format
-hline | If used with '-latex' between every line a '\hline' will be printed
-debug | Prints a shit ton of data
-mathmode | If used with '-latex' every cell will be inserted into math mode

## Supported File Formats
- .dat(separated by tabs and 2+ spaces)
- .csv
- .txt(same as dat)
- .m(matlab export file format)

## Changelog
For Changelog, check [CHANGELOG.md](CHANGELOG.md).

## License
This application is released under the *MIT license*.
Please see [LICENSE.md](LICENSE.md) for more information.