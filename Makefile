SHELL=/bin/bash

dev-front:
	@cargo watch -x 'run front'

tw-dev:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css --watch

tw-prod:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css --minify
