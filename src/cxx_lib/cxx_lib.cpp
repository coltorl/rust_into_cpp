#include "cxx_lib.hpp"

namespace {
constexpr std::string_view RET = "Hello From C++!";

}

std::string cxx_lib::func() { return std::string(RET); }
