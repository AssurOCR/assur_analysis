cmake_minimum_required(VERSION 3.10)

# Set the project name
project(smalltess)

# Set the C++ standard to C++11
set(CMAKE_CXX_STANDARD 14)

# Add the source files
set(SOURCES main.cpp)

# Find the Tesseract and Leptonica include directories
if (CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
    execute_process(COMMAND brew --prefix tesseract OUTPUT_VARIABLE TESSERACT_PREFIX OUTPUT_STRIP_TRAILING_WHITESPACE)
    execute_process(COMMAND brew --prefix leptonica OUTPUT_VARIABLE LEPTONICA_PREFIX OUTPUT_STRIP_TRAILING_WHITESPACE)
    execute_process(COMMAND brew --prefix opencv OUTPUT_VARIABLE OPENCV_PREFIX OUTPUT_STRIP_TRAILING_WHITESPACE)
    set(TESSERACT_INCLUDE_DIRS "${TESSERACT_PREFIX}/include")
    set(LEPTONICA_INCLUDE_DIRS "${LEPTONICA_PREFIX}/include")
    set(OPENCV_INCLUDE_DIRS "${OPENCV_PREFIX}/include/opencv4")
else()
    set(TESSERACT_INCLUDE_DIRS "/usr/local/include/tesseract")
    set(LEPTONICA_INCLUDE_DIRS "/usr/local/include")
    set(OPENCV_INCLUDE_DIRS "/usr/local/include")
endif()

# Add the include directories
include_directories(include ${TESSERACT_INCLUDE_DIRS} ${LEPTONICA_INCLUDE_DIRS} ${OPENCV_INCLUDE_DIRS})

# Find the Tesseract and Leptonica libraries
if (CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
    find_library(TESSERACT_LIBRARY tesseract PATHS "${TESSERACT_PREFIX}/lib")
    find_library(LEPTONICA_LIBRARY lept PATHS "${LEPTONICA_PREFIX}/lib")
    find_package( OpenCV REQUIRED PATHS "${OPENCV_PREFIX}/lib")
else()
    find_library(TESSERACT_LIBRARY tesseract)
    find_library(LEPTONICA_LIBRARY lept)
    find_package( OpenCV REQUIRED PATHS "${OPENCV_PREFIX}/lib")
endif()

# Add the

# Add the library target
add_library(smalltess SHARED ${SOURCES})

# Link the Tesseract and Leptonica libraries to the target
target_link_libraries(smalltess PRIVATE ${TESSERACT_LIBRARY} ${LEPTONICA_LIBRARY} ${OpenCV_LIBS})

install(TARGETS smalltess DESTINATION .)
