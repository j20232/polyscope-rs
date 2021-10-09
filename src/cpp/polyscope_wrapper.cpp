#include "polyscope_wrapper.h"

#include <functional>
#include <iostream>

#include "imgui.h"
#include "polyscope/point_cloud.h"
#include "polyscope/polyscope.h"

extern "C" {
void c_init() {
    polyscope::init();
}

void c_register_callback(void *function) {
    polyscope::state::userCallback = reinterpret_cast<void (*)()>(function);
}

void *c_register_float_point_cloud(const char *name, const float *pts,
                                   const int len) {
    std::vector<float> vec;
    vec.assign(pts, pts + len * 3);
    std::vector<std::array<float, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<float, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                  vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = polyscope::registerPointCloud(name, out);
    auto ret = reinterpret_cast<void *>(ps);
    return ret;
}

void *c_register_double_point_cloud(const char *name, const double *pts,
                                    const int len) {
    std::vector<double> vec;
    vec.assign(pts, pts + len * 3);
    std::vector<std::array<double, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<double, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                   vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = polyscope::registerPointCloud(name, out);
    auto ret = reinterpret_cast<void *>(ps);
    return ret;
}

void c_add_float_point_scalar_quantity(void *ps_point, const char *name,
                                       const float *values, const int len,
                                       const bool enabled) {
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    std::vector<float> out;
    out.assign(values, values + len);
    ps->addScalarQuantity(name, out)->setEnabled(enabled);
}

void c_add_double_point_scalar_quantity(void *ps_point, const char *name,
                                        const double *values, const int len,
                                        const bool enabled) {
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    std::vector<double> out;
    out.assign(values, values + len);
    ps->addScalarQuantity(name, out)->setEnabled(enabled);
}

void *c_add_float_point_color_quantity(void *ps_point, const char *name,
                                       const float *colors, const int len,
                                       const bool enabled) {
    std::vector<float> vec;
    vec.assign(colors, colors + len * 3);
    std::vector<std::array<float, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<float, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                  vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    ps->addColorQuantity(name, out)->setEnabled(enabled);
}

void *c_add_double_point_color_quantity(void *ps_point, const char *name,
                                        const double *colors, const int len,
                                        const bool enabled) {
    std::vector<double> vec;
    vec.assign(colors, colors + len * 3);
    std::vector<std::array<double, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<double, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                   vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    ps->addColorQuantity(name, out)->setEnabled(enabled);
}

void *c_add_float_point_vector_quantity(void *ps_point, const char *name,
                                        const float *vecs, const int len,
                                        const bool enabled) {
    std::vector<float> vec;
    vec.assign(vecs, vecs + len * 3);
    std::vector<std::array<float, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<float, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                  vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    ps->addVectorQuantity(name, out);
}

void *c_add_double_point_vector_quantity(void *ps_point, const char *name,
                                         const double *vecs, const int len,
                                         const bool enabled) {
    std::vector<double> vec;
    vec.assign(vecs, vecs + len * 3);
    std::vector<std::array<double, 3>> out;
    for (uint32_t i = 0; i < len; i++) {
        std::array<double, 3> p = {vec[i * 3 + 0], vec[i * 3 + 1],
                                   vec[i * 3 + 2]};
        out.emplace_back(p);
    }
    auto ps = static_cast<polyscope::PointCloud *>(ps_point);
    ps->addVectorQuantity(name, out);
}

void c_show() {
    polyscope::show();
}

bool c_generate_imgui_button(const char *name) {
    return ImGui::Button(name);
}

void c_generate_imgui_slider_int(const char *name, int *val, const int min,
                                 const int max) {
    ImGui::SliderInt(name, val, min, max);
}
}
