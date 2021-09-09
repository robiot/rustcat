#Makefile to create release files
RELEASE_PATH = "pkg-env/release"
RELEASE_VERSION := ${shell cargo pkgid | cut -d# -f2 | cut -d: -f2}
BIN_NAME = "rc"

.PHONY: install
install:
	cargo install cross
	rustup target add x86_64-unknown-linux-musl
	rustup target add x86_64-pc-windows-gnu

.PHONY: clean
clean:
	rm -rf ${RELEASE_PATH}/*

.PHONY: all
all:
	make clean
	make linux
	make deb
	make win
	

.PHONY: linux
linux:
	@echo "Building for Linux..."
	cross build --release --target=x86_64-unknown-linux-musl
	mkdir -p ${RELEASE_PATH} && tar -C target/x86_64-unknown-linux-musl/release -czf ${RELEASE_PATH}/rustcat_${RELEASE_VERSION}_amd64-linux.tar.gz ${BIN_NAME}

.PHONY: deb
deb:
	@echo "Building Debian Package..."
	cargo deb --target=x86_64-unknown-linux-musl
	mkdir -p ${RELEASE_PATH} && cp -rf target/x86_64-unknown-linux-musl/debian/rustcat_* ${RELEASE_PATH}


.PHONY: win
win:
	@echo "Building for Windows..."
	cross build --release --target=x86_64-pc-windows-gnu
	mkdir -p ${RELEASE_PATH} && zip ${RELEASE_PATH}/rustcat_${RELEASE_VERSION}_win64.zip target/x86_64-pc-windows-gnu/release/${BIN_NAME}.exe