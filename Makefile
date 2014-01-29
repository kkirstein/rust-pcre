# Makefile to build pcre binding library
#
# should be replaced by a proper rustpkg build later...
#

libsource = ./src/pcre/lib.rs
binsource = ./src/work.rs

# compiler switches
rustc = rustc
outdirflag = --out-dir .
clibdir = ./src/pcre/C
rclibflags = --lib -O -L$(clibdir)
rcbinflags = -O -L.

# utilities

# define phony targets
.PHONY: all lib clean

# lib target
lib:
	$(rustc) $(rclibflags) $(outdirflag) $(libsource)

# bin target
exe:
	$(rustc) $(rcbinflags) $(outdirflag) $(binsource)

all: lib exe

clean:
	$(RM) *.dll
	$(RM) *.exe


