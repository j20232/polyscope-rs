cmake_minimum_required(VERSION 3.10)
project("rust_polyscope" LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/libs)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/libs)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Set default build type
if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

# Warning options
if ("${CMAKE_CXX_COMPILER_ID}" MATCHES "Clang" OR "${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
    # using Clang (linux or apple) or GCC
    message("Using clang/gcc compiler flags")
    SET(BASE_CXX_FLAGS "-std=c++11 -Wall -Wextra")
    SET(DISABLED_WARNINGS " -Wno-unused-parameter -Wno-unused-variable -Wno-unused-function -Wno-deprecated-declarations -Wno-missing-braces -Wno-unused-private-field")
    SET(TRACE_INCLUDES " -H -Wno-error=unused-command-line-argument")

    if ("${CMAKE_CXX_COMPILER_ID}" MATCHES "Clang")
      message("Setting clang-specific options")
      SET(BASE_CXX_FLAGS "${BASE_CXX_FLAGS} -ferror-limit=3 -fcolor-diagnostics")
      set(warning_options "-Wall -Wextra -Wconversion")
      SET(CMAKE_CXX_FLAGS_DEBUG          "-g3 -fsanitize=address -fno-limit-debug-info")
    elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
      message("Setting gcc-specific options")
      SET(BASE_CXX_FLAGS "${BASE_CXX_FLAGS} -fmax-errors=5")
      SET(CMAKE_CXX_FLAGS_DEBUG          "-g3")
      SET(DISABLED_WARNINGS "${DISABLED_WARNINGS} -Wno-maybe-uninitialized -Wno-format-zero-length -Wno-unused-but-set-parameter -Wno-unused-but-set-variable")
    endif()
  
    SET(CMAKE_CXX_FLAGS "${BASE_CXX_FLAGS} ${DISABLED_WARNINGS}")
    SET(CMAKE_CXX_FLAGS_RELEASE        "-O3 -march=native -DNDEBUG")
elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
    set(BASE_CXX_FLAGS "${BASE_CXX_FLAGS} /W4")
    set(BASE_CXX_FLAGS "${BASE_CXX_FLAGS} /MP") # parallel build
    SET(DISABLED_WARNINGS "${DISABLED_WARNINGS} /wd\"4267\"")  # ignore conversion to smaller type (fires more aggressively than the gcc version, which is annoying)
    SET(DISABLED_WARNINGS "${DISABLED_WARNINGS} /wd\"4244\"")  # ignore conversion to smaller type (fires more aggressively than the gcc version, which is annoying)
    SET(DISABLED_WARNINGS "${DISABLED_WARNINGS} /wd\"4305\"")  # ignore truncation on initialization
    SET(CMAKE_CXX_FLAGS "${BASE_CXX_FLAGS} ${DISABLED_WARNINGS}")
    set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} /MD")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /MD")
    add_definitions(/D "_CRT_SECURE_NO_WARNINGS")
    add_definitions (-DNOMINMAX)  # don't use weird windows built-in min/max
    add_definitions (-D_USE_MATH_DEFINES)  # match unix behavior of constants in cmath
else()
    message(WARNING "Unsupported compiler for warning options. CMAKE_CXX_COMPILER_ID: ${CMAKE_CXX_COMPILER_ID}")
endif()

set(TARGET rust_polyscope)

# polyscope
add_subdirectory(third_party/polyscope)
target_include_directories(polyscope INTERFACE $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/third_party/polyscope/include>)

add_library(${TARGET} STATIC
    polyscope_wrapper.cpp
)

target_link_libraries(${TARGET} PRIVATE
    polyscope
)

target_include_directories(${TARGET} PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}
)

install (TARGETS ${TARGET} DESTINATION .)
install (TARGETS polyscope DESTINATION .)
install (TARGETS glad DESTINATION .)
install (TARGETS glfw DESTINATION .)
install (TARGETS imgui DESTINATION .)
install (TARGETS stb DESTINATION .)
