cmake_minimum_required(VERSION 3.0)

project("filcoin-webapi")

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

enable_language(Rust)
include(CMakeCargo)

cargo_build(NAME filecoin-webapi)

