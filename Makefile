test:
	pytest -v tests

clean:
	rm -rf .vscode tests/.pytest_cache __pycache__ */__pycache__