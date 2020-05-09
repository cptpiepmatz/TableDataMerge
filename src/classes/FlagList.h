#ifndef TABLEDATA_FLAGLIST_H
#define TABLEDATA_FLAGLIST_H

#include "../stdLibraries.h"

#include "DebugOutput.h"

class FlagList {
private:
    const char *possibleFlags[9] {
        // all the flags you can enter
        "latex",
        "comma",
        "hline",
        "precision",
        "nopause",
        "debug",
        "science",
        "sign",
        "mathmode"
    };

public:
    bool outputLatex;
    bool useComma;
    bool insertHLine;
    bool usePrecision;
    int precision;
    bool usePause;
    bool forceScientificNotation;
    bool forceSigningNumbers;
    bool useMathMode;

    int flagAmount;

    FlagList();

    bool setFlag(const char **arguments, int index, DebugOutput &debugOutput);
};


#endif //TABLEDATA_FLAGLIST_H
