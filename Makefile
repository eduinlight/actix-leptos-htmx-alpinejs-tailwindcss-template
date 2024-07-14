SHELL=/bin/bash

# FRONT
front-dev:
	@cargo watch -s 'make js-dev && make css-dev && cargo run -p front' \
		--ignore apps/front/static/styles.css \
		--ignore apps/front/static/main.js

js-dev:
	@cd apps/front && \
	bun build input.js --outdir ./static --entry-naming main.js

js-dev:
	@cd apps/front && \
	bun build input.js --outdir ./static --entry-naming main.js --minify

css-dev:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css

css-prod:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i input.css -o ./static/styles.css --minify
