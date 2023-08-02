.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: abi
abi:
	eosc -u https://eos.greymass.com get abi eosio.evm | antelope-abi2rs src/abi.rs

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google,sf/antelope

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run -e eos.firehose.eosnation.io:9001 map_actions -s 320658202 -t +518400 -o jsonl --production-mode

.PHONY: gui
gui:
	substreams gui -e eos.firehose.eosnation.io:9001 map_actions -s 320658202 -t +518400 --production-mode
