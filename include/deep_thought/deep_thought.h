/*
 * The DeepThougth impl using C++ standard types
 *
 * Uses private implementation (pimpl) approach to hide generated cxx types
 * This is just to encapsulates the RUST library, but not strictly necessary,
 * to use RUST with C++
 */

#pragma once

#include <memory>
#include <string>

namespace rs_ffi {
struct DeepThought;
}

namespace magrathea {

class DeepThought {
public:
  DeepThought();
  // a default destructor together with a unique_ptr with a forward declared type is not possible
  ~DeepThought();

  std::string ask(const std::string &question) const;

private:
  // hidden, forward declared impl
  std::unique_ptr<struct DeepThoughtImpl> _impl;
};

} // namespace magrathea