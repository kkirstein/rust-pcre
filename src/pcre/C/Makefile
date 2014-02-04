# Makefile
# build static lib of pcre regex library
#

pcre_sources =  \
pcre_byte_order.c  pcre_chartables.c   pcre_compile.c	    pcre_config.c \
pcre_dfa_exec.c    pcre_exec.c	       pcre_fullinfo.c	    pcre_get.c \
pcre_globals.c	   pcre_jit_compile.c  pcre_maketables.c    pcre_newline.c \
pcre_ord2utf8.c    pcre_refcount.c     pcre_string_utils.c  pcre_study.c \
pcre_tables.c	   pcre_ucd.c	       pcre_valid_utf8.c    pcre_version.c \
pcre_xclass.c

pcre_objs = $(pcre_sources:.c=.o)

pcre_lib = libpcre.a

# compiler defs
pcre_cc = gcc
pcre_cflags = -DHAVE_CONFIG_H -Wall -O2
pcre_incflags = -I.

# ar flags
pcre_ar = ar
pcre_arflags = rcus

# default target
all: $(pcre_lib)


# file targets
%.o: %.c
	$(pcre_cc) -c $< $(pcre_cflags) $(pcre_incflags) -o $@ 

$(pcre_lib): $(pcre_objs)
	$(pcre_ar) $(pcre_arflags) $@ $(pcre_objs)


# clean target
clean:
	$(RM) *.o

realclean: clean
	$(RM) $(pcre_lib)

