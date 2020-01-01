#include "stdLibraries.h"

#include "classes/DebugOutput.h"
DebugOutput debugOutput;

#include "classes/FlagList.h"
FlagList flagList;

#include "functions/countEntries.h"
#include "functions/extractEntries.h"
#include "functions/normalizeEntries.h"
#include "functions/exportEntries.h"

#include "functions/printHelp.h"
string buildVersion = "1.0.0";

void printVersion() {
  system("cls");
  cout << "[Running TableDataMerger v" << buildVersion << "]" << endl;
  cout << endl;
}

bool isSupported(const fs::path &filePath) {
  const char *supportedTypes[] = {
    ".txt",
    ".m",
    ".dat",
    ".csv"
  };

  for (const string &supportedType : supportedTypes) {
    if (filePath.extension().string() == supportedType) return true;
  }
  return false;
}

int main(const int argc, const char *argv[]) {
  printVersion();

  // test if any argument is passed
  if (argc == 1) printHelp();

  // check for flags
  cout << "Checking for Flags..." << endl;
  for (int i = 1; i < argc; i++) {
    if (!argv[i]) continue;
    if (argv[i][0] == '-') {
      string flagString = argv[i];
      debugOutput("Testing '" + flagString + "'");
      if (!flagList.setFlag(argv, i, debugOutput)) printHelp();
    }
  }

  // count files
  cout << "Counting File Amount..." << endl;
  int fileAmount = (argc - 1 - flagList.flagAmount);
  debugOutput("Found " + to_string(fileAmount) + " files");
  auto *filePaths = new fs::path[fileAmount];
  int fileIndex = 0;
  for (int i = 1; i < argc; i++) {
    if (argv[i] != nullptr) {
      filePaths[fileIndex] = fs::path(argv[i]);
      fileIndex++;
    }
  }

  // check for valid file types
  cout << "Checking if Files are supported..." << endl;
  for (int i = 0; i < fileAmount; i++) {
    debugOutput(
      "Checking if " + filePaths[i].extension().string() + " is supported"
      );
    if (!isSupported(filePaths[i])) printHelp();
  }

  // create file array
  auto *files = new ifstream[fileAmount];
  for (int i = 0; i < fileAmount; i++) {
    files[i].open(filePaths[i]);
  }

  cout << "Counting Entries..." << endl;

  int width = 0;
  int depth = 0;
  int totalWidth = 0;
  int totalDepth = 0;
  auto *fileWidths = new int[fileAmount];
  for (int i = 0; i < fileAmount; i++) {
    cout << '[' << i + 1 << '/' << fileAmount << ']' << endl;
    debugOutput("Testing " + filePaths[i].string());
    countEntries(
      width,
      depth,
      files[i],
      filePaths[i].extension().string());
    debugOutput("Width for file " + to_string(i + 1) + ": " + to_string(width));
    debugOutput("Depth for file " + to_string(i + 1) + ": " + to_string(depth));
    fileWidths[i] = width;
    totalWidth += width;
    if (totalDepth < depth) totalDepth = depth;
  }

  debugOutput("Total width: " + to_string(totalWidth));
  debugOutput("Total depth: " + to_string(totalDepth));

  auto **tableData = new string*[totalDepth];
  for (int i = 0; i < totalDepth; i++) {
    tableData[i] = new string[totalWidth];
  }

  cout << "Extracting Entries..." << endl;

  int offset = 0;
  for (int i = 0; i < fileAmount; i++) {
    cout << '[' << i + 1 << '/' << fileAmount << ']' << endl;
    debugOutput("Extracting " + filePaths[i].string());
    debugOutput("Used offset " + to_string(offset));
    extractEntries(
      tableData,
      offset,
      fileWidths[i],
      files[i],
      filePaths[i].extension().string());
    offset += fileWidths[i];
  }

  cout << "Normalizing Entries..." << endl;
  normalizeEntries(tableData, totalWidth, totalDepth);

  string outputPath = fs::current_path().string() + R"(\Output)";
  for (int i = 0; i < fileAmount; i++) {
    outputPath.append("_" + filePaths[i].stem().string());
  }
  if(flagList.outputLatex) outputPath.append(".tex");
  else outputPath.append(".dat");
  debugOutput("Output data to '" + outputPath + "'");

  ofstream outputFile(outputPath);
  cout << "Exporting Entries..." << endl;
  exportEntries(outputFile, tableData, totalWidth, totalDepth);

  if (debugOutput.useDebug) {
    debugOutput("Printing results...");
    for (int i = 0; i < totalDepth; i++) {
      for (int j = 0; j < totalWidth; j++) {
        cout << tableData[i][j] << "\t";
      }
      cout << endl;
    }
  }

  outputFile.close();
  cout << "DONE" << endl;
  if (flagList.usePause) system("pause");
}
