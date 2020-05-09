#include "FlagList.h"

#include "DebugOutput.h"

FlagList::FlagList() {
  this->outputLatex = false;
  this->useComma = false;
  this->insertHLine = false;
  this->usePrecision = false;
  this->precision = 0;
  this->usePause = true;
  this->forceScientificNotation = false;
  this->forceSigningNumbers = false;
  this->useMathMode = false;

  this->flagAmount = 0;
}

bool FlagList::setFlag(
  const char **arguments,
  const int index,
  DebugOutput &debugOutput
  ) {
  string stringArgument = arguments[index];
  string flagName = stringArgument.substr(1);

  if (flagName == this->possibleFlags[0]) {
    // "latex" flag
    this->outputLatex = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[1]) {
    // "comma" flag
    this->useComma = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[2]) {
    // "hline" flag
    this->insertHLine = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[3]) {
    // "precision" flag
    if (!arguments[index + 1]) return false;
    this->usePrecision = true;
    this->precision = stoi(arguments[index + 1]);
    arguments[index] = nullptr;
    arguments[index + 1] = nullptr;
    this->flagAmount += 2;
    return true;
  }
  if (flagName == this->possibleFlags[4]) {
    // "nopause" flag
    this->usePause = false;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[5]) {
    // "debug" flag
    debugOutput.useDebug = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[6]) {
    // "science" flag
    this->forceScientificNotation = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[7]) {
    // "sign" flag
    this->forceSigningNumbers = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  if (flagName == this->possibleFlags[8]) {
    // "mathmode" flag
    this->useMathMode = true;
    arguments[index] = nullptr;
    this->flagAmount++;
    return true;
  }
  return false;
}
