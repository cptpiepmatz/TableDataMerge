<p align="center">
  <a href="https://github.com/cptpiepmatz/great-on-deck-search">
    <img width="256" src="./icon/icon.svg">
  </a>
</p>
<h1 align="center">TableDataMerge</h1>
<p align="center">
  <b>🔀 Merge plain text tables together.</b>
</p>
<br>

<div align="center">

  [![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fcptpiepmatz%2FTableDataMerge%2Fmain%2FCargo.toml&query=%24.package.version&prefix=v&style=for-the-badge&label=version)](https://github.com/cptpiepmatz/TableDataMerge/releases)
  [![License](https://img.shields.io/github/license/cptpiepmatz/TableDataMerge?style=for-the-badge)](https://github.com/cptpiepmatz/TableDataMerge/blob/master/LICENSE)
  [![Rust](https://img.shields.io/badge/written%20in-rust-orange?style=for-the-badge)](https://www.rust-lang.org)

</div>



## About

TableDataMerge (alias 'tdm') is a versatile command-line tool designed to merge 
plain-text tables horizontally or vertically. 
It supports various input and output data types, and offers cell formatting 
options, including number formatting, prefixes, and suffixes.


## Features

- Merge tables horizontally or vertically
- Supports various input and output data types 
- Offers cell formatting options 
- Number formatting 
- Ability to add prefixes and suffixes to cell values


## Getting Started

Built binaries for TableDataMerge are available for Windows and Linux, and can 
be found at the 
[project's releases](https://github.com/cptpiepmatz/TableDataMerge/releases) 
page. 
Simply download the appropriate binary for your system.

To make the use of the tool more convenient, consider adding the binary to your 
PATH environment variable.


## Supported Data Types

TableDataMerge supports various input and output data types, providing great flexibility to fit different use cases.


### Input Data Types

- **CSV**:
  [Comma Separated Values](https://en.wikipedia.org/wiki/Comma-separated_values)
  is a simple and widely used data format that stores tabular data
  (numbers and text) in plain-text form.

- **DAT**:
  [DAT](https://en.wikipedia.org/wiki/DAT_(file_format))
  is a generic file extension for data files which can contain basic text or
  binary, although in this context it is used to denote text-based tabular data
  similar to CSV.

- **JSON**:
  [JavaScript Object Notation](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON)
  is a lightweight data-interchange format that is easy to read and write.
  It is based on a subset of JavaScript language.

- **M**:
  Matlab tables, where the table data from Matlab is saved in a plain-text form.


### Output Data Types

- **CSV**:
  [Comma Separated Values](https://en.wikipedia.org/wiki/Comma-separated_values)
  for easy import into various spreadsheet tools or data analysis tools.

- **DAT**:
  [DAT](https://en.wikipedia.org/wiki/DAT_(file_format))
  is often used for data exchange between applications written in different
  programming languages.

- **JSON**:
  [JavaScript Object Notation](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON)
  for further use in web-based applications and data processing tools.

- **MD**:
  [Markdown](https://en.wikipedia.org/wiki/Markdown)
  format for easy reading and writing of the data in text editors and for
  display on websites like GitHub.

- **TEX**:
  [LaTeX](https://www.latex-project.org/about/)
  source code for use in typesetting systems like LaTeX, providing beautifully
  formatted and highly customizable output.


## Usage

```shell
tdm [OPTIONS] <TO> <FILES>...
```

- **TO**: Output data type. 
  It specifies the output data type. 
  Possible values include: `csv`, `dat`, `tex`, `md`, `json`.

- **FILES**: File paths. 
  It specifies input file paths and optional additional data. 
  Example: `example.csv`.


## Options

- **-o, --out**: 
  Sets the output file path. 

- **-p, --precision**: 
  Sets the number of decimal places for numerical values, which will be 
  correctly rounded. 

- **-d, --decimal-sep**: 
  Sets the decimal separator for numerical values. 
  Default is `dot`. 

- **-e, --exponent**: 
  Enables scientific notation for numerical cells (e.g., '1.234e+05'). 

- **-s, --sign**: 
  Forces a sign on every number. 
  By default, only negative values have a sign.

- **-H, --hline**: 
  Inserts `\hline` between lines when using `tex` output format. 

- **-c, --csv-sep**: 
  Specifies the value separator for `csv` output format. 
  Default is `,`. 

- **-P, --prefix**: 
  Sets prefixes for numerical cells. 

- **-S, --suffix**: 
  Sets suffixes for numerical cells. 

- **-v, --vertical**: 
  Stacks tables vertically instead of concatenating them horizontally.

- **-h, --help**: 
  Prints help information.


## Examples

### Merging Two Tables with Precision and CSV Separator

```shell
tdm csv -o merged.csv -p 2 -c , table1.csv table2.dat
```

This command merges `table1.csv` and `table2.dat`, sets the output data type as 
`csv`, sets the number of decimal places as `2`, and uses `,` as the csv 
delimiter. 
The resulting table will be saved in `merged.csv`.


### Using Prefix and Suffix Options

```shell
tdm tex -P 0..3:$ -S 0..3:$ -S "1: m$" table.dat
```

This command does not merging operation since only one table is given as input.
It will however do all the transformations given.

First, the output format is specified as LaTeX ('tex') through the `tdm tex` 
portion of the command. 
This ensures the final output will be suitable for inclusion in a LaTeX 
document.

Next, the `-P 0..3:$` operation sets a prefix of a dollar sign `$` to the first 
three numerical columns of the table. 
This is achieved through the `0..3`: range specification, which is 0-indexed, 
meaning it starts counting from 0.

Following that, the `-S 0..3:$` operation appends a suffix of a dollar sign `$` 
to the same first three numerical columns. 
So far, the command has instructed the program to wrap the values in the first 
three numerical columns in dollar signs, effectively displaying them in math 
mode for latex.

However, the `-S "1: m$"` command then specifies a change to the second column. 
It alters the suffix for numerical values in the second column 
(again, remembering that column counting starts from 0), replacing the previous 
`$` suffix with ` m$`.
Rules applied later in the command will always override rules applied before.

So if we start this table:

| Column1 | Column2 | Column3 | Column4 |
|---------|---------|---------|---------|
| 1       | 2       | 3       | 4       |
| 5       | 6       | 7       | 8       |

The output would look something like this:
```tex
Column1 & Column2 & Column3 & Column4 \\
$1$ & $2 m$ & $3$ & 4 \\
$5$ & $6 m$ & $7$ & 8
```


## Contributing

If you're interested in contributing to the development of TableDataMerge, 
feel free to create a pull request. 
Please ensure that your code follows the existing style guidelines. <br>
(`cargo fmt`, nothing more 😉)


## Changelog

For a detailed list of changes in each version, please refer to the 
[CHANGELOG.md](https://github.com/cptpiepmatz/TableDataMerge/blob/master/CHANGELOG.md) 
in the repository.


## License

TableDataMerge is under the MIT license. 
For more details, please see the 
[LICENSE](https://github.com/cptpiepmatz/TableDataMerge/blob/master/LICENSE).
