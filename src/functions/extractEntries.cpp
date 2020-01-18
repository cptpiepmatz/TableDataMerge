#include "extractEntries.h"

#include "../globalObjects.h"

void extractEntries(
  string **entries,
  const int &offset,
  const int &width,
  ifstream &file,
  const string &fileType
) {
  smatch match;
  string line;
  int depthIndex = 0;
  if (fileType == ".txt" || fileType == ".dat") {
    regex search(R"(\S+( \S+)*)");
    while (getline(file, line)) {
      for (int i = 0; i < width; i++) {
        regex_search(line, match, search);
        entries[depthIndex][i + offset] = match.str();
        line = match.suffix().str();
      }
      depthIndex++;
    }
  }
  else if (fileType == ".csv") {
    regex search(R"([^;]+)");
    while (getline(file, line)) {
      for (int i = 0; i < width; i++) {
        regex_search(line, match, search);
        entries[depthIndex][i + offset] = match.str();
        line = match.suffix().str();
      }
      depthIndex++;
    }
  }
  else if (fileType == ".m") {
    while (getline(file, line)) {
      if (line.empty())  {
        continue;
      }
      if (line.front() == '%') {
        continue;
      }
      if (line.front() != ' ') {
        int assignIndex = line.find_first_of('=');
        entries[depthIndex][offset] = line.substr(0, assignIndex - 1);
        depthIndex++;
        line = line.substr(assignIndex + 3);
      }
      if (line.find_first_of(']') != line.find_first_not_of(' ')) {
        regex search(R"([\+\-]?\d+([\.,]\d+([eE][\+\-]?\d+)?)?)");
        while (regex_search(line, match, search)) {
          entries[depthIndex][offset] = match.str();
          line = match.suffix().str();
          depthIndex++;
        }
      }
    }
  }
  file.clear();
  file.seekg(0, ios::beg);
}
