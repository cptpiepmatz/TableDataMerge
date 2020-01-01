#ifndef TABLEDATA_DEBUGOUTPUT_H
#define TABLEDATA_DEBUGOUTPUT_H

#include "../stdLibraries.h"

class DebugOutput {
public:
    DebugOutput();

    bool useDebug;

    void operator()(const string &debugString);
};


#endif //TABLEDATA_DEBUGOUTPUT_H
