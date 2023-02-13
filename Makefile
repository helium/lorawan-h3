DESTDIR ?= serialized
INDEX_SRCDIR ?= extern/hplans
PARAMS_SRCDIR ?= region_params
RESOLUTION ?= 7
REGIONS ?= \
  AS923-1 \
  AS923-1B \
  AS923-1C \
  AS923-2 \
  AS923-3 \
  AS923-4 \
  AU915 \
  AU915-SB1 \
  CD900-1A \
  CN470 \
  EU433 \
  EU868 \
  IN865 \
  KR920 \
  RU864 \
  US915

INDEX_TARGETS = $(patsubst %,$(DESTDIR)/%.res$(RESOLUTION).h3idz, $(REGIONS)) 
PARAMS_TARGETS = $(patsubst %,$(DESTDIR)/%.rpz, $(REGIONS))
INDEX_SOURCES = $(patsubst %,$(INDEX_SRCDIR)/%.geojson, $(REGIONS))
PARAMS_SOURCES = $(patsubst %,$(PARAMS_SRCDIR)/%.json, $(REGIONS))

$(DESTDIR)/%.res$(RESOLUTION).h3idz: $(SRCDIR)/%.geojson
	./target/release/lw-generator index generate $< $@ --resolution $(RESOLUTION)

$(DESTDIR)/%.rpz: $(SRCDIR)/%.json
	./target/release/lw-generator params generate $< $@ 

all: compile index params

$(INDEX_TARGETS): | $(INDEX_SOURCES) $(DESTDIR) 

$(INDEX_SOURCES): | $(INDEX_SRCDIR)

$(PARAMS_TARGETS): | $(PARAMS_SOURCES) $(DESTDIR) 

$(PARAMS_SOURCES): | $(PARAMS_SRCDIR)

$(DESTDIR):
	mkdir -p $(DESTDIR)

extern/hplans:
	git submodule update --init

compile:
	cargo build --release

index: $(INDEX_TARGETS)

params: $(PARAMS_TARGETS) 
