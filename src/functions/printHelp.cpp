#include "printHelp.h"

void printLine() {
  cout << endl;
}

void printLine(const string &lineText) {
  cout << lineText << endl;
}

void printHelp() {
  system("cls");
  cout << "[Running TableDataMerger v" << buildVersion << "]" << endl;
  printLine();
  printLine("Merges multiple data tables to one.");
  printLine();
  printLine();
  printLine("Usage: TableDataMerge.exe <file(s) to merge...> [option flags...]");
  printLine();
  printLine("Option Flags:");
  printLine();
  printLine("  -precision <digits after seperator>\tChanges the digits after every separator on numbers");
  printLine("  -comma\t\t\t\tConverts all inputs to use commas instead of dots as separator");
  printLine("  -science\t\t\t\tEvery number will be displayed in scientific notation");
  printLine("  -sign\t\t\t\t\tEvery number will be displayed signed");
  printLine("  -nopause\t\t\t\tAfter runtime the application will no longer ask for input");
  printLine("  -latex\t\t\t\tOutputs merged table in latex format");
  printLine("  -hline\t\t\t\tIf used with '-latex' between every line a '\\hline' will be printed");
  printLine("  -debug\t\t\t\tPrints a shit ton of data");
  printLine("  -mathmode \t\t\t\tIf used with '-latex' every cell will be inserted into math mode");

  if (flagList.usePause) system("pause");

  exit(0);
}