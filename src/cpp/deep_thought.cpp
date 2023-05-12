#include "rs-cpp-ffi-template/include/deep_thought/deep_thought.h"
#include "rs-cpp-ffi-template/src/bindings.rs.h"

namespace magrathea {

// pimpl, just containing the actual rust implementation
// this is needed when we want to hide the generated rust::Box type
// Beeing opaque, we can't simply put ffi::RsDeepThought in a unique_ptr
struct DeepThoughtImpl {
  DeepThoughtImpl() : deep_thought(ffi::create_deep_thought()) {}

  ::rust::Box<ffi::RsDeepThought> deep_thought;
};

DeepThought::DeepThought() : _impl(std::make_unique<DeepThoughtImpl>()) {}

std::string DeepThought::ask(const std::string &question) const {
  // the generated bindings support some implicit conversion to rusty types
  ::rust::Str rs_str = question;

  ::rust::String rs_answer = _impl->deep_thought->ask(rs_str);

  std::string answer = std::string{rs_answer.begin(), rs_answer.end()};

  return answer;
}

} // namespace magrathea