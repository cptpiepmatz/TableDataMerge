cmake_minimum_required(VERSION 3.15)
project(TableDataMerge)

set(CMAKE_CXX_STANDARD 20)

add_executable(
        # main files
        TableDataMerge src/main.cpp
        src/globalObjects.h
        src/stdLibraries.h
        src/project.rc

        # function files
        src/functions/countEntries.h src/functions/countEntries.cpp
        src/functions/exportEntries.h src/functions/exportEntries.cpp
        src/functions/extractEntries.h src/functions/extractEntries.cpp
        src/functions/normalizeEntries.h src/functions/normalizeEntries.cpp
        src/functions/printHelp.h src/functions/printHelp.cpp

        # class files
        src/classes/DebugOutput.h src/classes/DebugOutput.cpp
        src/classes/FlagList.h src/classes/FlagList.cpp
        )