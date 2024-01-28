# Default target
all: docs

docs:
	typst c project-docs/docs.typ

.PHONY: docs
