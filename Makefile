.PHONY: all install clean uninstall

all:
	cargo build --release

install: all
	ln -s target/release/todo.rs /usr/bin/todo.rs

uninstall:
	rm /usr/bin/todo-rs

clean:
	rm -rf target
