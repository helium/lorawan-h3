ERLDESTDIR ?= serialized/erlang
RSDESTDIR ?= serialized/rust
SRCDIR ?= extern/hplans
RESOLUTION ?= 7
REGIONS ?= \
  AS923-1 \
  AS923-1B \
  AS923-2 \
  AS923-3 \
  AS923-4 \
  AU915 \
  CD900-1A \
  CN470 \
  EU433 \
  EU868 \
  IN865 \
  KR920 \
  RU864 \
  US915

TARGETS = $(patsubst %,$(ERLDESTDIR)/%.res$(RESOLUTION).h3idx, $(REGIONS)) $(patsubst %,$(RSDESTDIR)/%.res$(RESOLUTION).h3idx, $(REGIONS))
SOURCES = $(patsubst %,$(SRCDIR)/%.GEOJSON, $(REGIONS))

$(ERLDESTDIR)/%.res$(RESOLUTION).h3idx: $(SRCDIR)/%.geojson
	erl -pa _build/default/lib/*/ebin -noshell -eval "genh3:to_serialized_h3(\"$<\", \"$@\", $(RESOLUTION)), erlang:halt()"

$(RSDESTDIR)/%.res$(RESOLUTION).h3idx: $(SRCDIR)/%.geojson
	./target/release/lw-generator generate $< $@

all: $(TARGETS)

$(TARGETS): | $(SOURCES) $(ERLDESTDIR) $(RSDESTDIR)

$(SOURCES): | $(SRCDIR)

$(ERLDESTDIR):
	mkdir -p $(ERLDESTDIR)

$(RSDESTDIR):
	mkdir -p $(RSDESTDIR)

extern/hplans:
	git submodule update --init

compile:
	rebar3 compile

# Hack, bad makefile hygine, remove before merge.
check: $(TARGETS)
	cmp $(ERLDESTDIR)/AS923-1.res7.h3idx  $(RSDESTDIR)/AS923-1.res7.h3idx
	cmp $(ERLDESTDIR)/AS923-1B.res7.h3idx $(RSDESTDIR)/AS923-1B.res7.h3idx
	cmp $(ERLDESTDIR)/AS923-2.res7.h3idx  $(RSDESTDIR)/AS923-2.res7.h3idx
	cmp $(ERLDESTDIR)/AS923-3.res7.h3idx  $(RSDESTDIR)/AS923-3.res7.h3idx
	cmp $(ERLDESTDIR)/AS923-4.res7.h3idx  $(RSDESTDIR)/AS923-4.res7.h3idx
	cmp $(ERLDESTDIR)/AU915.res7.h3idx    $(RSDESTDIR)/AU915.res7.h3idx
	cmp $(ERLDESTDIR)/CD900-1A.res7.h3idx $(RSDESTDIR)/CD900-1A.res7.h3idx
	cmp $(ERLDESTDIR)/CN470.res7.h3idx    $(RSDESTDIR)/CN470.res7.h3idx
	cmp $(ERLDESTDIR)/EU433.res7.h3idx    $(RSDESTDIR)/EU433.res7.h3idx
	cmp $(ERLDESTDIR)/EU868.res7.h3idx    $(RSDESTDIR)/EU868.res7.h3idx
	cmp $(ERLDESTDIR)/IN865.res7.h3idx    $(RSDESTDIR)/IN865.res7.h3idx
	cmp $(ERLDESTDIR)/KR920.res7.h3idx    $(RSDESTDIR)/KR920.res7.h3idx
	cmp $(ERLDESTDIR)/RU864.res7.h3idx    $(RSDESTDIR)/RU864.res7.h3idx
	cmp $(ERLDESTDIR)/US915.res7.h3idx    $(RSDESTDIR)/US915.res7.h3idx
