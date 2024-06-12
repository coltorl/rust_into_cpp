cmake_minimum_required(VERSION 3.21)


set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 17)

project(
    rust_into_cpp
    LANGUAGES CXX C
)

set(COMPILE_FLAGS
    -Wall -Wextra -Wconversion -Wundef
    -Wdeprecated -Wweak-vtables -Wshadow
    -Wno-gnu-zero-variadic-macro-arguments
)

add_subdirectory(src)
