---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
---

# Integrating Rust into an Existing <br/>C/C++ Codebase using CMake

---

# How To Boil a Frog

---

# How To Boil a Frog

- Some of us can't simply rewrite our app in Rust
- But we might be able to pull off a minimally disruptive refactor using fairly "industry standard" tools (CMake)
- Corrosion (formerly known as cmake-cargo) is a project that attempts to deliver on that

---

# First, Introductions

- CMake is a popular tool (with accompanying scripting language) for writing cross-platform build scripts for C and C++ projects
- It's scripting language's syntax is best described as ugly

```cmake
set(my_list "string1;string2;string3")
```
```cmake
target_include_directories(
  ${PROJECT_NAME} PUBLIC $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>
                         $<INSTALL_INTERFACE:include/${PROJECT_NAME}-${PROJECT_VERSION}>
)
```
---
# First, Introductions

- It's documentation leaves a lot to be desired 
  - [How to Use CMake Without the Agonizing Pain - Part 1](https://alexreinking.com/blog/how-to-use-cmake-without-the-agonizing-pain-part-1.html)
  - However, it is quickly getting better
- But it works

---
# Scripting
To define a library:

```cmake
add_library(cxx_lib
    cxx_lib.cpp
)

target_include_directories(cxx_lib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
```

To define an executable:
```cmake
add_executable(cxx_app
    main.cpp
)
```

---
# Scripting

To link to a target:
```cmake
target_link_libraries(cxx_app PRIVATE 
    Qt6::Widgets fmt::fmt
    rs_lib rs_to_cxx_lib
)
```

---
# Scripting

To run Qt voodoo:
```cmake
qt_standard_project_setup()

qt_add_executable(cxx_app
    main.cpp

    mainwindow.hpp
    mainwindow.cpp
    mainwindow.ui
)
```

---
# Now with Rust!!!
Project Structure:
- CMakeLists.txt
- src/
    - c_lib
    - cxx_app
    - cxx_lib
    - rs_lib

---
# Now with Rust!!!
```toml
[package]
name = "rs_lib"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"
cxx = "1.0"
cxx-build = "1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }

[lib]
crate-type = ["staticlib"]
```
---
# Functions Exposed
```rust
#[cxx::bridge]
mod ffi {
    struct QuoteFfi {
        q: String,
        a: String,
        h: String,
    }
    struct QuoteResult {
        success: bool,
        data: QuoteFfi,
        msg: String,
    }
    extern "Rust" {
        fn rs_lib_hello() -> String;
        unsafe fn rs_lib_c_hello() -> String;
        fn rs_lib_cxx_hello() -> String;
        fn rs_lib_quote() -> QuoteResult;
    }
}
```
---
# Functions exposed to Rust
```rust
extern "C" {
    fn c_lib_hello(str: *mut *mut c_char) -> i32;
}
#[cxx::bridge(namespace = "cxx_lib")]
mod fficxx {
    unsafe extern "C++" {
        include!("cxx_lib.hpp");
        fn hello() -> UniquePtr<CxxString>;
    }
}
```
---
# Now with Rust!!!
```cmake
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
```
---
# Demo time (Maybe?)
---
# Limitations
- Returning `Result<T>` is implemented with C++ exceptions. Requiring a shim (like I had) to avoid an exception.
  - Though they plan on supporting Result better in the future
- cxxbridge required that I return std::strings wrapped in unique_ptrs.
- cxxbridge does not officially support async (though there are workarounds)
