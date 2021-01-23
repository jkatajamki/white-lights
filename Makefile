all: up

launch:
	./set-up-db.sh

stop:
	docker-compose stop

sleep:
	sleep 15

clean: stop
	docker-compose rm -f

logs:
	docker-compose logs -f

up: stop launch logs

scratch: clean launch sleep logs
