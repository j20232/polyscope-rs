#include "polyscope_wrapper.h"

#include "polyscope/polyscope.h"

extern "C" {
void init() {
    polyscope::init();
}
void show() {
    polyscope::show();
}
}
