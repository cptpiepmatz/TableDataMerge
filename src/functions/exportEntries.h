#ifndef TABLEDATA_EXPORTENTRIES_H
#define TABLEDATA_EXPORTENTRIES_H

#include "../stdLibraries.h"

#include "../classes/FlagList.h"

void exportEntries(
  ofstream &file,
  string **entries,
  const int &width,
  const int &depth
  );

#endif //TABLEDATA_EXPORTENTRIES_H
