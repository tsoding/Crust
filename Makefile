# Variables
RAYLIB_PATH_LINUX = ./raylib-5.5_linux_amd64/lib/
RAYLIB_PATH_MACOS = ./raylib-5.5_macos/lib/
RAYLIB_LIB = libraylib.a
CFLAGS = -lm
RUSTFLAGS = --edition 2021 -g -C opt-level=z --emit=obj -C panic="abort"
SRC = main.rs
OBJ = main.o

# Platform-specific settings
ifeq ($(shell uname), Darwin) # macOS
    PLATFORM = macos
    EXTRA_FLAGS = -framework CoreVideo -framework IOKit -framework Cocoa -framework GLUT -framework OpenGL
	LINK_FLAGS = -L$(RAYLIB_PATH) $(RAYLIB_PATH)$(RAYLIB_LIB)
    RAYLIB_PATH = $(RAYLIB_PATH_MACOS)
else # Linux
    PLATFORM = linux
    EXTRA_FLAGS =
	LINK_FLAGS = -L$(RAYLIB_PATH) -l:$(RAYLIB_LIB)
    RAYLIB_PATH = $(RAYLIB_PATH_LINUX)
endif

# Targets
main: $(OBJ)
	gcc -o $@ $(OBJ) $(CFLAGS) $(EXTRA_FLAGS) $(LINK_FLAGS)

$(OBJ): $(SRC)
	rustc $(RUSTFLAGS) $<

# Phony targets
.PHONY: clean
clean:
	rm -f $(OBJ) main
