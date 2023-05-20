# Changelog

## [2.0.0-rc.1] - 2023-05-20

### 🛠️ Improvements
- Rewritten the entire application in Rust #Oxidized
- Overhauled CLI experience, use `tdm` now
- New Icon 🎨
- Precision now actually rounds correctly
- Application is now standalone binary

### ✨ Features
- Parsing CSV files allow custom delimiter
- CSV delimiter can now be customized
- Custom pre- and suffixes
- Vertical stacking of tables

### 🗑️ Removed
- Mathmode, use pre- and suffixes instead

## [1.0.1] - 2020-01-18

### Changed
- Fixed using the flags "sign" and "precision" together
- Fixed ".m" files with only one element at the bottom line

## [1.0.2] - 2020-01-18

### Changed
- Fixed file handling if no file was given but flags were set

## [1.1.0] - 2020-05-09

### Added
- Added a flag for latex math environment called "mathmode"

## [1.1.1] - 2020-05-09

### Changed
- Fixed incorrect counting on trailing semicolons of csv files

## [1.1.2] - 2020-05-09

### Changed
- Updated error message if file type is not supported
- Fixed empty '$$' (latex does not like that)
