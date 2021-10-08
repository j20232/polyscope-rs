#pragma once

#include <array>

extern "C" {
void c_init();
void *c_register_point_cloud(const char *name, const std::array<float, 3> *pts,
                             const int len);
void c_add_point_scalar_quantity(void *ps_point, const char *name,
                                 const float *values, const int len,
                                 const bool enabled = true);
void c_show();
}
