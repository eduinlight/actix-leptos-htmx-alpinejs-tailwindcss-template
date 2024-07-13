SHELL=/bin/bash

front-dev:
	@cargo watch -s 'make tw-dev && cargo run -p front' --ignore apps/front/static/styles.css

tw-dev:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css

tw-prod:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css --minify
