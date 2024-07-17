SHELL=/bin/bash

# FRONT
front-dev:
	@cargo watch -s './scripts/start-front-dev.sh' \
		--ignore apps/front/static/styles.css \
		--ignore apps/front/static/main.js

js-dev:
	@cd apps/front && \
	bun build index.js --outdir ./static --entry-naming main.js --target browser

js-prod:
	@cd apps/front && \
	bun build index.js --outdir ./static --entry-naming main.js --target browser --minify

css-dev:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i styles.css -o ./static/styles.css

css-prod:
	@cd apps/front && \
	tailwindcss -c tailwind.config.js -i styles.css -o ./static/styles.css --minify
