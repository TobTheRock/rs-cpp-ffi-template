
#include "deep_thought/deep_thought.h"

// Corrosion renames the generated header file
#ifdef WITH_CORROSION
#include "deep_thought_ffi/bindings.h"
#else
#include "rs-cpp-ffi-template/src/bindings.rs.h"
#endif

namespace magrathea {

// pimpl, just containing the actual rust implementation
// this is needed when we want to hide the generated rust::Box type
// Beeing opaque, we can't simply put ffi::RsDeepThought in a unique_ptr
struct DeepThoughtImpl {
  DeepThoughtImpl(const std::shared_ptr<IBootLoader>& boot_loader)
      : 
      config({.clock_rate = 400, .version = "1.0.1"}),
      deep_thought(ffi::create_deep_thought(boot_loader, config)) {}

  ::ffi::Config config;
  ::rust::Box<ffi::RsDeepThought> deep_thought;
};

DeepThought::DeepThought(const std::shared_ptr<IBootLoader> &boot_loader)
    : _impl(std::make_unique<DeepThoughtImpl>(boot_loader)) {}
DeepThought::~DeepThought() {}

std::string DeepThought::ask(const std::string &question) const {
  // the generated bindings support some implicit conversion to rusty types
  ::rust::Str rs_str = question;

  ::rust::String rs_answer = _impl->deep_thought->ask(rs_str);

  // due to the opaque type we need to copy
  std::string answer = std::string{rs_answer.begin(), rs_answer.end()};

  return answer;
}

} // namespace magrathea