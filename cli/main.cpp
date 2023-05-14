#include "deep_thought/IBootLoader.h"
#include "deep_thought/deep_thought.h"
#include <cstdint>
#include <iostream>
#include <memory>
#include <ostream>
#include <string>

class TowelBootloader: public magrathea::IBootLoader {
  public:
    bool run() const override {
      std::cout << "CPP: Gathering towels..." << std::endl;

      return true;
    }
};

int main() {
  std::cout << "CPP: Ask DeepThought™ your question and confirm with ENTER"
            << std::endl;

  std::string question;
  std::getline(std::cin, question);

  const auto boot_loader = std::make_shared<TowelBootloader>();

  // the actual call into the Rust library
  const auto deep_thought = magrathea::DeepThought(boot_loader);
  const auto answer = deep_thought.ask(question);

  std::cout << "CPP: DeepThought™ answered: " << answer << std::endl;
}
