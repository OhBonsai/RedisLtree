default: help

builder: ## build rust cross-compiler base image, for mac developers
	docker build -t retree-builder -f builder.Dockerfile .

build: ## build fdauth docker image
	docker build -t retree -f Dockerfile .

tester: build ## build tester image
	docker build -t retree-tester -f tester.Dockerfile .

test: tester ## do some behavioral tester
	docker run -v ${PWD}/tests:/tests --name treetest  --rm retree-tester 

clean: ## clean
	rm -rf target
	rm -rf tests/.pytest_cache

## Help documentatin à la https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' ./Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'