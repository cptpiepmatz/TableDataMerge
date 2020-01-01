#ifndef TABLEDATA_EXTRACTENTRIES_H
#define TABLEDATA_EXTRACTENTRIES_H

#include "../stdLibraries.h"

void extractEntries(
  string **entries,
  const int &offset,
  const int &width,
  ifstream &file,
  const string &fileType
  );

#endif //TABLEDATA_EXTRACTENTRIES_H
