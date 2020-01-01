#include "DebugOutput.h"

DebugOutput::DebugOutput() {
  this->useDebug = false;
}

void DebugOutput::operator()(const string &debugString) {
  if (this->useDebug) {
    cout << "[DEBUG] " << debugString << endl;
  }
}