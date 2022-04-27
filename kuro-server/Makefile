L = lib
B = build
S = src

CC = g++
CFLAGS = -fcoroutines -std=c++20
CFLAGS += -Wall -O2 -D_GNU_SOURCE
CFLAGS += -I./include/
LDFLAGS = -L./$L/ -luring -lkuro

FORMAT = clang-format
FFLAGS = --style=google -i

SRC = $(shell echo $S/*.cc)
DEST = $B/kuro-server

$(DEST): $(SRC)
	@if [ ! -d $B ]; then mkdir $B; fi
	$(CC) $(CFLAGS) $(SRC) -o $(DEST) $(LDFLAGS)

run: $(DEST)
	@export LD_LIBRARY_PATH=./$L/ && ./$(DEST)

format:
	$(FORMAT) $(FFLAGS) $S/*

clean:
	@rm -rf $B
