#include "countEntries.h"

#include "../globalObjects.h"

// function for .dat .txt .csv
void findEntries(
  int &outputWidth, int &outputDepth, ifstream &file, regex &searchRegex) {
  string line;
  smatch match;
  int maxWidth = 0;
  int lineCount = 0;
  while (getline(file, line)) {
    int findings = 0;
    while (regex_search(line, match, searchRegex)) {
      findings++;
      line = match.suffix().str();
    }
    lineCount++;
    if (findings > maxWidth) maxWidth = findings;
  }
  outputWidth = maxWidth;
  outputDepth = lineCount;
}

// function for .m per line
int findEntriesMLine(string line) {
  int endDotsPos = line.rfind("...");
  line = line.substr(0, endDotsPos);
  regex search(R"([\+\-]?[\d]+([\.,]\d+([eE][\+\-]?\d+)?)?[;\] ]+)");
  smatch match;
  int findings = 0;
  while (regex_search(line, match, search)) {
    findings++;
    line = match.suffix().str();
  }
  return findings;
}
// function for .m
void findEntriesM(int &outputWidth, int &outputDepth, ifstream &file) {
  string line;
  smatch match;
  int findings = 0;
  while (getline(file, line)) {
    if (line.empty()) continue;
    if (line.front() == '%') continue;
    if (line.front() != ' ') {
      int assignIndex = line.find_first_of('=');
      findings++;
      findings += findEntriesMLine(line.substr(assignIndex + 3));
      continue;
    }
    if (line.find_first_of(']') != line.find_first_not_of(' ')) {
      findings += findEntriesMLine(line);
    }
  }
  outputDepth = findings;
  outputWidth = 1;
}

void countEntries(int &width, int &depth, ifstream &file, const string &fileType) {
  if (fileType == ".txt" || fileType == ".dat") {
    regex search(R"(\S+( \S+)*)");
    findEntries(width, depth, file, search);
  }
  else if (fileType == ".csv") {
    regex search(R"([^;]+;)");
    findEntries(width, depth, file, search);
  }
  else if (fileType == ".m") {
    findEntriesM(width, depth, file);
  }
  file.clear();
  file.seekg(0, ios::beg);
}