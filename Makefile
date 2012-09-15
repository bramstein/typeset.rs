TYPESET_SRC = $(shell find src -type f -name '*.rs')

typeset: $(TYPESET_SRC)
	echo $(TYPESET_SRC)
	rustc -o $@ $< 

.PHONE: clean
clean:
	rm -f typeset
