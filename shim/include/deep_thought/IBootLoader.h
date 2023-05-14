#pragma once

namespace magrathea {

class IBootLoader {
public:
  virtual ~IBootLoader() = default;
  virtual bool run() const = 0;
};

} // namespace magrathea