import:
	curl \
  		-X POST 'http://127.0.0.1:7700/indexes/visits/documents' \
  		-H 'Content-Type: application/json' \
  		--data-binary @visits_safari.json

addindex:
	curl \
  		-X PUT 'http://localhost:7700/indexes/visits/settings/searchable-attributes' \
  		-H 'Content-Type: application/json' \
  		--data-binary '["title", "url"]'

	curl \
  		-X PUT 'http://localhost:7700/indexes/visits/settings/sortable-attributes' \
  		-H 'Content-Type: application/json' \
  		--data-binary '["visit_time"]'

tasks:
	curl 'http://localhost:7700/tasks' | jq

ui:
	cd frontend && npm run dev

start:
	meilisearch --http-payload-size-limit=1GB
