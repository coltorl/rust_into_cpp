#include "cxx_lib.hpp"

namespace {
constexpr std::string_view RET = "Hello From C++!";

}

std::unique_ptr<std::string> cxx_lib::hello() { return std::make_unique<std::string>(RET); }
