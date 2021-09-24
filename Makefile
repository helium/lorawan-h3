DESTDIR ?= serialized
SRCDIR ?= extern/hplans
RESOLUTION ?= 7
REGIONS ?= \
  AS923-3  \
  AS923-4  \
  KR920    \
  AS923-2  \
  RU864    \
  CN470    \
  IN865    \
  US915    \
  EU433    \
  AS923-1  \
  Unknown  \
  AU915    \
  EU868

TARGETS = $(patsubst %,$(DESTDIR)/%.res$(RESOLUTION).h3idx, $(REGIONS))
SOURCES = $(patsubst %,$(SRCDIR)/%.GEOJSON, $(REGIONS))

$(DESTDIR)/%.res$(RESOLUTION).h3idx: $(SRCDIR)/%.geojson
	erl -pa _build/default/lib/*/ebin -noshell -eval "genh3:to_serialized_h3(\"$<\", \"$@\", $(RESOLUTION)), erlang:halt()"

all: $(TARGETS)

$(TARGETS): | $(SOURCES) $(DESTDIR)

$(SOURCES): | $(SRCDIR)

$(DESTDIR):
	mkdir $(DESTDIR)

extern/hplans:
	git submodule update --init

compile:
	rebar3 compile
