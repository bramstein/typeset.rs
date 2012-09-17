TYPESET_SRC = $(shell find src -type f -name '*.rs')
TYPESET_DEPS = src/typeset.rc $(TYPESET_SRC)

typeset: $(TYPESET_DEPS)
	rustc -o $@ $<

.PHONE: clean
clean:
	rm -f typeset
