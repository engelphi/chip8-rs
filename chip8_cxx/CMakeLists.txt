if (APPLE)
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
		set(CARGO_CMD cargo post build --target aarch64-apple-darwin --target x86_64-apple-darwin --package chip8_cxx --verbose)
		set(TARGET_DIR "universal-apple-darwin/debug")
	else ()
		set(CARGO_CMD cargo post build --target aarch64-apple-darwin --target x86_64-apple-darwin --package chip8_cxx --release --verbose)
		set(TARGET_DIR "universal-apple-darwin/release")
	endif ()
elseif (WIN32)
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
		set(CARGO_CMD cargo build --target x86_64-pc-windows-msvc --verbose)
		set(TARGET_DIR "x86_64-pc-windows-msvc/debug")
	else ()
		set(CARGO_CMD cargo build --target x86_64-pc-windows-msvc --release --verbose)
		set(TARGET_DIR "x86_64-pc-windows-msvc/release")
	endif ()
elseif (LINUX)
	if (CMAKE_BUILD_TYPE STREQUAL "Debug")
		set(CARGO_CMD cargo build --verbose)
		set(TARGET_DIR "debug")
	else ()
		set(CARGO_CMD cargo build --release --verbose)
		set(TARGET_DIR "release")
	endif ()
else ()
	message(FATAL_ERROR "Unsupported platform")
endif()


if (APPLE)
	set(CHIP8_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libchip8_cxx.a")
elseif (WIN32)
	set(CHIP8_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/chip8_cxx.lib")
else()
	set(CHIP8_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libchip8_cxx.a")
endif()

set (CHIP8_CXX "${CMAKE_CURRENT_BINARY_DIR}/chip8_cxx.cpp")

if (APPLE)
	add_custom_command(
		OUTPUT ${CHIP8_LIB} ${CHIP8_CXX}
		COMMAND cargo install cargo-post
		COMMAND MACOSX_DEPLOYMENT_TARGET=${CMAKE_OSX_DEPLOYMENT_TARGET} CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/chip8_cxx/src/lib.rs.cc ${CHIP8_CXX}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/chip8_cxx/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/chip8_cxx.h
		COMMAND cp -r ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust ${CMAKE_CURRENT_BINARY_DIR}/
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	)
else()
	add_custom_command(
		OUTPUT ${CHIP8_LIB} ${CHIP8_CXX}
		COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} env RUSTFLAGS="${RUST_FLAGS}" ${CARGO_CMD}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/chip8_cxx/src/lib.rs.cc ${CHIP8_CXX}
		COMMAND cp ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/chip8_cxx/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/chip8_cxx.h
		COMMAND cp -r ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust ${CMAKE_CURRENT_BINARY_DIR}/
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	)
endif()

add_library(chip8_cxx STATIC ${CHIP8_CXX})
if (APPLE)
	target_link_libraries(chip8_cxx PRIVATE pthread dl ${CHIP8_LIB} PUBLIC "-framework SystemConfiguration")
elseif (WIN32)
	target_link_libraries(chip8_cxx PRIVATE ${CHIP8_LIB} userenv.lib)
else()
	target_link_libraries(chip8_cxx PRIVATE pthread dl ${CHIP8_LIB})
endif()

target_include_directories(chip8_cxx PUBLIC ${CMAKE_CURRENT_BINARY_DIR})
