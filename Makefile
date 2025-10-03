.PHONY: clean

# https://stackoverflow.com/questions/714100/os-detecting-makefile
ifeq ($(OS),Windows_NT)
    ARGS = ".\raylib\raylib-5.5_win64_msvc16\lib\raylib.lib Gdi32.lib WinMM.lib shell32.lib User32.lib"
    OUT = main.exe
else
    ARGS = "-L./raylib/raylib-5.5_linux_amd64/lib -l:libraylib.a -lc -lm"
    OUT = main
endif

$(OUT): main.rs
	/home/anon/dev/github/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc --edition 2021 -g -C opt-level=z -C link-args=$(ARGS) -C panic="abort" main.rs -o $(OUT)

clean:
	rm -f main
	rm -f main.exe
	rm -f *.o
	rm -f *.pdb
