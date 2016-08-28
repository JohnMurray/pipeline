default: release

setup:
	npm install -g doctoc
	cargo install rusty-tags
	# NOTE: This requires that our .zshrc file contain the the following:
	# export RUST_SRC_PATH=$HOME/projects/rust-src/
	git clone https://github.com/rust-lang/rust.git ~/projects/rust-src
	cd ~/projects/rust-src && git checkout stable

release:
	doctoc readme.md --github --notitle --maxlevel 3
