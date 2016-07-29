.PHONY: db
db:
        mkdir -p data
        docker run -p 127.0.0.1:27017:27017 -v data:/data/db --rm --name=migrator-test-db mongo