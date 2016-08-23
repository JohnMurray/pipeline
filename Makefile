default: release

setup:
	npm install -g doctoc

release:
	doctoc readme.md --github --notitle --maxlevel 3
