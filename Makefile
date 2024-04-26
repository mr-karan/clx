BUILD_DIR := bin

all: clean build run

clean:
	rm -rf $(BUILD_DIR)

build:
	go build -o $(BUILD_DIR)/clx main.go

run:
	$(BUILD_DIR)/clx
