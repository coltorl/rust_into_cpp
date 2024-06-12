add_library(c_lib c_lib.c)

target_include_directories(c_lib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
