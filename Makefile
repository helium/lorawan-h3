DESTDIR ?= tmp
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

TARGETS = $(patsubst %,$(DESTDIR)/%.res$(RESOLUTION).h3idx, $(REGIONS))
SOURCES = $(patsubst %,$(SRCDIR)/%.GEOJSON, $(REGIONS))

$(DESTDIR)/%.res$(RESOLUTION).h3idx: $(SRCDIR)/%.geojson
	./target/release/lw-generator generate $< $@

all: $(TARGETS)

$(TARGETS): | $(SOURCES) $(DESTDIR)

$(SOURCES): | $(SRCDIR)

$(DESTDIR):
	mkdir $(DESTDIR)

extern/hplans:
	git submodule update --init

compile:
	rebar3 compile
