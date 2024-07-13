CONFIG_PRESET := "ninja-multi-clang"
BUILD_DIR := "build/ninja-multi-clang"
BUILD_PRESET := "debug"
PATH_TO_LOGS := "$(CURDIR)/cxx_app.log"

export CXX_APP_LOGS = $(PATH_TO_LOGS)

.PHONY: all configure build clean slides

all: build

configure:
	cmake --preset $(CONFIG_PRESET)

build: configure
	cmake --build $(BUILD_DIR) --preset $(BUILD_PRESET) --target all

run: build
	@echo -e 
	cmake --build $(BUILD_DIR) --preset $(BUILD_PRESET) --target run_cxx_app

clean:
	@rm -rf $(BUILD_DIR)

slides:
	@cd slides && marp slides.md && xdg-open slides.html


