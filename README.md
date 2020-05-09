<p align="center">
    <img src="icon/icon.svg" width="256"/>
</p>

# TableDataMerge
![GitHub release (latest by date)](https://img.shields.io/github/v/release/derPiepmatz/TableDataMerge)
![GitHub](https://img.shields.io/github/license/derPiepmatz/TableDataMerge)

## About
TableDataMerge is a tool to merge together plain text tables as they are in a
.csv or .dat file.

## Usage
TableDataMerge is a command line tool, hence you'll probably need *cmd*,
*powershell* or similar.

```
TableDataMerge.exe <file(s) to merge...> [option flags...]
```

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