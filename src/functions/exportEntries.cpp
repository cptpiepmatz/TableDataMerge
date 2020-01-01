#include "exportEntries.h"

#include "../globalObjects.h"

void exportEntries(
  ofstream &file,
  string **entries,
  const int &width,
  const int &depth) {
  if (flagList.outputLatex) {
    // output separated with '&' for latex
    for (int i = 0; i < depth; i++) {
      for (int j = 0; j < width; j++) {
        file << entries[i][j];
        if (j != (width - 1)) file << " & ";
      }
      if (i != (depth - 1)) {
        file << R"( \\)" << endl;
        if (flagList.insertHLine) {
          // in latex this will add horizontal lines between every row
          file << R"(\hline)" << endl;
        }
      }
    }
    return;
  }

  // standard tab divided .dat file
  for (int i = 0; i < depth; i++) {
    for (int j = 0; j < width; j++) {
      file << entries[i][j];
      if (j != (width - 1)) file << "\t";
    }
    if (i != (depth - 1)) file << endl;
  }
}