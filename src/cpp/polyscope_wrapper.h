#pragma once

#include <array>

extern "C" {
void c_init();
void c_register_callback(void *function);

// ---------- Point Cloud ----------
void *c_register_float_point_cloud(const char *name,
                                   const std::array<float, 3> *pts,
                                   const int len);
void *c_register_double_point_cloud(const char *name,
                                    const std::array<double, 3> *pts,
                                    const int len);

void c_add_float_point_scalar_quantity(void *ps_point, const char *name,
                                       const float *values, const int len,
                                       const bool enabled = true);
void c_add_double_point_scalar_quantity(void *ps_point, const char *name,
                                        const double *values, const int len,
                                        const bool enabled = true);

void *c_add_float_point_color_quantity(void *ps_point, const char *name,
                                       const std::array<float, 3> *colors,
                                       const int len,
                                       const bool enabled = true);
void *c_add_double_point_color_quantity(void *ps_point, const char *name,
                                        const std::array<double, 3> *colors,
                                        const int len,
                                        const bool enabled = true);

void *c_add_float_point_vector_quantity(void *ps_point, const char *name,
                                        const std::array<float, 3> *vecs,
                                        const int len,
                                        const bool enabled = true);
void *c_add_double_point_vector_quantity(void *ps_point, const char *name,
                                         const std::array<double, 3> *vecs,
                                         const int len,
                                         const bool enabled = true);

void c_show();

bool c_generate_imgui_button(const char *name);
void c_generate_imgui_slider_int(const char *name, int *val, const int min,
                                 const int max);
}
