#include "normalizeEntries.h"

#include "../globalObjects.h"
#include "../classes/DebugOutput.h"

void replaceDotWComma(string &entry) {
  for (char &i : entry) {
    if (i == '.') {
      i = ',';
      return;
    }
  }
}

void replaceCommaWDot(string &entry) {
  for (char &i : entry) {
    if (i == ',') {
      i = '.';
      return;
    }
  }
}

void replaceToPrecision(
  string &entry,
  const char &divider,
  const int &precision
) {
  int dividerIndex = entry.find_first_of(divider);
  if (precision == 0) {
    entry = entry.substr(0, dividerIndex);
    return;
  }
  entry.resize(
    entry.length()
    + precision
    - entry.substr(dividerIndex + 1).length(),
    '0');
}

void replaceToPrecisionScientific(
  string &entry,
  const char &divider,
  const int &precision
  ) {
  int dividerIndex = entry.find_first_of(divider);
  int eIndex = entry.find_first_of('e');
  if (eIndex == entry.size() - 1) eIndex = entry.find_first_of('E');
  if (precision == 0) {
    entry = entry.substr(0, dividerIndex) + entry.substr(eIndex);
    entry.resize(entry.length());
    return;
  }
  string ePart = entry.substr(eIndex);

  bool setToZero = false;
  for (char & i : entry) {
    if (i == 'e' || i == 'E') setToZero = true;
    if (setToZero) i = '0';
  }
  entry.resize(entry.length() + precision - (eIndex - dividerIndex), '0');
  entry.replace(entry.length() - ePart.length() + 1, ePart.length(), ePart);
}

void appendDecimal(string &entry) {
  regex search(R"([\+\-]?\d+)");
  smatch match;
  regex_match(entry, match, search);
  if (match.empty()) return;
  debugOutput("Appending decimals for " + entry);
  entry.resize(entry.length() + 2, '0');
  entry[entry.length() - 2] = '.';
}

void addSigning(string &entry) {
  regex search(R"(\d)");
  smatch match;
  regex_search(entry, match, search);
  if (!match.empty() && match.prefix().str().empty()) {
    debugOutput("Adding signing to " + entry);
    entry = "+" + entry;
  }
}

void scientify(string &entry, const char &divider) {
  int dividerIndex = entry.find_first_of(divider);

  int notZeroIndex = entry.find_first_not_of('0');
  if (notZeroIndex == dividerIndex) {
    notZeroIndex = entry.substr(dividerIndex + 1).find_first_not_of('0');
  }

  int exponent = dividerIndex - notZeroIndex - 1;
  string exponentString = to_string(exponent);
  if (exponent > 0) exponentString.insert(0, "+");
  if (abs(exponent) < 10) exponentString.insert(1, "0");
  // i want at least two digits for the exponent
  exponentString.insert(0, "e");

  entry = entry.substr(0, dividerIndex) + entry.substr(dividerIndex + 1);
  entry.insert(notZeroIndex + 1, ",");
  entry.erase(0, notZeroIndex);
  entry.insert(entry.length(), exponentString);
}

void normalizeEntries(
  string **entries,
  const int &width,
  const int &depth
) {
  regex searchDot(R"([\+\-]?\d+\.\d+)");
  regex searchComma(R"([\+\-]?\d+,\d+)");
  regex searchDotScientific(R"([\+\-]?\d+\.\d+[eE][\+\-]\d+)");
  regex searchCommaScientific(R"([\+\-]?\d+,\d+[eE][\+\-]\d+)");
  smatch match;

  if (flagList.precision != 0) {
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        appendDecimal(entries[i][j]);
      }
    }
  }

  if (flagList.forceSigningNumbers) {
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        addSigning(entries[i][j]);
      }
    }
  }

  if (flagList.forceScientificNotation) {
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        regex_match(entries[i][j], match, searchComma);
        if (!match.empty()) {
          debugOutput("Scientifies " + entries[i][j]);
          scientify(entries[i][j], ',');
        }
        appendDecimal(entries[i][j]);
        regex_match(entries[i][j], match, searchDot);
        if (!match.empty()) {
          debugOutput("Scientifies " + entries[i][j]);
          scientify(entries[i][j], '.');
        }
      }
    }
  }

  if (flagList.useComma) {
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        // non-scientific notation
        regex_match(entries[i][j], match, searchDot);
        if (!match.empty()) {
          debugOutput("Replacing dots with commas for " + entries[i][j]);
          replaceDotWComma(entries[i][j]);
        }
        regex_match(entries[i][j], match, searchComma);
        if (!match.empty() && flagList.usePrecision) {
          debugOutput("Fixing precision for " + entries[i][j]);
          replaceToPrecision(entries[i][j], ',', flagList.precision);
        }

        // scientific notation
        regex_match(entries[i][j], match, searchDotScientific);
        if (!match.empty()) {
          debugOutput("Replacing dots with commas for " + entries[i][j]);
          replaceDotWComma(entries[i][j]);
        }
        regex_match(entries[i][j], match, searchCommaScientific);
        if (!match.empty() && flagList.usePrecision) {
          debugOutput("Fixing precision for " + entries[i][j]);
          replaceToPrecisionScientific(entries[i][j], ',', flagList.precision);
        }
      }
    }
  }
  else {
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        // non-scientific notation
        regex_match(entries[i][j], match, searchComma);
        if (!match.empty()) {
          debugOutput("Replacing commas with dots for " + entries[i][j]);
          replaceCommaWDot(entries[i][j]);
        }
        regex_match(entries[i][j], match, searchDot);
        if (!match.empty() && flagList.usePrecision) {
          debugOutput("Fixing precision for " + entries[i][j]);
          replaceToPrecision(entries[i][j], '.', flagList.precision);
        }

        // scientific notation
        regex_match(entries[i][j], match, searchCommaScientific);
        if (!match.empty()) {
          debugOutput("Replacing commas with dots for " + entries[i][j]);
          replaceCommaWDot(entries[i][j]);
        }
        regex_match(entries[i][j], match, searchDotScientific);
        if (!match.empty() && flagList.usePrecision) {
          debugOutput("Fixing precision for " + entries[i][j]);
          replaceToPrecisionScientific(entries[i][j], '.', flagList.precision);
        }
      }
    }
  }
}
