TARGET = wasm32-unknown-unknown
BUILD_DIR = target/$(TARGET)/release
OUTPUT_WASM = $(shell ls $(BUILD_DIR)/*.wasm 2>/dev/null)
INDEX_HTML = ~/.macroquad_index.html

all: build copy_wasm copy_index

build:
	cargo build --release --target $(TARGET)

copy_wasm:
	cp $(OUTPUT_WASM) game.wasm

copy_index:
	cp $(INDEX_HTML) index.html

clean:
	cargo clean
	rm -f game.wasm index.html
