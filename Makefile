tmp=_git_distcheck
http=rust-http
bin_path=bin
doc_path=doc
http_lib_path=$(http)/build
LINK_OPTS=-L$(http_lib_path)

all:
	rustc lib.rs -O $(LINK_OPTS) -o reddit

test:
	rustc lib.rs --test --cfg debug $(LINK_OPTS) -o reddittest

doc:
	mkdir -p $(doc_path)
	rustdoc lib.rs --output-dir $(doc_path)

distcheck:
	rm -rf $(tmp)
	git clone --recursive . $(tmp)
	make -C $(tmp) deps
	make -C $(tmp)
	rm -rf $(tmp)

deps:
	cd $(http); make;

.PHONY:doc