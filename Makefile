BUILD_DIR := bin
VERSION := $(shell git describe --tags --abbrev=0)

all: clean build run

clean:
	rm -rf $(BUILD_DIR)

build:
	go build -ldflags "-X main.version=$(VERSION)" -o $(BUILD_DIR)/clx main.go

run:
	$(BUILD_DIR)/clx
