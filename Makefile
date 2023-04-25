test:
	pytest -v tests

clean:
	find . -path '*/__pycache__*' -delete
	find . -path '*/.pytest_cache*' -delete