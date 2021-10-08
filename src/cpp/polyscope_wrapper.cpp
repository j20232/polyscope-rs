#include "polyscope_wrapper.h"

#include <iostream>

#include "polyscope/point_cloud.h"
#include "polyscope/polyscope.h"

extern "C" {
void c_init() {
    polyscope::init();
}

void *c_register_point_cloud(const char *name, const std::array<float, 3> *pts,
                             const int len) {
    std::vector<std::array<float, 3>> out;
    out.assign(pts, pts + len);
    auto ps = polyscope::registerPointCloud(name, out);
    auto ret = reinterpret_cast<void *>(ps);
    return ret;
}

void c_add_point_scalar_quantity(void *ps_point, const char *name,
                                 const float *values, const int len,
                                 const bool enabled) {
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    std::vector<float> out;
    out.assign(values, values + len);
    ps->addScalarQuantity(name, out)->setEnabled(enabled);
}

void c_show() {
    polyscope::show();
}
}
