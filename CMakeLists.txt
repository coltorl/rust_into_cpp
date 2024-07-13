cmake_minimum_required(VERSION 3.22)

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 17)

project(
    rust_into_cpp
    LANGUAGES CXX C
)

# clang compiler flags
set(COMPILE_FLAGS
    -fdiagnostics-color=always
    -Wall -Wextra -pedantic -Wconversion -Wundef
    -Wdeprecated -Wweak-vtables -Wshadow
    -Wno-gnu-zero-variadic-macro-arguments
)

find_package(Qt6 REQUIRED COMPONENTS Widgets)
find_package(fmt REQUIRED)

# required to link against rust lib
find_package(OpenSSL REQUIRED)

add_subdirectory(src)
