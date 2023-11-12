TARGETS=\
	orderless_example orderless_example.test

all: $(TARGETS)

clean:
	rm -vf $(TARGETS)

%_example: %_example.rs lib%.so
	rustc -L . $<

%_example.test: %_example.rs lib%.so
	rustc -L . $< --test -o $@

lib%.so: %.rs
	rustc --crate-type proc-macro $<

.PHONY: all clean
