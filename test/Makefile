.PHONY: db
db:
	mkdir -p data
	docker run \
		-p 5432:5432 \
		-v data:/var/lib/postgresql/data \
		--rm \
		--name=migrator-test-db \
		-e POSTGRES_USER=test_user \
		-e POSTGRES_PASSWORD=test_password \
	       	-e POSTGRES_DB=test_db \
		postgres
