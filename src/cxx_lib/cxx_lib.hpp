#ifndef CXX_LIB_HPP
#define CXX_LIB_HPP

#include <string>
#include <memory>

namespace cxx_lib {

// returning a std::unique_ptr because rust cxxbridge does not support c++ string's yet
std::unique_ptr<std::string> hello();

}

#endif // CXX_LIB_HPP
