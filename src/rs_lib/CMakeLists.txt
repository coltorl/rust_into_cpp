include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml)
corrosion_experimental_cbindgen(TARGET rs_lib HEADER_NAME "rs_lib.h")
corrosion_add_cxxbridge(rs_to_cxx_lib
        CRATE rs_lib
        FILES lib.rs
)

corrosion_link_libraries(rs_lib c_lib cxx_lib OpenSSL::SSL)
