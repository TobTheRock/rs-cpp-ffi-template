#include "deep_thought/deep_thought.h"
#include <cstdint>
#include <iostream>
#include <string>

int main() {
  std::cout << "Ask DeepThought™ your question and confirm with ENTER"
            << std::endl;

  std::string question;
  std::getline(std::cin, question);

  // the actual call into the Rust library
  const auto deep_thought = magrathea::DeepThought {};
  const auto answer = deep_thought.ask(question);

  std::cout << "DeepThought™ answered: " << answer << std::endl;
}
