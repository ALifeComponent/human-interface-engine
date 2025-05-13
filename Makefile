.PHONY: init
init:
	@echo "Initializing project"
	make init-hooks

.PHONY: init-hooks
init-hooks:
	@echo "Initializing git hooks..."
	ln -s $(realpath ./scripts/pre-commit) .git/hooks/pre-commit || true
	@echo "Git hooks initialized."

