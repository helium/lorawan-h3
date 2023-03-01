DESTDIR ?= serialized
INDEX_SRCDIR ?= extern/hplans
PARAMS_SRCDIR ?= region_params
RESOLUTION ?= 7
REGIONS ?= $(shell cat regions.txt)

INDEX_TARGETS = $(patsubst %,$(DESTDIR)/%.res$(RESOLUTION).h3idz, $(REGIONS)) 
INDEX_SOURCES = $(patsubst %,$(INDEX_SRCDIR)/%.geojson, $(REGIONS))
PARAMS_TARGETS = $(patsubst %,$(DESTDIR)/%.rpz, $(REGIONS))
PARAMS_SOURCES = $(patsubst %,$(PARAMS_SRCDIR)/%.json, $(REGIONS))

$(DESTDIR)/%.res$(RESOLUTION).h3idz: $(INDEX_SRCDIR)/%.geojson
	./target/release/lw-generator index generate $< $@ --resolution $(RESOLUTION)

$(DESTDIR)/%.rpz: $(PARAMS_SRCDIR)/%.json
	@[ -f '$<' ] && (echo 'Processing $<' && ./target/release/lw-generator params generate $< $@) || echo 'Missing $<'
	
all: compile params

$(INDEX_TARGETS): | $(INDEX_SOURCES) $(DESTDIR) 

$(INDEX_SOURCES): | $(INDEX_SRCDIR) regions.txt

$(PARAMS_TARGETS): | $(PARAMS_SOURCES) $(DESTDIR) 

$(PARAMS_SOURCES): | $(PARAMS_SRCDIR) regions.txt

$(DESTDIR):
	mkdir -p $(DESTDIR)

extern/hplans:
	git submodule update --init

compile:
	cargo build --release

index: $(INDEX_TARGETS)

params: $(PARAMS_TARGETS)
