# Makefile to build pcre binding library
#
# should be replaced by a proper rustpkg build later...
#

libsource = ./src/pcre/lib.rs
binsource = ./src/work.rs
pcre_sourcedir = ./src/pcre/C
pcre_lib = libpcre.a

# compiler switches
rustc = rustc
outdirflag = --out-dir .
clibdir = ./src/pcre/C
rclibflags = --lib -O -L$(clibdir)
rcbinflags = -O -L.

# include Makefile for pcre lib
#include ./src/pcre/C/Makefile

# define phony targets
.PHONY: all lib clean

# default target
all: lib exe

# lib target
lib: $(pcre_lib)
	$(rustc) $(rclibflags) $(outdirflag) $(libsource)

$(pcre_lib):
	$(MAKE) -C $(pcre_sourcedir) $@

# bin target
exe:
	$(rustc) $(rcbinflags) $(outdirflag) $(binsource)


clean:
	$(MAKE) -C $(pcre_sourcedir) clean
	$(RM) *.dll
	$(RM) *.exe


